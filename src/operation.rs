#[derive(Clone)]
pub struct Operation {
    pub input_turn: usize,
    pub output_turn: usize,
    pub print: bool,
}

impl Operation {
    pub fn new(input_turn: usize, output_turn: usize, print: bool) -> Self {
        Operation {
            input_turn,
            output_turn,
            print,
        }
    }
}
