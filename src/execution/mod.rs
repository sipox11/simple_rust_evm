pub struct Context {
    pc: usize,
    code: String,
}

impl Context {
    pub fn new(code: String) -> Self {
        Self {
            pc: 0,
            code,
        }
    }

    pub fn debug(&self) {
        let code = &self.code;
        let pc = self.pc;
        println!("== Execution context ==");
        println!("pc = {pc}");
        println!("code = [{code}]");
        println!("Stack: []");
        println!("Memory: []");
        println!("=======================");

    }

    // pub fn read_code
}

pub fn test() {
    println!("Test!");
}