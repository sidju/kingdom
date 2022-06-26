#!/bin/sh

grep -i -B 1 --color=auto -n '# \(magic item slots\?\|minor\|medium\|major\|armor\)' *.yaml
