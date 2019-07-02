#!/bin/bash

set -euxo pipefail

cargo check --target $TARGET
