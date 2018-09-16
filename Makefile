.PHONY: default
default: build ;

PREFIX?=/usr/local

build:
	@echo Building pyedge...
	cargo build --release --target=x86_64-unknown-linux-musl

install: build
	./install.sh ${PREFIX}

clean:
	cargo clean
	rm -rfv target/x86_64-unknown-linux-musl
	@echo Build directory cleaned up

ninstall:
	rm -vf ${PREFIX}/bin/pyedge
