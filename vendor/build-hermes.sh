#!/bin/bash
set -euxo pipefail
cd hermes
CC=/usr/bin/gcc-14 CXX=/usr/bin/g++-14 cmake -S . -B build -G Ninja
CC=/usr/bin/gcc-14 CXX=/usr/bin/g++-14 cmake --build ./build
