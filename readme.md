### about
This is a comparison of golang's and rust's performance for a implementation of fibonacci algorithm. 

### results
OBS: rust is the `async-std` branch

| name       | average (ns/operation)|
|------------|----------------------|
| fib_go_2k  | 4.918                |
| fib_r_2k   | 2.339                |
| fib_go_3k  | 11.244               |
| fib_r_3k   | 4.345                |
| fib_go_10k | 83.450               |
| fib_r_10k  | 34.594               |
| fib_go_20k | 140.047              |
| fib_r_20k  | 122.787              |
| fib_go_40k | 474.574              |
| fib_r_40k  | 467.620              |