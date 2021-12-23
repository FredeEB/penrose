#!/usr/bin/env bash
# call from root dir
git clean -dfx aux
pushd aux
git archive -o target.tar.gz HEAD
makepkg -Ccsi
popd
