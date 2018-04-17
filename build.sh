#!/bin/sh

set -e

cargo build --target $TARGET --all-features --release

if [ -z $DISABLE_EXAMPLES ]; then
	# TODO: Make building examples with cargo work. Currently only works with xargo.
	# cargo build --target $TARGET --all-features --examples
	cargo build --target $TARGET --all-features
fi
