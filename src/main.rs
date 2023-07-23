use hex;

pub mod execution;
pub mod stack;
pub mod memory;

fn main() {
    // let code: String = String::from("6006600702"); // Calculate 6x7 and leave result on the top of the stack
    // let code: String = String::from("602560056089602C01020300"); // Calculate (5*(137+44) - 37) and leave result on the top of the stack. Expected result 868
    let code: String = String::from("602560056089602C01020360005360016000f3"); // Calculate (5*(137+44) - 37) and return result from memory (offset 0). Expected result 868
    let bytecode: Vec<u8> = hex::decode(code).expect("Invalid bytecode!");
    // Display bytecode as a list of opcodes in hex
    for opcode in &bytecode {
        println!("0x{:x}", opcode);
    }
    let mut execution_ctx = execution::Context::new(bytecode); // bytecode is moved out of scope
    // Execute bytecode
    execution_ctx.execute();
}

