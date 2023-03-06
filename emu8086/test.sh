#!/bin/bash

set -e

build() {
	cargo build -r 
}

test() {
	build

  # Clean the old results
	/bin/rm tests/listing*rebuilt*

	# Build all known asm test files
	for f in $(ls tests/listing_*asm); do
		nasm $f
	done

	# Decode everything into a `.asm.rebuilt`
	for f in $(ls tests/listing_* | rg -v asm); do
		./target/release/emu8086 $f > $f.asm.rebuilt.decoded  2>/dev/null
	done

	# Build the output
	for f in $(ls tests/listing_*asm.rebuilt.decoded); do
		nasm $f
	done

	# radare2 diff
	for f in $(ls tests/listing_* | rg -v asm); do
		echo $f $f.asm.rebuilt
	  radiff2 $f $f.asm.rebuilt
	done
}

test
