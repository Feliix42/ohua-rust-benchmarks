#!/bin/bash

mkdir -p seq/src/vacation/prime/generated
rm -rf seq/src/vacation/prime/generated/*
echo pub mod server\; > seq/src/vacation/prime/generated/mod.rs
ohuac build seq/src/vacation/prime/server.rs -o seq/src/vacation/prime/generated -c ohua-config.yaml
