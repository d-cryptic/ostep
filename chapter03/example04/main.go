package main

import (
	"log"
	"os"
	"os/exec"
)

func main() {
	// open or create the output file with permissions rwx------ (0700)

	outFile, err := os.OpenFile("example4.output", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0700)
	if err != nil {
		log.Fatalf("error opening output file: %v", err)
	}
	defer outFile.Close()

	// prepare the command: wc main.go
	cmd := exec.Command("wc", "main.go")

	// redirect the child's stdout to our file
	cmd.Stdout = outFile

	// propagate stderr so we see errors if wc fails
	cmd.Stderr = os.Stderr

	// run the command (this does a fork+exec under the hood)
	if err := cmd.Run(); err != nil {
		log.Fatalf("command execution failed: %v", err)
	}

	// when cmd.Run() returns, the child has exited and we have waited for it
}
