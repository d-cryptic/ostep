# Overview
> Golang Implementation of [Link](https://github.com/remzi-arpacidusseau/ostep-homework/blob/master/cpu-intro/README.md)
> Simulate the state of a process state changes as it runs on a CPU.

```
RUNNING - the process is using the CPU right now
READY   - the process could be using the CPU right now
          but (alas) some other process is
BLOCKED - the process is waiting on I/O
          (e.g., it issued a request to a disk)
DONE    - the process is finished executing
```
## Build the binary

```
go build .
```

## Get the options

```
./homework01 -h
```

## use the cpu, no IO
```
./homework01 -l 5:100
```

## see what happens under the hood

```
./homework01 -l 5:100 -c
```

## with io

```
./homework01 -l 3:0 -L 5
./homework01 -l 3:0 -L 5 -c
```

## print stats

```
./homework01 -l 3:0 -L 5 -p
./homework01 -l 3:0 -L 5 -c -p
```
