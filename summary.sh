#!/bin/sh

cargo run --release > summary.yaml
bat summary.yaml
rm summary.yaml
