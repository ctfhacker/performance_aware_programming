#!/bin/bash
function generate_data() {
	# Check if data is already generated
	if [ -f "./data_10000000_flex.json" ]; then
		echo "Data already generated.. skipping"
		return
	fi

	# Head into the generator binary
	cd gen_haversine_data

	# Build the generator
	cargo build -r 

	# Generate various lengths of data
	./target/release/gen_haversine_data 10000 
	./target/release/gen_haversine_data 100000 
	./target/release/gen_haversine_data 1000000 
	./target/release/gen_haversine_data 10000000 

	# Copy the data to the parent directory
	mv data*json ..

	# Back to root
	cd ..
}


function test_naive_python() {
  echo "#################### Naive Python ####################"
	for f in $(ls data*); do python3 naive.py $f; done
}

generate_data
test_naive_python
	