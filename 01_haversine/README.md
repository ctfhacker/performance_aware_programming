# Haversine Distance Problem

This repository is an attempt at the first problem in Casey Muratori's 
[Performance Aware Programming](https://www.computerenhance.com/) series.

The problem will be to optimize an implementation of the haversine
distance problem, starting with a python implementation.

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

Naive python script found in `naive.py`

## Rust implementation

Rust implementation included with a comparsion between `serde_json` and `simd_json`.

## Testing

Test script included to generate data sets of `10000`, `100000`, `1000000`, `10000000`
and then run each one through the test suite

```
./test.sh
```

```
CPU: Intel(R) Xeon(R) W-2245 CPU @ 3.90GHz
Data already generated.. skipping
Building the rust harness
-------------------- data_10000000_flex.json --------------------
################ Python ################
Result     =  10005.61
Input      =   7.4336 seconds |  46.71% of total time
Math       =   8.4795 seconds |  53.29% of total time
Total      =  15.9130 seconds
Throughput =     0.63 Mhaversines/seconds
################ Rust   ################
Reading via serde_json: 5.50s 
Reading via simd_json:  2.98s 
simdjson speedup over serde: 1.8466x
Result     = 9993.71
Input      = 2.9783s    |  88.38% of total time (using simdjson)
Math       = 391.4946ms |  11.62% of total time (using simdjson)
Total      = 3.3698s  seconds (using simdjson)
Throughput = 2.97 Mhaversines/seconds
-------------------- data_1000000_flex.json --------------------
################ Python ################
Result     =  10007.36
Input      =   0.7643 seconds |  47.67% of total time
Math       =   0.8391 seconds |  52.33% of total time
Total      =   1.6034 seconds
Throughput =     0.62 Mhaversines/seconds
################ Rust   ################
Reading via serde_json: 573.10ms
Reading via simd_json:  322.89ms
simdjson speedup over serde: 1.7749x
Result     = 10007.34
Input      = 322.8870ms |  89.07% of total time (using simdjson)
Math       = 39.6361ms  |  10.93% of total time (using simdjson)
Total      = 362.5231ms seconds (using simdjson)
Throughput = 2.76 Mhaversines/seconds
-------------------- data_100000_flex.json --------------------
################ Python ################
Result     =  10015.28
Input      =   0.0768 seconds |  47.03% of total time
Math       =   0.0866 seconds |  52.97% of total time
Total      =   0.1634 seconds
Throughput =     0.61 Mhaversines/seconds
################ Rust   ################
Reading via serde_json: 55.78ms
Reading via simd_json:  30.97ms
simdjson speedup over serde: 1.8008x
Result     = 10015.27
Input      = 30.9749ms  |  88.49% of total time (using simdjson)
Math       = 4.0295ms   |  11.51% of total time (using simdjson)
Total      = 35.0044ms seconds (using simdjson)
Throughput = 2.86 Mhaversines/seconds
-------------------- data_10000_flex.json --------------------
################ Python ################
Result     =  10022.03
Input      =   0.0080 seconds |  48.99% of total time
Math       =   0.0084 seconds |  51.01% of total time
Total      =   0.0164 seconds
Throughput =     0.61 Mhaversines/seconds
################ Rust   ################
Reading via serde_json: 5.60ms
Reading via simd_json:  2.75ms
simdjson speedup over serde: 2.0400x
Result     = 10022.02
Input      = 2.7470ms   |  86.99% of total time (using simdjson)
Math       = 411.0130µs |  13.01% of total time (using simdjson)
Total      = 3.1581ms seconds (using simdjson)
Throughput = 3.17 Mhaversines/seconds
-------------------- data_10_flex.json --------------------
################ Python ################
Result     =  10524.02
Input      =   0.0001 seconds |  44.24% of total time
Math       =   0.0001 seconds |  55.76% of total time
Total      =   0.0002 seconds
Throughput =     0.06 Mhaversines/seconds
################ Rust   ################
Reading via serde_json: 44.38µs
Reading via simd_json:  32.94µs
simdjson speedup over serde: 1.3472x
Result     = 10524.02
Input      = 32.9400µs  |  60.73% of total time (using simdjson)
Math       = 21.2960µs  |  39.27% of total time (using simdjson)
Total      = 54.2360µs seconds (using simdjson)
Throughput = 0.18 Mhaversines/seconds
```
