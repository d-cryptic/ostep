package main

import (
  "fmt"
  "os"
  "time"
)

func Spin(seconds int) {
  start := time.Now()
  for {
    if time.Since(start).Seconds() >= float64(seconds) {
      break
    }
  }
}

func main() {
  // each process has its own memory space
  // so p can point to same virtual address
  // but it's actually different physical address under the hood
  p := new(int)
  fmt.Printf("(%d) address pointed to by p: %p\n", os.Getpid(), p)

  *p = 0
  for {
    Spin(1)
    *p += 1
    fmt.Printf("(%d) p: %d\n", os.Getpid(), *p)
  }
}
