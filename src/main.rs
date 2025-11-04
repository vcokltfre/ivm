use std::env::args;

mod vm;

fn main() {
    // let data = vec![
    //     0x01, 0x06, 0x02, // PUSH_TYPE_INTEGER_POWER_SUB, power 2
    //     0x04,             // DUP
    //     0x12,             // MUL
    //     0x03, 0x01, 0x41, // STORE 'A'
    //     0x02, 0x01, 0x41, // LOAD 'A'
    //     0x00,             // DEBUG
    // ];
    // let data = vec![
    //     0x01, 0x05, 0x02, // PUSH_INTEGER_POWER, power 2
    //     0x04,             // DUP
    //     0x12,             // MUL
    //     0xE0, 0x01, 0x41, // LABEL 'A'
    //     0x04,             // DUP
    //     0xF0,             // DISPLAY_STDOUT
    //     0x01, 0x06, 0x01, // PUSH_INTEGER_POWER_SUB, power 1
    //     0x11,             // SUB
    //     0x04,             // DUP
    //     0x01, 0x06, 0x00, // PUSH_INTEGER_POWER_SUB, power 0
    //     0xD0, 0x02,       // CMP (not equal)
    //     0xE2, 0x01, 0x41, // JUMP (true) 'A'
    // ];

    let filename = args().nth(1).expect("No input file specified");
    let data = std::fs::read(filename).expect("Failed to read input file");

    let mut vm = vm::VM::new(data);

    vm.run(true);
    vm.run(false);
}
