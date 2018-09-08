#[macro_use]
extern crate log;
extern crate cast;
extern crate env_logger;
extern crate libc;
extern crate regex;

use cast::{i8, u8};
use libc::{c_char, c_int};
use log::Level;
use regex::Regex;
use std::cmp::Ordering;
use std::env;
use std::ffi::{CStr, CString};
use std::fs;
use std::io;
use std::path::Path;
use std::vec::Vec;

type CCharStr = Vec<c_char>;

#[derive(Debug, Eq)]
struct PythonVersion {
    path: CString,
    major: u32,
    minor: u32,
}

impl Ord for PythonVersion {
    fn cmp(&self, other: &PythonVersion) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
    }
}

impl PartialOrd for PythonVersion {
    fn partial_cmp(&self, other: &PythonVersion) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PythonVersion {
    fn eq(&self, other: &PythonVersion) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

fn find_python(dir: &Path) -> io::Result<Vec<PythonVersion>> {
    let mut python_list: Vec<PythonVersion> = Vec::new();
    let py_exec = Regex::new(r"^(python)(\d).(\d)$")
        .expect("Failed to build regex to match python executables");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.metadata()?.is_file() {
                let filename = entry.file_name();
                if let Some(file_str) = filename.to_str() {
                    if let Some(parts) = py_exec.captures(file_str) {
                        python_list.push(PythonVersion {
                            major: parts
                                .get(2)
                                .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0)),
                            minor: parts
                                .get(3)
                                .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0)),
                            path: CString::new(entry.path().to_str().unwrap_or("/dev/null"))
                                .unwrap(),
                        });
                    }
                }
            }
        }
    }
    Ok(python_list)
}

fn into_libc_c_char(input: &CString) -> CCharStr {
    let data_slice = input.as_bytes_with_nul();
    let append_zero = *data_slice.last().unwrap() != '\0' as u8;
    let in_size = data_slice.len();
    let size = match append_zero {
        true  => in_size + 1,
        false => in_size
    };
    let mut output: CCharStr = vec![0i8; size];
    for (a, b) in output.iter_mut().zip(data_slice) {
        *a = i8(*b).unwrap_or(65);
    }
    if append_zero {
        debug!("Appending zero");
        output[in_size] = '\0' as i8;
    }
    debug!("converter: {:?}",&output);
    output
}

fn main() {
    env_logger::init();
    let search_paths = vec![Path::new("/usr/bin"), Path::new("/usr/local/bin")];
    let mut python_list: Vec<PythonVersion> = Vec::new();
    for directory in search_paths {
        match find_python(directory) {
            Ok(mut list) => python_list.append(&mut list),
            Err(error) => println!("dir error: {:?}", error),
        }
    }
    python_list.sort();

    let latest_python = &python_list[python_list.len() - 1];
    let proc = into_libc_c_char(&latest_python.path);
    let mut c_argv: Vec<CCharStr> = Vec::new();
    c_argv.push(proc);

    for argument in env::args().skip(1) {
        let ffi_str = CString::new(argument).unwrap();
        let c_string = into_libc_c_char(&ffi_str);
        c_argv.push(c_string);
    }

    let mut c_argv_ptrs: Vec<_> = c_argv.iter().map(|arg| arg.as_ptr()).collect();
    c_argv_ptrs.push(std::ptr::null()); // argv must be terminated with NULL

    if log_enabled!(Level::Info) {
        let size = c_argv[0].iter().len() - 1;
        let mut converted_back: Vec<u8> = Vec::with_capacity(size);
        for char_str in c_argv[0].iter_mut().take(size) {
            converted_back.push(u8(*char_str).unwrap());
        }
        let final_string = CString::new(converted_back).unwrap();
        info!("Executing \"{}\"", final_string.to_str().unwrap());
    }

    unsafe {
        let argv: *const *const c_char = c_argv_ptrs.as_ptr();
        let app = c_argv[0].as_ptr();
        let result: c_int = libc::execvp(app, argv);
        if result != 0 {
            let error: c_int = *libc::__errno_location();
            let err_cstr = CStr::from_ptr(libc::strerror(error));
            match err_cstr.to_str() {
                Ok(error_string) => error!("execvp failed!: {}", error_string),
                Err(_) => error!("execvp failed!: {}", error),
            }
        }
    }
}
