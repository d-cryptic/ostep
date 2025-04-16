package main

import (
	"flag"
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"strings"
)

func RandomSeed(seed int64) {
	rand.Seed(seed)
}

// scheduler/process switch behaviors
const (
	SchedSwitchOnIO  = "SWITCH_ON_IO"
	SchedSwitchOnEnd = "SWITCH_ON_END"
)

// io finished behaviors
const (
	IORunLater     = "IO_RUN_LATER"
	IORUNImmediate = "IO_RUN_IMMEDIATE"
)

// process states
const (
	StateRunning = "RUNNING"
	StateReady   = "READY"
	StateDone    = "DONE"
	StateBlocked = "BLOCKED"
)

// members of process structures
const (
	ProcCode  = "code_"
	ProcPC    = "pc_"
	ProdID    = "pid_"
	ProcState = "proc_state_"
)

// things a process can do - instruction types
const (
	DoCompute = "cpu"
	DoIO      = "io"
	DoIODone  = "io_done"
)

// Process represents a simulated process
type Process struct {
	Code   []string // sequence of instructions e.g. "cpu", "io"
	PC     int      // Program counter to track the current instruction
	PID    int      // process ID
	State  string   // current state of the process
	IOTime int      // Remaining IO time if the process is blocked
}

// Scheduler: Process Scheduler
type Scheduler struct {
	currProc              int
	procInfo              map[int]*Process
	processSwitchBehavior string
	ioDoneBehavior        string
	ioLength              int
	ioFinishTimes         map[int][]int
}

// NewScheduler initializes and returns a new Scheduler
func NewScheduler(processSwitchBehavior, ioDoneBehavior string, ioLength int) *Scheduler {
	return &Scheduler{
		currProc:              -1,
		procInfo:              make(map[int]*Process),
		processSwitchBehavior: processSwitchBehavior,
		ioDoneBehavior:        ioDoneBehavior,
		ioLength:              ioLength,
		ioFinishTimes:         make(map[int][]int),
	}
}

// NewProcess creates a new process and adds it to the scheduler
func (s *Scheduler) NewProcess() int {
	pid := len(s.procInfo)
	s.procInfo[pid] = &Process{
		PID:   pid,
		PC:    0,
		Code:  []string{},
		State: StateReady,
	}
	return pid
}

// LoadProgram parses and loads a program into a new process
// program string format: "c7,i,c5,i"
func (s *Scheduler) LoadProgram(program string) error {
	pid := s.NewProcess()
	instructions := strings.Split(program, ",")
	for _, instr := range instructions {
		if len(instr) == 0 {
			continue
		}

		opcode := instr[0]
		switch opcode {
		case 'c':
			num, err := strconv.Atoi(instr[1:])
			if err != nil {
				return fmt.Errorf("invalid compute instructions: %s", instr)
			}

			for i := 0; i < num; i++ {
				s.procInfo[pid].Code = append(s.procInfo[pid].Code, DoCompute)
			}

		case 'i':
			s.procInfo[pid].Code = append(s.procInfo[pid].Code, DoIO, DoIODone)

		default:
			return fmt.Errorf("bad opcode %s (should be 'c' or 'i')", string(opcode))
		}
	}

	return nil
}

// Load parses and loads a program description into a new process
// The programDescription string is in the format: "X:Y"
func (s *Scheduler) Load(programDescription string) error {
	pid := s.NewProcess()
	tmp := strings.Split(programDescription, ":")

	if len(tmp) != 2 {
		return fmt.Errorf("bad description (%s): must be number <x:y>\n where X is number of instructions\n and Y is the percent change that an instruction is CPU not IO", tmp)
	}

	numInstructions, err := strconv.Atoi(tmp[0])
	if err != nil {
		return fmt.Errorf("invalid number of instructions: %v", err)
	}

	chanceCPU, err := strconv.ParseFloat(tmp[1], 64)
	if err != nil {
		return fmt.Errorf("invalid CPU scheme: %v", err)
	}

	chanceCPU /= 100.0

	for i := 0; i < numInstructions; i++ {
		if rand.Float64() < chanceCPU {
			s.procInfo[pid].Code = append(s.procInfo[pid].Code, DoCompute)
		} else {
			s.procInfo[pid].Code = append(s.procInfo[pid].Code, DoIO, DoIODone)
		}
	}

	return nil
}

// MoveToReady -> Transition to READY state
func (s *Scheduler) MoveToReady(expected string, pid int) error {
	if pid == -1 {
		pid = s.currProc
	}

	proc, exists := s.procInfo[pid]
	if !exists {
		return fmt.Errorf("process with PID %d does not exist", pid)
	}

	if proc.State != expected {
		return fmt.Errorf("expected state %s, but got %s", expected, proc.State)
	}

	proc.State = StateReady

	return nil
}

// MoveToWait transitions a process to BLOCKED State
func (s *Scheduler) MoveToWait(expected string) error {
	if s.currProc == -1 {
		return fmt.Errorf("no current process set")
	}

	proc, exists := s.procInfo[s.currProc]
	if !exists {
		return fmt.Errorf("process with PID %d does not exist", s.currProc)
	}

	if proc.State != expected {
		return fmt.Errorf("expected state %s, but got %s", expected, proc.State)
	}

	proc.State = StateBlocked

	return nil
}

// MoveToRunning transitions the current process to the RUNNING state
func (s *Scheduler) MoveToRunning(expected string) error {
	if s.currProc == -1 {
		return fmt.Errorf("no current process set")
	}

	proc, exists := s.procInfo[s.currProc]
	if !exists {
		return fmt.Errorf("process with PID %d does not exist", s.currProc)
	}

	if proc.State != expected {
		return fmt.Errorf("expected state %s, but got %s", expected, proc.State)
	}

	proc.State = StateRunning

	return nil
}

// MoveToDone transitions the current process to DONE state
func (s *Scheduler) MoveToDone(expected string) error {
	if s.currProc == -1 {
		return fmt.Errorf("no current process set")
	}

	proc, exists := s.procInfo[s.currProc]
	if !exists {
		return fmt.Errorf("process with PID %d does not exist", s.currProc)
	}

	if proc.State != expected {
		return fmt.Errorf("expected state %s, but got %s", expected, proc.State)
	}

	proc.State = StateDone
	return nil
}

// NextProc select next process to run
func (s *Scheduler) NextProc(pid int) error {
	if pid != -1 {
		s.currProc = pid
		return s.MoveToRunning(StateReady)
	}

	totalProcs := len(s.procInfo)
	for i := s.currProc + 1; i < totalProcs; i++ {
		if s.procInfo[i].State == StateReady {
			s.currProc = i
			return s.MoveToRunning(StateReady)
		}
	}

	for i := 0; i <= s.currProc; i++ {
		if s.procInfo[i].State == StateReady {
			s.currProc = i
			return s.MoveToRunning(StateReady)
		}
	}

	return nil
}

// GetNumProcesses returns total number of processes
func (s *Scheduler) GetNumProcesses() int {
	return len(s.procInfo)
}

// GetNumInstructions returns total number of instructions for a given process
func (s *Scheduler) GetNumInstructions(pid int) int {
	proc, exists := s.procInfo[pid]
	if !exists {
		return 0
	}

	return len(proc.Code)
}

// GetInstruction return instruction at a given index for a process
func (s *Scheduler) GetInstruction(pid, index int) string {
	proc, exists := s.procInfo[pid]
	if !exists {
		return fmt.Sprintf("process with PID %d does not exist", pid)
	}

	if index < 0 || index >= len(proc.Code) {
		return fmt.Sprintf("instruction index %d out of bounds for process %d", index, pid)
	}
	return proc.Code[index]
}

// GetNumActive returns number of processes which is in active state
func (s *Scheduler) GetNumActive() int {
	count := 0
	for _, proc := range s.procInfo {
		if proc.State != StateDone {
			count++
		}
	}

	return count
}

// GetNumRunnable returns number of processes that are in ready||Running state
func (s *Scheduler) GetNumRunnable() int {
	count := 0
	for _, proc := range s.procInfo {
		if proc.State == StateReady || proc.State == StateRunning {
			count++
		}
	}

	return count
}

// Switch prints a specified number of columns with fixed width
func (s *Scheduler) Space(numColumns int) {
	for i := 0; i < numColumns; i++ {
		fmt.Printf("%10s", " ")
	}
}

// CheckIfDone checks if current process has completed its instructions
func (s *Scheduler) CheckIfDone() error {
	if s.currProc == -1 {
		return fmt.Errorf("no current process set")
	}

	proc, exists := s.procInfo[s.currProc]
	if !exists {
		return fmt.Errorf("process with PID %d does not exist", s.currProc)
	}

	if len(proc.Code) == 0 {
		if proc.State == StateRunning {
			proc.State = StateDone
			return s.NextProc(-1)
		}
	}

	return nil
}

func (s *Scheduler) GetIOSInFlight(currentTime int) int {
	count := 0
	for _, finishTimes := range s.ioFinishTimes {
		for _, t := range finishTimes {
			if t > currentTime {
				count++
			}
		}
	}

	return count
}

// Run executes the simulation of CPU scheduler
func (s *Scheduler) Run() (cpuBusy, ioBusy, clockTick int) {
	if len(s.procInfo) == 0 {
		return
	}

	// track outstanding ios per process
	s.ioFinishTimes = make(map[int][]int)
	for pid := range s.procInfo {
		s.ioFinishTimes[pid] = []int{}
	}

	// make first process active
	s.currProc = 0
	_ = s.MoveToRunning(StateReady)

	// output headers for each column
	fmt.Printf("%-5s", "Time")
	for pid := range s.procInfo {
		fmt.Printf("%14s", fmt.Sprintf("PID:%2d", pid))
	}
	fmt.Printf("%14s%14s\n", "CPU", "IOs")

	cpuBusy = 0
	ioBusy = 0
	clockTick = 0

	for s.GetNumActive() > 0 {
		clockTick++

		// check for io iofinish
		ioDone := false
		for pid := range s.procInfo {
			for _, t := range s.ioFinishTimes[pid] {
				if t == clockTick {
					ioDone = true
					_ = s.MoveToReady(StateBlocked, pid)

					if s.ioDoneBehavior == "IO_RUN_IMMEDIATE" {
						if s.currProc != pid && s.procInfo[s.currProc].State == StateRunning {
							_ = s.MoveToReady(StateRunning, s.currProc)
						}
						_ = s.NextProc(pid)
					} else {
						if s.processSwitchBehavior == "SWITCH_ON_END" && s.GetNumRunnable() > 1 {
							_ = s.NextProc(pid)
						}

						if s.GetNumRunnable() == 1 {
							_ = s.NextProc(pid)
						}
					}

					_ = s.CheckIfDone()
				}
			}
		}

		// if current proc is RUNNING and has an instruction, execute it
		instructionToExecute := ""
		if s.procInfo[s.currProc].State == StateRunning && len(s.procInfo[s.currProc].Code) > 0 {
			instructionToExecute = s.procInfo[s.currProc].Code[0]
			s.procInfo[s.currProc].Code = s.procInfo[s.currProc].Code[1:]
			cpuBusy++
		}

		// output current state
		if ioDone {
			fmt.Printf("%3d*", clockTick)
		} else {
			fmt.Printf("%3d", clockTick)
		}

		for pid := range s.procInfo {
			if pid == s.currProc && instructionToExecute != "" {
				fmt.Printf("%14s", "RUN:"+instructionToExecute)
			} else {
				fmt.Printf("%14s", s.procInfo[pid].State)
			}
		}

    if instructionToExecute == "" {
      fmt.Printf("%14s", " ")
    } else {
      fmt.Printf("%14s", "1")
    }

		// io output
		numOutStanding := s.GetIOSInFlight(clockTick)
		if numOutStanding > 0 {
			fmt.Printf("%14d\n", numOutStanding)
			ioBusy++
		} else {
			fmt.Printf("%14s\n", " ")
		}

		// handle io instruction
		if instructionToExecute == DoIO {
			_ = s.MoveToWait(StateRunning)
			s.ioFinishTimes[s.currProc] = append(s.ioFinishTimes[s.currProc], clockTick+s.ioLength+1)
			if s.processSwitchBehavior == "SWITCH_ON_IO" {
				_ = s.NextProc(-1)
			}
		}

		// Check if current process is done
		_ = s.CheckIfDone()

	}

	return cpuBusy, ioBusy, clockTick
}

func main() {
	// Define command-line flags
	seed := flag.Int("s", 0, "the random seed")
	program := flag.String("P", "", "more specific controls over programs")
	processList := flag.String("l", "", "a comma-separated list of processes to run, in the form X1:Y1,X2:Y2,... where X is the number of instructions that process should run, and Y the chances (from 0 to 100) that an instruction will use the CPU or issue an IO")
	ioLength := flag.Int("L", 5, "how long an IO takes")
	processSwitchBehavior := flag.String("S", SchedSwitchOnIO, "when to switch between processes: SWITCH_ON_IO, SWITCH_ON_END")
	ioDoneBehavior := flag.String("I", IORunLater, "type of behavior when IO ends: IO_RUN_LATER, IO_RUN_IMMEDIATE")
	solve := flag.Bool("c", false, "compute answers for me")
	printStats := flag.Bool("p", false, "print statistics at end; only useful with -c flag (otherwise stats are not printed)")

	// Parse the flags
	flag.Parse()

	// Validate Process switch behavior
	if *processSwitchBehavior != SchedSwitchOnIO && *processSwitchBehavior != SchedSwitchOnEnd {
		fmt.Fprintf(os.Stderr, "Invalid process switch behavior: %s\n", *processSwitchBehavior)
		os.Exit(1)
	}

	// Validate I/O done behavior
	if *ioDoneBehavior != IORUNImmediate && *ioDoneBehavior != IORunLater {
		fmt.Fprintf(os.Stderr, "Invalid I/O done behavior: %s\n", *ioDoneBehavior)
		os.Exit(1)
	}

	// Set the random seed
	rand.Seed(int64(*seed))

	// initialize the scheduler
	s := NewScheduler(*processSwitchBehavior, *ioDoneBehavior, *ioLength)

	// load processes from the process list
	if *program != "" {
		programs := strings.Split(*program, ",")
		for _, p := range programs {
			err := s.LoadProgram(p)
			if err != nil {
				fmt.Fprintf(os.Stderr, "Error loading program: %v\n", err)
				os.Exit(1)
			}
		}
	} else {
		for _, p := range strings.Split(*processList, ",") {
			if err := s.Load(p); err != nil {
				fmt.Fprintf(os.Stderr, "Error loading process: %v\n", err)
				os.Exit(1)
			}
		}
	}

	// validate io length
	if *ioLength < 0 {
		fmt.Fprintln(os.Stderr, "I/O length must be non-negative")
		os.Exit(1)
	}

	// if solve flag is not set, print process traces
	if !*solve {
		fmt.Println("Produce a trace of what would happen when you run these processes:")
		for pid := 0; pid < s.GetNumProcesses(); pid++ {
			fmt.Printf("Process %d\n", pid)
			for inst := 0; inst < s.GetNumInstructions(pid); inst++ {
				fmt.Printf("  %s\n", s.GetInstruction(pid, inst))
			}
			fmt.Println()
		}

		fmt.Print("Important behaviors:\n  System will switch when ")
		if *processSwitchBehavior == SchedSwitchOnIO {
			fmt.Println("the current process is FINISHED or ISSUES AN IO")
		} else {
			fmt.Println("the current process is FINISHED")
		}

		fmt.Print("  After IOs, the process issuing the IO will ")
		if *ioDoneBehavior == IORUNImmediate {
			fmt.Println("run IMMEDIATELY")
		} else {
			fmt.Println("run LATER (when it is its turn)")
		}
		fmt.Println()
		os.Exit(0)
	}

	// Run the scheduler
	cpuBusy, ioBusy, clockTick := s.Run()

	// Print statistics if requested
	if *printStats {
		fmt.Println()
		fmt.Printf("Stats: Total Time %d\n", clockTick)
		if clockTick > 0 {
			fmt.Printf("Stats: CPU Busy %d (%.2f%%)\n", cpuBusy, 100.0*float64(cpuBusy)/float64(clockTick))
			fmt.Printf("Stats: IO Busy  %d (%.2f%%)\n", ioBusy, 100.0*float64(ioBusy)/float64(clockTick))
		} else {
			fmt.Println("Stats: CPU Busy 0 (0.00%)")
			fmt.Println("Stats: IO Busy  0 (0.00%)")
		}
		fmt.Println()
	}

}
