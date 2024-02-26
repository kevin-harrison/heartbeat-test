# heartbeat-test
## To Run
Start process with PID 1 first
```bash
cargo run 1
```
Then start process with PID 2
```bash
cargo run 2
```

## Problem
The process with PID 1 will exhibit a cyclic latency spike pattern but I don't know why. The following is an example (each line is the time to send a heartbeat request and then recieve its reply): 
```
0
0
0
28
0
0
20
0
0
2
0
0
0
0
0
0
0
0
0
0
0
0
39
0
0
29
0
0
13
1
0
1
0
0
0
0
0
0
0
0
0
0
1
1
38
0
0
20
0
0
11
0
0
0
0
```
