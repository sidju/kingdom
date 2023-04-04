#!/bin/sh

grep -i -B 1 --color=auto -n '# Holds: ' *.yaml
