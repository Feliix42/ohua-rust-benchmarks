#!/bin/bash

echo "Preparing test-build setup ..."
cd ..
rm -rf build
mkdir -p build
rm src/generated/futures.rs
cp -R src build

cd build/src

echo "done."

echo "Running ohuac ..."
mkdir -p generated
# run the Ohua compiler to produce the graph output
ohuac build original/futures.rs -c ohua.yaml -o generated/futures.ohuao
echo "done."

jq . generated/futures.ohuao > generated/futures.ohuao.tmp
mv generated/futures.ohuao.tmp generated/futures.ohuao

echo "Running rust code gen ..."
# produce rust code
ohuarust --intermediate generated/futures.ohuao --output generated/futures.rs
cp generated/futures.rs ../../src/generated
echo "done."
