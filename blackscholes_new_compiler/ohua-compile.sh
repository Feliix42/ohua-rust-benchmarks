#!/bin/bash

mkdir -p src/generated
rm -rf src/generated/*
echo pub mod original\; > src/generated/mod.rs
ohuac build src/original.rs -o src/generated -c ohua-config.yaml
