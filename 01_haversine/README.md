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
################ Rust   ################
Reading via serde_json: 5.85s 
Reading via simd_json:  3.00s 
simdjson speedup over serde: 1.9520x
Input      = 2.9958s   
----------------------------------- naive_work with 1 cores -----------------------------------
Average                        = 9993.71
simdjson                       = 2.9958s    |  88.43% of total time (using simdjson)
naive_work                     = 392.0557ms |  11.57% of total time (using simdjson)
Total                          = 3.3879s  seconds (using simdjson)
Throughput                     = 2.95 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 2 cores -------------------------------
Average                        = 10005.64
simdjson                       = 2.9958s    |  92.23% of total time (using simdjson)
rayon_work_par_iter            = 252.5576ms |   7.77% of total time (using simdjson)
Total                          = 3.2484s  seconds (using simdjson)
Throughput                     = 3.08 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 4 cores -------------------------------
Average                        = 10005.64
simdjson                       = 2.9958s    |  93.78% of total time (using simdjson)
rayon_work_par_iter            = 198.6084ms |   6.22% of total time (using simdjson)
Total                          = 3.1944s  seconds (using simdjson)
Throughput                     = 3.13 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 8 cores -------------------------------
Average                        = 10005.64
simdjson                       = 2.9958s    |  93.27% of total time (using simdjson)
rayon_work_par_iter            = 216.3191ms |   6.73% of total time (using simdjson)
Total                          = 3.2121s  seconds (using simdjson)
Throughput                     = 3.11 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 12 cores -------------------------------
Average                        = 10005.64
simdjson                       = 2.9958s    |  93.82% of total time (using simdjson)
rayon_work_par_iter            = 197.2389ms |   6.18% of total time (using simdjson)
Total                          = 3.1931s  seconds (using simdjson)
Throughput                     = 3.13 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 16 cores -------------------------------
Average                        = 10005.64
simdjson                       = 2.9958s    |  93.84% of total time (using simdjson)
rayon_work_par_iter            = 196.7106ms |   6.16% of total time (using simdjson)
Total                          = 3.1925s  seconds (using simdjson)
Throughput                     = 3.13 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 2 cores ------------------------------
Average                        = 10003.23
simdjson                       = 2.9958s    |  93.14% of total time (using simdjson)
manual_chunk_parallel          = 220.5439ms |   6.86% of total time (using simdjson)
Total                          = 3.2164s  seconds (using simdjson)
Throughput                     = 3.11 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 4 cores ------------------------------
Average                        = 10005.05
simdjson                       = 2.9958s    |  96.10% of total time (using simdjson)
manual_chunk_parallel          = 121.6410ms |   3.90% of total time (using simdjson)
Total                          = 3.1175s  seconds (using simdjson)
Throughput                     = 3.21 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 8 cores ------------------------------
Average                        = 10005.56
simdjson                       = 2.9958s    |  97.71% of total time (using simdjson)
manual_chunk_parallel          = 70.1619ms  |   2.29% of total time (using simdjson)
Total                          = 3.0660s  seconds (using simdjson)
Throughput                     = 3.26 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 12 cores ------------------------------
Average                        = 10005.59
simdjson                       = 2.9958s    |  98.19% of total time (using simdjson)
manual_chunk_parallel          = 55.1001ms  |   1.81% of total time (using simdjson)
Total                          = 3.0509s  seconds (using simdjson)
Throughput                     = 3.28 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 16 cores ------------------------------
Average                        = 10005.62
simdjson                       = 2.9958s    |  98.38% of total time (using simdjson)
manual_chunk_parallel          = 49.2437ms  |   1.62% of total time (using simdjson)
Total                          = 3.0451s  seconds (using simdjson)
Throughput                     = 3.28 Mhaversines/seconds
-------------------- data_1000000_flex.json --------------------
################ Rust   ################
Reading via serde_json: 616.13ms
Reading via simd_json:  320.07ms
simdjson speedup over serde: 1.9250x
Input      = 320.0703ms
----------------------------------- naive_work with 1 cores -----------------------------------
Average                        = 10007.34
simdjson                       = 320.0703ms |  89.14% of total time (using simdjson)
naive_work                     = 39.0093ms  |  10.86% of total time (using simdjson)
Total                          = 359.0796ms seconds (using simdjson)
Throughput                     = 2.78 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 2 cores -------------------------------
Average                        = 10007.37
simdjson                       = 320.0703ms |  87.87% of total time (using simdjson)
rayon_work_par_iter            = 44.1890ms  |  12.13% of total time (using simdjson)
Total                          = 364.2593ms seconds (using simdjson)
Throughput                     = 2.75 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 4 cores -------------------------------
Average                        = 10007.38
simdjson                       = 320.0703ms |  93.54% of total time (using simdjson)
rayon_work_par_iter            = 22.1076ms  |   6.46% of total time (using simdjson)
Total                          = 342.1779ms seconds (using simdjson)
Throughput                     = 2.92 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 8 cores -------------------------------
Average                        = 10007.38
simdjson                       = 320.0703ms |  94.15% of total time (using simdjson)
rayon_work_par_iter            = 19.8769ms  |   5.85% of total time (using simdjson)
Total                          = 339.9472ms seconds (using simdjson)
Throughput                     = 2.94 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 12 cores -------------------------------
Average                        = 10007.38
simdjson                       = 320.0703ms |  94.11% of total time (using simdjson)
rayon_work_par_iter            = 20.0380ms  |   5.89% of total time (using simdjson)
Total                          = 340.1083ms seconds (using simdjson)
Throughput                     = 2.94 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 16 cores -------------------------------
Average                        = 10007.38
simdjson                       = 320.0703ms |  94.19% of total time (using simdjson)
rayon_work_par_iter            = 19.7531ms  |   5.81% of total time (using simdjson)
Total                          = 339.8234ms seconds (using simdjson)
Throughput                     = 2.94 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 2 cores ------------------------------
Average                        = 10007.37
simdjson                       = 320.0703ms |  88.92% of total time (using simdjson)
manual_chunk_parallel          = 39.8912ms  |  11.08% of total time (using simdjson)
Total                          = 359.9615ms seconds (using simdjson)
Throughput                     = 2.78 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 4 cores ------------------------------
Average                        = 10007.31
simdjson                       = 320.0703ms |  92.27% of total time (using simdjson)
manual_chunk_parallel          = 26.8300ms  |   7.73% of total time (using simdjson)
Total                          = 346.9003ms seconds (using simdjson)
Throughput                     = 2.88 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 8 cores ------------------------------
Average                        = 10007.29
simdjson                       = 320.0703ms |  95.00% of total time (using simdjson)
manual_chunk_parallel          = 16.8537ms  |   5.00% of total time (using simdjson)
Total                          = 336.9241ms seconds (using simdjson)
Throughput                     = 2.97 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 12 cores ------------------------------
Average                        = 10007.22
simdjson                       = 320.0703ms |  96.47% of total time (using simdjson)
manual_chunk_parallel          = 11.7252ms  |   3.53% of total time (using simdjson)
Total                          = 331.7955ms seconds (using simdjson)
Throughput                     = 3.01 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 16 cores ------------------------------
Average                        = 10007.20
simdjson                       = 320.0703ms |  97.08% of total time (using simdjson)
manual_chunk_parallel          = 9.6113ms   |   2.92% of total time (using simdjson)
Total                          = 329.6816ms seconds (using simdjson)
Throughput                     = 3.03 Mhaversines/seconds
-------------------- data_100000_flex.json --------------------
################ Rust   ################
Reading via serde_json: 81.50ms
Reading via simd_json:  31.59ms
simdjson speedup over serde: 2.5794x
Input      = 31.5945ms 
----------------------------------- naive_work with 1 cores -----------------------------------
Average                        = 10015.27
simdjson                       = 31.5945ms  |  88.96% of total time (using simdjson)
naive_work                     = 3.9196ms   |  11.04% of total time (using simdjson)
Total                          = 35.5141ms seconds (using simdjson)
Throughput                     = 2.82 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 2 cores -------------------------------
Average                        = 10015.28
simdjson                       = 31.5945ms  |  89.07% of total time (using simdjson)
rayon_work_par_iter            = 3.8788ms   |  10.93% of total time (using simdjson)
Total                          = 35.4733ms seconds (using simdjson)
Throughput                     = 2.82 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 4 cores -------------------------------
Average                        = 10015.28
simdjson                       = 31.5945ms  |  89.65% of total time (using simdjson)
rayon_work_par_iter            = 3.6461ms   |  10.35% of total time (using simdjson)
Total                          = 35.2407ms seconds (using simdjson)
Throughput                     = 2.84 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 8 cores -------------------------------
Average                        = 10015.28
simdjson                       = 31.5945ms  |  89.58% of total time (using simdjson)
rayon_work_par_iter            = 3.6742ms   |  10.42% of total time (using simdjson)
Total                          = 35.2688ms seconds (using simdjson)
Throughput                     = 2.84 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 12 cores -------------------------------
Average                        = 10015.28
simdjson                       = 31.5945ms  |  89.84% of total time (using simdjson)
rayon_work_par_iter            = 3.5712ms   |  10.16% of total time (using simdjson)
Total                          = 35.1657ms seconds (using simdjson)
Throughput                     = 2.84 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 16 cores -------------------------------
Average                        = 10015.28
simdjson                       = 31.5945ms  |  90.17% of total time (using simdjson)
rayon_work_par_iter            = 3.4459ms   |   9.83% of total time (using simdjson)
Total                          = 35.0404ms seconds (using simdjson)
Throughput                     = 2.85 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 2 cores ------------------------------
Average                        = 10015.11
simdjson                       = 31.5945ms  |  81.32% of total time (using simdjson)
manual_chunk_parallel          = 7.2588ms   |  18.68% of total time (using simdjson)
Total                          = 38.8534ms seconds (using simdjson)
Throughput                     = 2.57 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 4 cores ------------------------------
Average                        = 10015.01
simdjson                       = 31.5945ms  |  88.94% of total time (using simdjson)
manual_chunk_parallel          = 3.9277ms   |  11.06% of total time (using simdjson)
Total                          = 35.5223ms seconds (using simdjson)
Throughput                     = 2.82 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 8 cores ------------------------------
Average                        = 10014.59
simdjson                       = 31.5945ms  |  93.20% of total time (using simdjson)
manual_chunk_parallel          = 2.3062ms   |   6.80% of total time (using simdjson)
Total                          = 33.9007ms seconds (using simdjson)
Throughput                     = 2.95 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 12 cores ------------------------------
Average                        = 10014.36
simdjson                       = 31.5945ms  |  93.70% of total time (using simdjson)
manual_chunk_parallel          = 2.1252ms   |   6.30% of total time (using simdjson)
Total                          = 33.7198ms seconds (using simdjson)
Throughput                     = 2.97 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 16 cores ------------------------------
Average                        = 10013.81
simdjson                       = 31.5945ms  |  95.06% of total time (using simdjson)
manual_chunk_parallel          = 1.6416ms   |   4.94% of total time (using simdjson)
Total                          = 33.2361ms seconds (using simdjson)
Throughput                     = 3.01 Mhaversines/seconds
-------------------- data_10000_flex.json --------------------
################ Rust   ################
Reading via serde_json: 14.37ms
Reading via simd_json:  8.09ms
simdjson speedup over serde: 1.7760x
Input      = 8.0891ms  
----------------------------------- naive_work with 1 cores -----------------------------------
Average                        = 10022.02
simdjson                       = 8.0891ms   |  85.37% of total time (using simdjson)
naive_work                     = 1.3866ms   |  14.63% of total time (using simdjson)
Total                          = 9.4757ms seconds (using simdjson)
Throughput                     = 1.06 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 2 cores -------------------------------
Average                        = 10022.03
simdjson                       = 8.0891ms   |  91.21% of total time (using simdjson)
rayon_work_par_iter            = 779.6470µs |   8.79% of total time (using simdjson)
Total                          = 8.8688ms seconds (using simdjson)
Throughput                     = 1.13 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 4 cores -------------------------------
Average                        = 10022.03
simdjson                       = 8.0891ms   |  94.03% of total time (using simdjson)
rayon_work_par_iter            = 513.5340µs |   5.97% of total time (using simdjson)
Total                          = 8.6027ms seconds (using simdjson)
Throughput                     = 1.16 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 8 cores -------------------------------
Average                        = 10022.03
simdjson                       = 8.0891ms   |  94.22% of total time (using simdjson)
rayon_work_par_iter            = 496.0210µs |   5.78% of total time (using simdjson)
Total                          = 8.5852ms seconds (using simdjson)
Throughput                     = 1.16 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 12 cores -------------------------------
Average                        = 10022.03
simdjson                       = 8.0891ms   |  94.26% of total time (using simdjson)
rayon_work_par_iter            = 492.1770µs |   5.74% of total time (using simdjson)
Total                          = 8.5813ms seconds (using simdjson)
Throughput                     = 1.17 Mhaversines/seconds
------------------------------- rayon_work_par_iter with 16 cores -------------------------------
Average                        = 10022.03
simdjson                       = 8.0891ms   |  94.02% of total time (using simdjson)
rayon_work_par_iter            = 514.0850µs |   5.98% of total time (using simdjson)
Total                          = 8.6032ms seconds (using simdjson)
Throughput                     = 1.16 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 2 cores ------------------------------
Average                        = 10019.33
simdjson                       = 8.0891ms   |  89.85% of total time (using simdjson)
manual_chunk_parallel          = 914.2370µs |  10.15% of total time (using simdjson)
Total                          = 9.0034ms seconds (using simdjson)
Throughput                     = 1.11 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 4 cores ------------------------------
Average                        = 10016.42
simdjson                       = 8.0891ms   |  92.62% of total time (using simdjson)
manual_chunk_parallel          = 644.8030µs |   7.38% of total time (using simdjson)
Total                          = 8.7339ms seconds (using simdjson)
Throughput                     = 1.14 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 8 cores ------------------------------
Average                        = 10011.80
simdjson                       = 8.0891ms   |  93.27% of total time (using simdjson)
manual_chunk_parallel          = 583.3190µs |   6.73% of total time (using simdjson)
Total                          = 8.6725ms seconds (using simdjson)
Throughput                     = 1.15 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 12 cores ------------------------------
Average                        = 10009.07
simdjson                       = 8.0891ms   |  92.93% of total time (using simdjson)
manual_chunk_parallel          = 614.9970µs |   7.07% of total time (using simdjson)
Total                          = 8.7041ms seconds (using simdjson)
Throughput                     = 1.15 Mhaversines/seconds
------------------------------ manual_chunk_parallel with 16 cores ------------------------------
Average                        = 10001.94
simdjson                       = 8.0891ms   |  90.14% of total time (using simdjson)
manual_chunk_parallel          = 884.9950µs |   9.86% of total time (using simdjson)
Total                          = 8.9741ms seconds (using simdjson)
Throughput                     = 1.11 Mhaversines/seconds
```
