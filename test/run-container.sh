#!/bin/sh
set -eu

podman run -i --rm -v .:/source -w /source ${1:-registry.fedoraproject.org/fedora:latest} <<EOF
dnf install -y cargo clippy rustfmt
test/run.sh
[ -z "${DEBUG:-}" ] || sleep infinity
EOF
