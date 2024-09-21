#!/bin/sh
#
# Use this script to run your program LOCALLY.
#
# Note: Changing this script WILL NOT affect how CodeCrafters runs your program.
#
# Learn more: https://codecrafters.io/program-interface

# set -x # Print commands before executing them
set -e # Exit early if any commands fail
# set -o pipefail # Propagate exit code of last command in pipe to exit code of this script

# Copied from .codecrafters/compile.sh
#
# - Edit this to change how your program compiles locally
# - Edit .codecrafters/compile.sh to change how your program compiles remotely
(
  cd "$(dirname "$0")" # Ensure compile steps are run within the repository directory
  cargo build --release --target-dir=/tmp/codecrafters-build-grep-rust --manifest-path Cargo.toml
)

# Copied from .codecrafters/run.sh
#
# - Edit this to change how your program runs locally
# - Edit .codecrafters/run.sh to change how your program runs remotely
exec /tmp/codecrafters-build-grep-rust/release/codecrafters-grep "$@"
