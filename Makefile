.PHONY: default
default: build ;

PREFIX?=/usr/local

install:
	cp -v target/release/pyedge ${PREFIX}/bin

build:
	echo Building pyedge...
	cargo build --release

clean:
	cargo clean
	echo Build directory cleaned up
	
uninstall:
	rm -vf ${PREFIX}/bin/pyedge
