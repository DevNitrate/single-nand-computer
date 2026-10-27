use crate::{nand::NandComputer, operation::Operation};

mod nand;
mod operation;

/*

    Have an array, at each timestep the NAND gate can choose inputs from any address in the array and output to any address. IRL: have the memory be on a wheel with by default input A touching address 0, input B address 1 and the output address 2. A, B and output always keep this relative spacing. At each operation the wheel can turn n addresses and uses the input to compute the NAND and then the wheel turn m addresses and puts the output inside the cell. Note the inputs have to be next to each other.

*/

const WHEEL_SIZE: usize = 10;

// A NOTB 1 0

fn main() {
    let mut program: Vec<Operation> = Vec::new();
    program.push(Operation::new(0, 0, false)); // Put A NAND B at adress 2
    program.push(Operation::new(0, 7, false)); // Put A NAND B at adress 9
    program.push(Operation::new(4, 0, false)); // Put B NAND adress 2 at adress 3
    program.push(Operation::new(8, 3, false)); // Put A NAND adress 9 at adress 4
    program.push(Operation::new(1, 0, true)); // Put A XOR 5 at adress 5
    program.push(Operation::new(7, 4, false)); // Put A NAND B at adress 6
    program.push(Operation::new(2, 0, true)); // Put A AND B at address 8

    for i in 0..=1 {
        for j in 0..=1 {
            let mut memory_wheel: [bool; WHEEL_SIZE] = [true; WHEEL_SIZE];
            memory_wheel[0] = i != 0;
            memory_wheel[1] = j != 0;
            println!("A: {} ; B: {}", i, j);
            let mut computer: NandComputer = NandComputer::new(program.clone(), memory_wheel);
            computer.run();
        }
    }
}
