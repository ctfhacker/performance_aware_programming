#!/bin/bash
function print_cpuinfo() {
	if [ -f "/proc/cpuinfo" ]; then
		MODEL=`grep "model name" /proc/cpuinfo | tr -s ' ' | cut -d' ' -f3- | head -n 1`
		echo "CPU: $MODEL"
	fi
}

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


function test() {
	echo "Building the rust harness"
	cd ./rust
	cargo build -r >/dev/null 2>/dev/null
	cd ..

	for f in $(ls data*); do 
	  echo "-------------------- $f --------------------"
	  # echo "################ Python ################"
		# python3 naive.py $f
	  echo "################ Rust   ################"
		./rust/target/release/haversine_distance $f
	done
}

# Run the tests, capturing the output
print_cpuinfo 
generate_data 
test

print_cpuinfo > output
generate_data >> output
test >> output

# Create the readme with the current test output
cp .README.md.orig README.md
echo '```' >> README.md
cat output >> README.md
echo '```' >> README.md

# Remove the output file
/bin/rm output
	