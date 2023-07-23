const MAX_STACK_SIZE: u16 = 1024;

#[derive(Debug)]
pub struct Stack {
    items: u16,
    data: Vec<isize>
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: 0,
            data: vec![],
        }
    }

    pub fn pop(&mut self) -> isize {
        if self.items == 0 {
            panic!("Stack underflow!");
        }
        self.items -= 1;
        self.data.pop().unwrap()
    }

    pub fn push(&mut self, val: isize) {
        if self.items == MAX_STACK_SIZE {
            panic!("Stack overflow!");
        }
        self.data.push(val);
        self.items += 1;
    }

    pub fn get(&self, pos: usize) -> &isize {
        self.data.get(pos).unwrap()
    }
}