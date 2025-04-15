package main

import (
	"fmt"
	"syscall"
)

func lowLevelImplementation() {
  const (
    filePath = "/tmp/file"
    flags = syscall.O_WRONLY | syscall.O_CREAT | syscall.O_TRUNC
    mode = 0700
  )

  // open the file
  fd, err := syscall.Open(filePath, flags, uint32(mode))
  if err != nil {
    fmt.Sprintf("open failed: %w", err)
  }

  defer syscall.Close(fd)

  // writing data
  data := []byte("hello world\n")
  n, err := syscall.Write(fd, data)
  if err != nil {
    panic(fmt.Sprintf("write failed: %v", err))
  }

  if n != len(data) {
    panic(fmt.Sprintf("expected to write %d bytes, but wrote %d", len(data), n))
  }
}
