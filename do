#!/bin/bash

# dive into rt folder
cd rt

# build release executable
cargo build --release

# jump up to root folder
cd ..

# copy release executable to  rt folder level with same name
cp ./rt/target/release/rt ./rt/rt