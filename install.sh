#!/bin/bash

ARG1=$1

TARGET=${ARG1:-"/usr/local"}

echo "Installing pyedge to \"$TARGET\""
DIR=$(dirname $(readlink -f $0))

function maybe_sudo() {
    if ! [ $(id -u) = 0 ]; then
        sudo $1
    else
        $1
    fi
}

maybe_sudo "cp -fv $DIR/target/x86_64-unknown-linux-musl/release/pyedge $TARGET/bin/pyedge"
