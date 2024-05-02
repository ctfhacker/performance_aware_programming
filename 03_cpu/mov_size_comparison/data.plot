set terminal png size 1920,1080
set output 'data.png'
set title 'MOV width comparisons'
set xlabel 'Number of movs'
set ylabel 'Throughput GB/s'
set xtics (1, 2, 3, 4, 5, 6)
plot 'data/read_16_samereg' with linespoints, \
'data/read_32_samereg' with linespoints, \
'data/read_4_samereg' with linespoints, \
'data/read_64_samereg' with linespoints, \
'data/read_8_samereg' with linespoints
