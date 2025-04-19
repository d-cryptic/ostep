package main

import (
	"fmt"
	"os"
	"syscall"
)

//Go’s runtime (the scheduler, garbage collector, goroutines, etc.) doesn’t like
// fork() alone — especially on macOS and Linux with Go 1.20+ — because:
// 1. it launches multiple threads
// 2. And when you do fork() directly (without exec),
// the child process is in a weird state: only one thread is copied, while the
// rest of the Go runtime is.
//
// Why example01 worked?
// Both parent and child immediately just print and exit.
// You’re not calling wait(), so there’s no interaction with the Go runtime’s
// internal expectations around process tracking.
// Since fork() is used very lightly and the child exits quickly,
// it avoids any serious interference with the Go scheduler.
//
// Why adding wait() causes trouble?
// syscall.Wait4(int(pid), ...)
// go's runtime expects:
// 1. proper process tracking
// 2. clean state
// 3. full functional fork+exec lifecycle
//
// Since Go is not multi threaded (it has its own scheduler and runtime threads),
// calling fork() alone and then using wait() - without immediately doing exec() causes:
// - memory/state confusion
// - race conditions internally
// - wait - no child processes errors

func main() {
	fmt.Printf("hello (pid:%d)\n", os.Getpid())

	// fork process
	pid, _, err := syscall.RawSyscall(syscall.SYS_FORK, 0, 0, 0)
	if err != 0 {
		fmt.Fprintf(os.Stderr, "fork failed: %v\n", err)
		os.Exit(1)
	}

	if pid == 0 {
		// child process
		fmt.Printf("Child (pid:%d)\n", os.Getpid())
	} else {
		// parent process
		var status syscall.WaitStatus
		wpid, err := syscall.Wait4(int(pid), &status, 0, nil)
		if err != nil {
			fmt.Fprintf(os.Stderr, "wait failed: %v\n", err)
			os.Exit(1)
		}

		fmt.Printf("parent of %d (rc_wait:%d) (pid:%d)\n", pid, wpid, os.Getpid())
	}

	child()
}
