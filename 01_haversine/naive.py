from math import radians, sin, cos, sqrt, asin 
import time
import json
import argparse

# Parse the command line for the input file
parser = argparse.ArgumentParser()
parser.add_argument("input", help="Input JSON file to time")
args = parser.parse_args()

print(f'---- Parsing {args.input} ---- ')

# Open the given JSON file
json_file = open(args.input)

# Read the input
start_time = time.time()
data = json.load(json_file)
mid_time = time.time()

# Average the haversines
def haversine_degrees(x0, y0, x1, y1, radius):
    dY = radians(y1 - y0)
    dX = radians(x1 - x0)
    y0 = radians(y0)
    y1 = radians(y1)

    root = (sin(dY/2)**2) + cos(y0) * cos(y1) * (sin(dX/2)**2)
    result = 2 * radius * asin(sqrt(root))
    return result

earth_radius_km = 6371
sum = 0
count = 0
for pair in data['pairs']:
    sum += haversine_degrees(pair['x0'], pair['y0'], pair['x1'], pair['y1'], earth_radius_km)
    count += 1

average = sum / count
end_time = time.time()

total_time = end_time - start_time
input_time = mid_time - start_time
math_time  = end_time - mid_time

# Display the result
print(f'Result: {average:6.2f}')
print(f'Input = {input_time:8.4f} seconds | {input_time / total_time * 100.:6.2f}% of total time')
print(f'Math  = {math_time:8.4f} seconds | {math_time / total_time * 100.:6.2f}% of total time')
print(f'Total = {(end_time - start_time):8.4f} seconds')
print(f'Throughput = {count / (end_time - start_time):8.2f} haversines/seconds')
