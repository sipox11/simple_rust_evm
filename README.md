# Simple EVM in Rust

## Initial version

- Only stack, no memory and no storage.
- No return data, only trace printing to inspect the stack.
- Opcodes implemented: 
  - 0x00 (STOP). Halts execution.
  - 0x01 (ADD). Stack input a, b. Stack output a+b modulo 2e256.
  - 0x02 (MUL). Stack input a, b. Stack output a*b modulo 2e256.
  - 0x03 (SUB). Stack input a, b. Stack output a-b.
  - 0x04 (DIV). Stack input a, b. Stack output a/b. Panic if b == 0
  - ...
  - 0x50 (POP). Remove item from the stack
  - ...
  - 0x5F (PUSH0). Pushes value 0 on the stack.
  - 0x60 (PUSH1). Pushes 1 byte on the stack (8 bits).
  - ...
  - 0x80 (DUP1). Duplicates first item of the stack, placing it on top of the stack.
  - 0x80 (DUP2). Duplicates second item of the stack, placing it on top of the stack.

Only basic arithmetic operators involving the stack, only 1 byte operations with the stack.

Execution bytecode will be hard-coded on the main entrypoint and it will get executed. 

The main entry point will loop through the bytecode, one byte at a time. Every opcode gets processed, potentially needing to fetch another byte if the opcode needs params. Execution stops either when STOP is hit, or when there are no more opcodes to run. 

How to run: 

`cargo build && cargo run`

Output (for bytecode `602560056089602C01020300`): 

```
PUSH1 @ pc=0
Pre-execution: Stack { items: 0, data: [] }
Post-execution: Stack { items: 1, data: [37] }
PUSH1 @ pc=2
Pre-execution: Stack { items: 1, data: [37] }
Post-execution: Stack { items: 2, data: [37, 5] }
PUSH1 @ pc=4
Pre-execution: Stack { items: 2, data: [37, 5] }
Post-execution: Stack { items: 3, data: [37, 5, 137] }
PUSH1 @ pc=6
Pre-execution: Stack { items: 3, data: [37, 5, 137] }
Post-execution: Stack { items: 4, data: [37, 5, 137, 44] }
ADD @ pc=8
Pre-execution: Stack { items: 4, data: [37, 5, 137, 44] }
Post-execution: Stack { items: 3, data: [37, 5, 181] }
MUL @ pc=9
Pre-execution: Stack { items: 3, data: [37, 5, 181] }
Post-execution: Stack { items: 2, data: [37, 905] }
SUB @ pc=10
Pre-execution: Stack { items: 2, data: [37, 905] }
Post-execution: Stack { items: 1, data: [868] }
STOP @ pc=11
Pre-execution: Stack { items: 1, data: [868] }
Post-execution: Stack { items: 1, data: [868] }
```

## V2 - Adding memory and RETURN

- Only stack and memory, no storage.
- Add return data via memory, trace printing to inspect the stack and memory
- Opcodes implemented: 
  - 0x00 (STOP). Halts execution.
  - 0x01 (ADD). Stack input a, b. Stack output a+b modulo 2e256.
  - 0x02 (MUL). Stack input a, b. Stack output a*b modulo 2e256.
  - 0x03 (SUB). Stack input a, b. Stack output a-b.
  - 0x04 (DIV). Stack input a, b. Stack output a/b. Panic if b == 0
  - ...
  - 0x50 (POP). Remove item from the stack
  - 0x51 (MLOAD). Load 1 word from memory (32 bytes)
  - 0x52 (MSTORE). Write 1 word to memory (32 bytes)
  - ...
  - 0x5F (PUSH0). Pushes value 0 on the stack.
  - 0x60 (PUSH1). Pushes 1 byte on the stack (8 bits).
  - ...
  - 0x80 (DUP1). Duplicates first item of the stack, placing it on top of the stack.
  - 0x80 (DUP2). Duplicates second item of the stack, placing it on top of the stack.
  - ...
  - 0xF3 (RETURN). Halts execution and returns output data

Only basic arithmetic operators involving the stack and memory.

Execution bytecode will be hard-coded on the main entrypoint and it will get executed. 

The main entry point will loop through the bytecode, one byte at a time. Every opcode gets processed, potentially needing to fetch more bytes if the opcode needs params. Execution stops either when STOP is hit, or when there are no more opcodes to run. 

How to run: 

`cargo build && cargo run`

Output (for bytecode `602560056089602C01020360005260206000f3`): 

It is expected to store the result of the operation both at the top of the stack, on memory and return the result from execution.

```
PUSH1 @ pc=0
Pre-execution: Stack { items: 0, data: [] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 1, data: [37] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
PUSH1 @ pc=2
Pre-execution: Stack { items: 1, data: [37] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 2, data: [37, 5] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
PUSH1 @ pc=4
Pre-execution: Stack { items: 2, data: [37, 5] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 3, data: [37, 5, 137] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
PUSH1 @ pc=6
Pre-execution: Stack { items: 3, data: [37, 5, 137] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 4, data: [37, 5, 137, 44] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
ADD @ pc=8
Pre-execution: Stack { items: 4, data: [37, 5, 137, 44] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 3, data: [37, 5, 181] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
MUL @ pc=9
Pre-execution: Stack { items: 3, data: [37, 5, 181] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 2, data: [37, 905] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
SUB @ pc=10
Pre-execution: Stack { items: 2, data: [37, 905] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 1, data: [868] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
PUSH1 @ pc=11
Pre-execution: Stack { items: 1, data: [868] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 2, data: [868, 0] }
Post-execution: Memory { data: {64: 96}, fmp: 96 }
MSTORE @ pc=13
Pre-execution: Stack { items: 2, data: [868, 0] }
Pre-execution: Memory { data: {64: 96}, fmp: 96 }
Post-execution: Stack { items: 0, data: [] }
Post-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
PUSH1 @ pc=14
Pre-execution: Stack { items: 0, data: [] }
Pre-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
Post-execution: Stack { items: 1, data: [32] }
Post-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
PUSH1 @ pc=16
Pre-execution: Stack { items: 1, data: [32] }
Pre-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
Post-execution: Stack { items: 2, data: [32, 0] }
Post-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
RETURN @ pc=18
Pre-execution: Stack { items: 2, data: [32, 0] }
Pre-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
Post-execution: Stack { items: 0, data: [] }
Post-execution: Memory { data: {0: 868, 64: 96}, fmp: 96 }
===================================================
Successful execution! Return value: 868
===================================================
```