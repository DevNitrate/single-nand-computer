use crate::{WHEEL_SIZE, operation::Operation};

pub struct NandComputer {
    program: Vec<Operation>,
    memory_wheel: [bool; WHEEL_SIZE],
    pointer: usize,
}

impl NandComputer {
    pub fn new(program: Vec<Operation>, memory_wheel: [bool; WHEEL_SIZE]) -> Self {
        NandComputer {
            program,
            memory_wheel,
            pointer: 0,
        }
    }

    pub fn run(&mut self) {
        for op in &self.program {
            self.pointer = (self.pointer + op.input_turn) % WHEEL_SIZE;
            let a: bool = self.memory_wheel[self.pointer];
            let b: bool = self.memory_wheel[(self.pointer + 1) % WHEEL_SIZE];
            let output: bool = !(a & b);

            if op.print {
                println!("{}", output as u8)
            }

            self.pointer = (self.pointer + op.output_turn) % WHEEL_SIZE;
            self.memory_wheel[(self.pointer + 2) % WHEEL_SIZE] = output;
        }
    }
}
