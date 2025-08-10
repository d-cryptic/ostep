# xv6 Context Switch Implementation

## What

This is the low-level assembly code from MIT's xv6 operating system that implements context switching between processes. The `swtch` function saves the current process's CPU registers and loads another process's registers, enabling the OS to switch between running processes.

## Why

Context switching is fundamental to multitasking operating systems. When the OS scheduler decides to run a different process, it must:
1. Save the current process's CPU state (registers, stack pointer, instruction pointer)
2. Load the new process's previously saved CPU state
3. Jump to where the new process left off

This low-level mechanism enables time-sharing and process isolation.

## Purpose

- **Process Scheduling**: Enable the kernel to switch between processes
- **Multitasking**: Allow multiple processes to share CPU time
- **System Calls**: Return control to user processes after kernel operations
- **Interrupt Handling**: Resume processes after handling interrupts

## Code Explanation

### Function Signature (`swtch.s:1`)
```assembly
void swtch(struct context *old, struct context *new);
```
Takes pointers to two context structures - one to save current state, one to load new state.

### Context Structure Layout
Based on the assembly offsets, the `struct context` contains:
```c
struct context {
    uint eip;    // 0(%eax)  - instruction pointer
    uint esp;    // 4(%eax)  - stack pointer  
    uint ebx;    // 8(%eax)  - general purpose registers
    uint ecx;    // 12(%eax)
    uint edx;    // 16(%eax)
    uint esi;    // 20(%eax)
    uint edi;    // 24(%eax)
    uint ebp;    // 28(%eax) - base pointer
};
```

### Save Phase (`swtch.s:7-16`)
```assembly
movl 4(%esp), %eax    # Load 'old' context pointer
popl 0(%eax)          # Save return address (EIP)
movl %esp, 4(%eax)    # Save stack pointer
movl %ebx, 8(%eax)    # Save general purpose registers
movl %ecx, 12(%eax)
movl %edx, 16(%eax)
movl %esi, 20(%eax)
movl %edi, 24(%eax)
movl %ebp, 28(%eax)
```
**Key insight**: `popl 0(%eax)` removes the return address from stack and saves it as the instruction pointer to resume at.

### Restore Phase (`swtch.s:18-28`)
```assembly
movl 4(%esp), %eax    # Load 'new' context pointer  
movl 28(%eax), %ebp   # Restore base pointer
movl 24(%eax), %edi   # Restore general purpose registers
movl 20(%eax), %esi
movl 16(%eax), %edx
movl 12(%eax), %ecx
movl 8(%eax), %ebx
movl 4(%eax), %esp    # Switch to new stack
pushl 0(%eax)         # Push new return address onto stack
ret                   # Jump to new process
```
**Key insight**: After switching stacks with `movl 4(%eax), %esp`, the `ret` instruction jumps to wherever the new process was previously executing.

## Critical Details

### Stack Switching (`swtch.s:26`)
```assembly
movl 4(%eax), %esp    # stack is switched here
```
This is the moment of transition - we're now using the new process's stack.

### Return Address Handling (`swtch.s:9,27`)
```assembly
popl 0(%eax)          # Save current return address
pushl 0(%eax)         # Restore new return address
```
The return address determines where execution continues after `ret`.

### Register Preservation
All callee-saved registers (EBX, ESI, EDI, EBP) plus stack pointer are saved/restored. EAX, ECX, EDX are caller-saved so not preserved across function calls.

## Usage Context

This function is called by the xv6 scheduler:
1. Timer interrupt occurs
2. Kernel saves trap frame
3. Scheduler calls `swtch(&current->context, &next->context)`
4. Current process state saved, new process state loaded
5. Execution continues in new process

## Learning Outcomes

Understanding this code teaches:
- Low-level CPU state management
- Stack manipulation and switching
- How multitasking works at the hardware level
- The role of registers in process state
- Assembly language and system programming concepts