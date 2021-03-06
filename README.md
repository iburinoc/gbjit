GbJit
===

An attempt at an x86\_64 JIT gameboy (and possible gameboy colour) emulator.
The intention is to map gb instructions to some number of x86 instructions,
and then just execute from there, with occasional extra code generation
in the case of self-modifying code.

Compilation Model
---

### Register mapping
- ah is a staging area, as is [rsp]
- The LAHF representation of F is at [rsp + 0x02]
- A maps to al (this one is a bit backwards, because the LAHF puts eflags in ah)
- BC maps to bh,bl
- DE maps to ch,cl
- HL maps to dh,dl
- SP maps to r12w
- PC maps to r13w
- intenable is placed in the 16th bit of r13
- The cycle count is stored in r14
- The cycle count an interrupt will be generated at is in r15

TODO: Determine if some of those should be on the stack instead of a register

### Stack layout
0x68 bytes of stack space are allocated
Top 0x30 are used for saving rbx, rbp, r12-r15

0x00-0x02: Scratch space
0x02-0x03: LAHF format F
0x03-0x08: Storage space for A, DE, and HL, when calling external functions
0x08-0x10: CpuState address
0x10-0x18: Memory parameter
0x18-0x20: Return address for oneoff instructions
0x20-0x28: Int disabled cycle limit
0x28-0x30: Int enabled cycle limit
0x30-0x38: Currently unused

### Execution model
Each arbitrary-sized page of instructions is compiled separately, and has an entry
and exit routine.
