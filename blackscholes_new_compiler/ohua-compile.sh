#!/bin/bash

mkdir -p src/generated
rm -rf src/generated/*
echo pub mod original\; > src/generated/mod.rs
echo pub mod opt\; >> src/generated/mod.rs
ohuac build src/original.rs -o src/generated -c ohua-config.yaml
ohuac build src/opt.rs -o src/generated -c ohua-config.yaml
