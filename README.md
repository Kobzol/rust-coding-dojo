# Coding dojo: 8-bit assembly interpreter
This is a [coding dojo](https://codingdojo.org/) that was performed at the [Rust Prague September 
2024](https://www.meetup.com/rust-prague/events/303346494) meetup.

## Goal
Implement a simple assembly interpreter that can perform arithmetic, logical and output 
operations on a register set and a block of memory.

## Rules
- One person is coding (`Driver`), another one guides them (`Copilot`)
- After 6-7 minutes => `Copilot` becomes `Driver`, new `Copilot` arrives
- No interaction with audience unless `Copilot`/`Driver` asks for it
- Five/six rounds
- Remember to create commits after atomic changes (and when the driver changes)
- You can use external crates
- Don't use AI :)

## Subgoals
- Implement a CPU (**easy**)
  - Set of registers
  - Block of memory
  - Instruction pointer
- Implement `Move` instruction (**easy**)
  - First argument is memory address
  - Second argument is a constant
- Implement instruction execution (**easy**)
  - Model execution errors
- Handle reads/writes to invalid memory (**easy**)
  - Model uninitialized data in registers/memory
- Implement read and write expressions (**medium**)
  - Write expression can be anything with an address (any l-value)
  - Read expression can be anything with an address or a constant
- Add support for reading both constants and memory addresses to `Move` (**easy**)
- Implement `Add` and `Sub` instructions (**easy**)
- Add `Program` abstraction (list of instructions) and a method to execute a program (**easy**)
- Write tests (**medium**)
- CpuBuilder for initializing the CPU (**easy**)
- Add `PrintInt` instruction (**easy**)
  - Check `PrintInt` output in tests (**medium**)
- Parse instructions from a string (**hard**)
  ```nasm
  start:
    MOV R0, 3
    PRINT R0
    SUB R0, 1
    JNZ R0, start
  ```
  - Handle invalid instructions
  - Handle missing (or superfluous) arguments
  - Handle invalid register indices
- Show parsing error annotated with the line where the error has occured (**easy**)
  ```text
  > MOV X
  Line 5: missing argument for MOV
  ```
- Add instruction labels and `JumpIfNotZero` conditional jump instruction (**hard**)
  ```nasm
  start:
    MOV R0, 3
    PRINT R0
    SUB R0, 1
    JNZ R0, start
  ```

### Bonus
- Add `PrintString` instruction (**medium**)
- Implement call stack and function calls (**very hard**)
