build:
	cargo build -r 
test: build
	nasm tests/listing_37.asm
	nasm tests/listing_38.asm
	./target/release/emu8086 ./tests/listing_37 > tests/listing_37_rebuilt.asm
	./target/release/emu8086 ./tests/listing_38 > tests/listing_38_rebuilt.asm
	nasm tests/listing_37_rebuilt.asm
	nasm tests/listing_38_rebuilt.asm
	# radare2 diff
	radiff2 tests/listing_37 tests/listing_37_rebuilt
	radiff2 tests/listing_38 tests/listing_38_rebuilt

