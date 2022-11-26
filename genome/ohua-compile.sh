#!/usr/bin/env bash

mkdir -p src/generated
rm -rf src/generated/*
echo pub mod ohua\; > src/generated/mod.rs
ohuac build src/ohua.rs -o src/generated -c ohua-config.yaml
