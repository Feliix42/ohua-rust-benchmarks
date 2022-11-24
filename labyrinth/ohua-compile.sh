#!/bin/bash

mkdir -p src/ohua/generated
rm -rf src/ohua/generated/*
echo pub mod original\; > src/ohua/generated/mod.rs
ohuac build src/ohua/original.rs -o src/ohua/generated -c ohua-config.yaml
