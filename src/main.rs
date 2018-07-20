extern crate regex;

use std::path::Path;
use std::io;
use std::fs;
use regex::Regex;
use std::vec::Vec;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct PythonVersion
{
    path: String,
    major: u32,
    minor: u32
}

impl Ord for PythonVersion {
    fn cmp(&self, other: &PythonVersion) -> Ordering {
        self.major.cmp(&other.major).then(self.minor.cmp(&other.minor))
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
    let py_exec = Regex::new(r"^(python)(\d).(\d)$").expect("Failed to build regex to match python executables");
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            if entry.metadata()?.is_file() {
                let filename = entry.file_name();
                if let Some(file_str) = filename.to_str()
                {
                    if let Some(parts) = py_exec.captures(file_str) {
                        python_list.push(PythonVersion {
                            major: parts.get(2).map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0)),
                            minor: parts.get(3).map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0)),
                            path: String::from(entry.path().to_string_lossy())
                        });
                    }
                }
            }
        }
    }
    Ok(python_list)
}

fn main() {
    let search_paths = vec![Path::new("/usr/bin"), Path::new("/usr/local/bin")];
    let mut complete_list: Vec<PythonVersion> = Vec::new();
    for directory in search_paths {
        match find_python(directory) {
            Ok(mut list) => complete_list.append(&mut list),
            Err(error) => println!("dir error: {:?}", error)
        }
    }
    complete_list.sort();

    for entry in complete_list {
        println!("{:?}", entry);
    }
}