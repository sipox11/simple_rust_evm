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