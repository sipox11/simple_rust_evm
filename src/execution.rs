use std::{collections::HashMap, usize};
use super::stack::Stack;
use super::memory::Memory;
use cast::{isize, usize};

pub struct Context {
    pc: usize,
    code: Vec<u8>,
    finished: bool,
    instruction_set: HashMap<u8, Opcode>,
    stack: Stack,
    memory: Memory,
    success: bool,
    return_data: usize,
}

#[derive(Debug)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    POP,
    MLOAD,
    MSTORE,
    PUSH0,
    PUSH1,
    DUP1,
    DUP2,
    RETURN,
}

impl Context {
    // Create new execution context instance from bytecode as a vector of bytes
    // Initialize program counter to zero
    pub fn new(code: Vec<u8>) -> Self {
        load_instructions();
        Self {
            pc: 0,
            code,
            finished: false,
            instruction_set: load_instructions(),
            stack: Stack::new(),
            memory: Memory::new(),
            return_data: 0,
            success: false,
        }
    }

    /// Getter to inspect execution result (success and return data)
    pub fn result(&self) -> (bool, usize) {
        (self.success, self.return_data)
    }

    // Execute loaded bytecode until the end
    pub fn execute(&mut self) {
        loop {
            // Read and process next opcode
            self.execute_opcode();
            // Increase PC
            self.pc += 1;
            if self.pc >= self.code.len() {
                self.finished = true;
            }
            if self.finished {
                break;
            }
        }
    }

    pub fn execute_opcode(&mut self) {
        let opcode = match self.code.get(self.pc) {
            Some(opcode) => {
                match self.instruction_set.get(opcode) {
                    Some(opc) => opc,
                    None => &Opcode::STOP
                }
            },
            None => &Opcode::STOP,
        };
        // Print opcode information
        println!("{:?} @ pc={}", opcode, self.pc);
        println!("Pre-execution: {:?}", self.stack);
        println!("Pre-execution: {:?}", self.memory);
        match opcode {
            // Halt execution
            Opcode::STOP => {
                self.stop();
            },
            // Extract top two values from the stack, add them and push result to the stack
            Opcode::ADD => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push((a + b) % isize::MAX);
            },
            // Extract top two values from the stack, multiply them and push result to the stack
            Opcode::MUL => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push((a * b) % isize::MAX);
            },
            // Extract top two values from the stack, subtract them and push result to the stack. Panic on underflow
            Opcode::SUB => {
                let a = self.stack.pop();
                let b = self.stack.pop();
                self.stack.push(a - b);
            }
            // Extract top two values from the stack, divide them (int division) and push result to the stack
            Opcode::DIV => println!("DIV"),
            // Pop last element of the stack
            Opcode::POP => {
                self.stack.pop();
            },
            // Loads value from specific offset onto the stack. The offset is read from the top of the stack
            Opcode::MLOAD => {
                // Read offset from the stack
                let offset = self.stack.pop();
                // Read that offset from memory
                let data = self.memory.load(usize(offset).unwrap());
                // Push the read value on top of the stack
                self.stack.push(data);
            },
            // Loads value from specific offset onto the stack
            Opcode::MSTORE => {
                // Read offset from the stack
                let offset = usize(self.stack.pop()).unwrap();
                // Read value from the stack
                let val = self.stack.pop();
                // Store val on specific offset within memory
                self.memory.store_at(offset, val);
            },
            // Push value 0 to the stack
            Opcode::PUSH0 => {
                self.stack.push(0);
            },
            // Push 1 byte of value on the stack
            Opcode::PUSH1 => {
                // Increase PC in 1 byte because we're reading one byte of data
                self.pc += 1;
                match self.code.get(self.pc) {
                    Some(data) => self.stack.push(isize(*data)),
                    None => panic!("No more data to read, corrupt bytecode!"),
                }
            },
            // Duplicate first stack item on top of the stack
            Opcode::DUP1 => {
                let item = self.stack.get(0);
                self.stack.push((*item).clone());
            },
            // Duplicate second stack item on top of the stack
            Opcode::DUP2 => {
                let item = self.stack.get(1);
                self.stack.push((*item).clone());
            },
            // Return: Halt execution and return data
            Opcode::RETURN => {
                // Get starting offset from stack
                let offset = usize(self.stack.pop()).unwrap();
                // Get size from stack
                let val = self.stack.pop();
                // Read data and return it
                let data = usize(self.memory.load(offset)).unwrap_or(0); // TODO: We're ignoring sizes for now
                self.return_data = data;
                self.success = true;

            }
            _ => {
                println!("Unknown opcode, halting!");
                self.stop();
            },
        }
        println!("Post-execution: {:?}", self.stack);
        println!("Post-execution: {:?}", self.memory);

    }

    fn stop(&mut self) {
        self.finished = true;
    }
}


// Load up all implemented instructions associated by key with the opcode number in hex
fn load_instructions() -> HashMap<u8, Opcode> {
    let mut instruction_set: HashMap<u8, Opcode> = HashMap::new();
    // STOP
    instruction_set.insert(0x00, Opcode::STOP);
    // ADD
    instruction_set.insert(0x01, Opcode::ADD);
    // MUL
    instruction_set.insert(0x02, Opcode::MUL);
    // SUB
    instruction_set.insert(0x03, Opcode::SUB);
    // DIV
    instruction_set.insert(0x04, Opcode::DIV);
    // POP
    instruction_set.insert(0x50, Opcode::POP);
    // MLOAD
    instruction_set.insert(0x51, Opcode::MLOAD);
    // MSTORE
    instruction_set.insert(0x52, Opcode::MSTORE);
    // PUSH0
    instruction_set.insert(0x5F, Opcode::PUSH0);
    // PUSH1
    instruction_set.insert(0x60, Opcode::PUSH1);
    // DUP1
    instruction_set.insert(0x80, Opcode::DUP1);
    // DUP2
    instruction_set.insert(0x81, Opcode::DUP2);
    // RETURN
    instruction_set.insert(0xf3, Opcode::RETURN);
    // Return full set
    instruction_set
}