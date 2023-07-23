pub mod execution;

fn main() {
    let bytecode = String::from("00");
    let execution_ctx = execution::Context::new(bytecode);
    execution_ctx.debug();
}
