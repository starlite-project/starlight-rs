#!/bin/sh

git clone https://github.com/rui314/mold.git
cd mold
git checkout v1.0.3
make -j$(nproc) CXX=clang++
make install
cd ..
