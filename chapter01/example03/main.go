package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
)

var counter1 int
var counter2 int
var loops int
var mu sync.Mutex

// without synchronization - produces race condition -> use -race flag while running the program
func worker1(wg *sync.WaitGroup) {
	defer wg.Done()
	for i := 0; i < loops; i++ {
		counter1++ // not safe
	}
}

func worker2(wg *sync.WaitGroup) {
	defer wg.Done() // one done per add
	for i := 0; i < loops; i++ {
		mu.Lock()
    counter2++
    mu.Unlock()
	}
}

// NOTES:
// 1. call wg.Add(N) before starting N goroutines
// 2. Call wg.Done() exactly once per goroutine
// 3. Pass *sync.WaitGroup into goroutines

func main() {
	if len(os.Args) != 2 {
		fmt.Println("usage: threads <value>")
		os.Exit(1)
	}

	loops, _ = strconv.Atoi(os.Args[1])

	var wg1 sync.WaitGroup
	wg1.Add(2) // match the number of goroutines

	fmt.Printf("Initial Value: %d\n", counter1)

	go worker1(&wg1)
	go worker1(&wg1)

	wg1.Wait()
	fmt.Printf("Final Value: %d\n", counter1)

  var wg2 sync.WaitGroup
  wg2.Add(2)
  fmt.Printf("Initial Value: %d\n", counter2)

	go worker2(&wg2)
	go worker2(&wg2)
	wg2.Wait()
	fmt.Printf("Final Value: %d\n", counter2)
}
