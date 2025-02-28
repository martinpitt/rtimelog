#!/bin/sh
set -eu

podman run -i --rm -v .:/source -w /source ${1:-registry.fedoraproject.org/fedora:latest} <<EOF
set -eu
dnf install -y cargo clippy rustfmt
if ! test/run.sh; then
    [ -z "${DEBUG:-}" ] || sleep infinity
    exit 1
fi
EOF
