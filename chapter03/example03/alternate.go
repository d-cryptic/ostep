package main

import (
	"fmt"
	"os"
	"os/exec"
)

func alternate() {
	fmt.Printf("hello (pid:%d)\n", os.Getpid())

	// simulate a fork exec model using exec.Command
	cmd := exec.Command("wc", "main.go")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	err := cmd.Start()
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to start command: %v\n", err)
		os.Exit(1)
	}

	// simulated "child" process
	fmt.Printf("child (pid:%d)\n", cmd.Process.Pid)

	// parent waits for child to complete
	err = cmd.Wait()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Command failed: %v\n", err)
	}

	fmt.Printf("parent of %d (pid:%d)\n", cmd.Process.Pid, os.Getpid())
}
