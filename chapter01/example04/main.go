package main

import (
  "fmt"
  "os"
)

func main() {
  f, err  := os.OpenFile("/tmp/file", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0700)
  if err != nil {
    panic(err)
  }

  defer f.Close()

  n, err := f.WriteString("hello world\n")
  if err != nil {
    panic(err)
  }

  if n != 13 {
    // panic(fmt.Sprintf("expected 13 bytes, wrote %d", n))
    fmt.Sprintf("expected 13 bytes, wrote %d", n)
  }

  lowLevelImplementation()
}
