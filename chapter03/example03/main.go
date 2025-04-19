package main

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
)

// child process won't be created as seen in example02
// so created an `alternate()` function which does child creation simulation in go
// c vs go
// fork() vs exec.Command().Start() -> creates subprocess
// execvp(myargs[0], myargs) -> exec.Command(...).Run() or Start() + wait()
// wait(NULL) vs cmd.Wait()

func main() {
	fmt.Printf("hello (pid:%d)\n", os.Getpid())

	// create a child process using fork
	pid, _, errno := syscall.RawSyscall(syscall.SYS_FORK, 0, 0, 0)
	if errno != 0 {
		fmt.Fprintf(os.Stderr, "fork failed: %v\n", errno)
		os.Exit(1)
	}

	if pid == 0 {
		// in child process
		fmt.Printf("child (pid:%d)\n", os.Getpid())

		// replace current process with wc command
		cmd := exec.Command("wc", "main.go")
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr

		if err := cmd.Run(); err != nil {
			fmt.Fprintf(os.Stderr, "exec failed: %v\n", err)
			os.Exit(1)
		}

		// this line should not be reaches
		fmt.Println("this shouldn't print out")
	} else {
		// in parent process
		var ws syscall.WaitStatus
		syscall.Wait4(int(pid), &ws, 0, nil)
		fmt.Printf("parent of %d (pid:%d)\n", pid, os.Getpid())
	}

	alternate()
}
