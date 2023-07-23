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

Output (for bytecode `6089602C016005026025800300`): 

```
PUSH1 @ pc=0
Pre-execution: Stack { items: 0, data: [] }
Post-execution: Stack { items: 1, data: [137] }
PUSH1 @ pc=2
Pre-execution: Stack { items: 1, data: [137] }
Post-execution: Stack { items: 2, data: [137, 44] }
ADD @ pc=4
Pre-execution: Stack { items: 2, data: [137, 44] }
Post-execution: Stack { items: 1, data: [181] }
PUSH1 @ pc=5
Pre-execution: Stack { items: 1, data: [181] }
Post-execution: Stack { items: 2, data: [181, 5] }
MUL @ pc=7
Pre-execution: Stack { items: 2, data: [181, 5] }
Post-execution: Stack { items: 1, data: [905] }
PUSH1 @ pc=8
Pre-execution: Stack { items: 1, data: [905] }
Post-execution: Stack { items: 2, data: [905, 37] }
DUP1 @ pc=10
Pre-execution: Stack { items: 2, data: [905, 37] }
Post-execution: Stack { items: 3, data: [905, 37, 905] }
SUB @ pc=11
Pre-execution: Stack { items: 3, data: [905, 37, 905] }
Post-execution: Stack { items: 2, data: [905, 868] }
STOP @ pc=12
Pre-execution: Stack { items: 2, data: [905, 868] }
Post-execution: Stack { items: 2, data: [905, 868] }
```