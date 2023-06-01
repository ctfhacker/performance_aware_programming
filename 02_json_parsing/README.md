# Section 2 - Intel x86 

This directory contains the code for Section 2 of the Performance Aware Programming course.

* [Haversine data generator](./gen_haversine_data)

```
cd gen_haversine_data
cargo run -r -- 1000 1234
```

```
Generating 1000 random points from range (-90..90, (-180..180))
Haversine: 5330.02296717414537852164
Generated file written to data_1000_seed_1234.json
Generated ansewr written to data_1000_seed_1234.answer
```

* [Manual JSON parser](./json_parser)

```
cd json_parser
cargo run -r -- ./data_1000_seed_1234.json
```

```
Input size: 102184 (99.79 KB)
Using answer file: "./data_1000_seed_1234.answer"
Pair count: 1000
Haversine: 5330.02296717414537852164
--- Validation ---
Answer:    5330.02296717414537852164
Difference:   0.00000000000000000000
```