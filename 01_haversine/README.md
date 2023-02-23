# Haversine Distance Problem

This repository is an attempt at the first problem in Casey Muratori's 
[Performance Aware Programming](https://www.computerenhance.com/) series.

The problem will be to optimize an implementation of the haversine
distance problem, starting with a python implementation.

This repo uses Rust and Python. Be sure to [install rust](https://www.rust-lang.org/tools/install)
and `apt install python3`.

## Generating input data

We are testing against a 10 million data point input for the initial
tests. Includes in this repo is a utility to generate this random
data.

```
cd gen_haversine_data
cargo run -r -- 10000000
```

This will write the file `data_10000000_flex.json` to the local directory.

## Naive python

Naive python script found in `naive.py`. 

## Testing

Test script included to generate data sets of `10000`, `100000`, `1000000`, `10000000`
and then run each one through the test suite

```
./test.sh
```

```
Data already generated.. skipping
#################### Naive Python ####################
---- Parsing data_10000000_flex.json ----
Result: 10010.86
Input =   7.2801 seconds |  46.44% of total time
Math  =   8.3960 seconds |  53.56% of total time
Total =  15.6761 seconds
Throughput = 637913.92 haversines/seconds
---- Parsing data_1000000_flex.json ----
Result: 10003.04
Input =   0.7437 seconds |  46.33% of total time
Math  =   0.8614 seconds |  53.67% of total time
Total =   1.6051 seconds
Throughput = 623026.21 haversines/seconds
---- Parsing data_100000_flex.json ----
Result: 10010.18
Input =   0.0770 seconds |  46.81% of total time
Math  =   0.0875 seconds |  53.19% of total time
Total =   0.1645 seconds
Throughput = 607812.89 haversines/seconds
---- Parsing data_10000_flex.json ----
Result: 10033.42
Input =   0.0077 seconds |  47.98% of total time
Math  =   0.0084 seconds |  52.02% of total time
Total =   0.0161 seconds
Throughput = 620321.53 haversines/seconds
---- Parsing data_1000_flex.json ----
Result: 9775.77
Input =   0.0008 seconds |  45.57% of total time
Math  =   0.0009 seconds |  54.43% of total time
Total =   0.0017 seconds
Throughput = 582785.05 haversines/seconds
```
