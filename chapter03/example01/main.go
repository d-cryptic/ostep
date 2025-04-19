package main

import (
	"fmt"
	"log"
	"os"
	"syscall"
)

// The OS creates an exact copy of the current process: same memory, same code, same program counter.
// •	But — here’s the key — the return value of fork() is different in each process:
// •	In the parent process, fork() returns the PID of the child
// •	In the child process, fork() returns 0
// •	If something goes wrong, it returns -1

// After fork():
// 	•	The kernel duplicates the process (memory + registers).
// 	•	The only difference in the child is the return value of fork() (it’s zero).
// 	•	Then, both parent and child start executing from the next instruction after the fork() call.
// That’s how both the parent and child get to run the same getpid() line, but fork() helps you figure out who you are in the new world.

func main() {
	fmt.Printf("hello (pid:%d)\n", os.Getpid())

	// fork the process
	pid, _, err := syscall.RawSyscall(syscall.SYS_FORK, 0, 0, 0) // RawSysCall -  low-level, unsafe, and not portable across platforms.
	if err != 0 {
		log.Fatalf("fork failed: %v\n", err)
	}

	if pid == 0 {
		// child process
		fmt.Printf("child (pid:%d)\n", os.Getpid())
	} else {
		// parent process
		fmt.Printf("parent of %d (pid:%d)\n", pid, os.Getpid())
	}
}
