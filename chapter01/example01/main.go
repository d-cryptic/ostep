package main

import (
  "fmt"
  "os"
  "time"
)

// Spin(1) -> CPU cycles for ~1 second
func Spin(seconds int) {
  start := time.Now()
  for {
    if time.Since(start).Seconds() >= float64(seconds) {
      break
    }
  }
}

func main() {
  if len(os.Args) != 2 {
    fmt.Println("usage: cpu <string>")
    os.Exit(1)
  }

  str := os.Args[1]

  for {
    Spin(1)
    fmt.Println(str)
  }
}
