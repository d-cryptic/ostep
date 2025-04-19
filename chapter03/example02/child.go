package main

import (
	"fmt"
	"os"
	"os/exec"
)

// fork() only - works, simple, avoids runtime interference
// fork + wait - fail - child dies quickly or runtime gets confused
// exec.Command() or ForkExec - recommended way in Go for child process creation
//
//
// Below implementation is how the Go runtime expects process creation to work -
// like fork() + exec() in C
// The same Go binary is executed again, with "child" as an argument.
// When that happens, the new process runs the child branch.
// This avoids fork() alone, which causes trouble in the Go runtime.

func child() {
	fmt.Printf("hello (pid:%d)\n", os.Getpid())

	execPath, err := os.Executable()
	if err != nil {
		fmt.Fprintf(os.Stderr, "could not find executable: %v\n", err)
		os.Exit(1)
	}

	cmd := exec.Command(execPath, "child")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if len(os.Args) > 1 && os.Args[1] == "child" {
		fmt.Printf("Child (pid:%d)\n", os.Getpid())
		os.Exit(0)
	}

	// parent spawns child process
	err = cmd.Run()
	if err != nil {
		fmt.Fprintf(os.Stderr, "exec failed: %v\n", err)
	}

	fmt.Printf("parent (pid:%d)\n", os.Getpid())
}
