use std::{collections::HashMap};

const OP_DEBUG: u8 = 0x00;
const OP_PUSH: u8 = 0x01;
const OP_LOAD: u8 = 0x02;
const OP_STORE: u8 = 0x03;
const OP_DUP: u8 = 0x04;
const OP_SWAP: u8 = 0x05;
const OP_POP: u8 = 0x06;

const OP_ADD: u8 = 0x10;
const OP_SUB: u8 = 0x11;
const OP_MUL: u8 = 0x12;
const OP_DIV: u8 = 0x13;
const OP_MOD: u8 = 0x14;

const OP_STR_GET_SLICE: u8 = 0x20;
const OP_STR_LENGTH: u8 = 0x21;

const OP_CMP: u8 = 0xD0;

const OP_LABEL: u8 = 0xE0;
const OP_JUMP: u8 = 0xE1;
const OP_JUMP_IF_TRUE: u8 = 0xE2;
const OP_JUMP_IF_FALSE: u8 = 0xE3;
const OP_CALL: u8 = 0xE4;
const OP_RETURN: u8 = 0xE5;

const OP_DISPLAY_STDOUT: u8 = 0xF0;
const OP_DISPLAY_STDERR: u8 = 0xF1;
const OP_INPUT: u8 = 0xF2;
const OP_EXIT: u8 = 0xFF;

const PUSH_TYPE_INTEGER: u8 = 0x01;
const PUSH_TYPE_FLOAT: u8 = 0x02;
const PUSH_TYPE_STRING: u8 = 0x03;
const PUSH_TYPE_BOOLEAN: u8 = 0x04;
const PUSH_TYPE_INTEGER_POWER: u8 = 0x05;
const PUSH_TYPE_INTEGER_POWER_SUB: u8 = 0x06;

const CMP_TYPE_EQUAL: u8 = 0x01;
const CMP_TYPE_NOT_EQUAL: u8 = 0x02;
const CMP_TYPE_LESS_THAN: u8 = 0x03;
const CMP_TYPE_GREATER_THAN: u8 = 0x04;
const CMP_TYPE_LESS_EQUAL: u8 = 0x05;
const CMP_TYPE_GREATER_EQUAL: u8 = 0x06;

#[derive(Debug, Clone)]
enum IVMType {
    Integer { value: i64 },
    Float { value: f64 },
    String { value: String },
    Boolean { value: bool },
}

impl IVMType {
    fn compare(&self, other: &IVMType, cmp_type: u8) -> Option<bool> {
        match (self, other) {
            (IVMType::Integer { value: lhs }, IVMType::Integer { value: rhs }) => match cmp_type {
                CMP_TYPE_EQUAL => Some(lhs == rhs),
                CMP_TYPE_NOT_EQUAL => Some(lhs != rhs),
                CMP_TYPE_LESS_THAN => Some(lhs < rhs),
                CMP_TYPE_GREATER_THAN => Some(lhs > rhs),
                CMP_TYPE_LESS_EQUAL => Some(lhs <= rhs),
                CMP_TYPE_GREATER_EQUAL => Some(lhs >= rhs),
                _ => None,
            },
            (IVMType::Float { value: lhs }, IVMType::Float { value: rhs }) => match cmp_type {
                CMP_TYPE_EQUAL => Some(lhs == rhs),
                CMP_TYPE_NOT_EQUAL => Some(lhs != rhs),
                CMP_TYPE_LESS_THAN => Some(lhs < rhs),
                CMP_TYPE_GREATER_THAN => Some(lhs > rhs),
                CMP_TYPE_LESS_EQUAL => Some(lhs <= rhs),
                CMP_TYPE_GREATER_EQUAL => Some(lhs >= rhs),
                _ => None,
            },
            (IVMType::String { value: lhs }, IVMType::String { value: rhs }) => match cmp_type {
                CMP_TYPE_EQUAL => Some(lhs == rhs),
                CMP_TYPE_NOT_EQUAL => Some(lhs != rhs),
                CMP_TYPE_LESS_THAN => Some(lhs < rhs),
                CMP_TYPE_GREATER_THAN => Some(lhs > rhs),
                CMP_TYPE_LESS_EQUAL => Some(lhs <= rhs),
                CMP_TYPE_GREATER_EQUAL => Some(lhs >= rhs),
                _ => None,
            },
            (IVMType::Boolean { value: lhs }, IVMType::Boolean { value: rhs }) => match cmp_type {
                CMP_TYPE_EQUAL => Some(lhs == rhs),
                CMP_TYPE_NOT_EQUAL => Some(lhs != rhs),
                _ => None,
            },
            _ => None,
        }
    }

    fn add(&self, other: &IVMType) -> Option<IVMType> {
        match (self, other) {
            (IVMType::Integer { value: lhs }, IVMType::Integer { value: rhs }) => {
                Some(IVMType::Integer { value: lhs + rhs })
            }
            (IVMType::Float { value: lhs }, IVMType::Float { value: rhs }) => {
                Some(IVMType::Float { value: lhs + rhs })
            }
            (IVMType::String { value: lhs }, IVMType::String { value: rhs }) => {
                Some(IVMType::String {
                    value: format!("{}{}", lhs, rhs),
                })
            }
            _ => None,
        }
    }

    fn sub(&self, other: &IVMType) -> Option<IVMType> {
        match (self, other) {
            (IVMType::Integer { value: lhs }, IVMType::Integer { value: rhs }) => {
                Some(IVMType::Integer { value: lhs - rhs })
            }
            (IVMType::Float { value: lhs }, IVMType::Float { value: rhs }) => {
                Some(IVMType::Float { value: lhs - rhs })
            }
            _ => None,
        }
    }

    fn mul(&self, other: &IVMType) -> Option<IVMType> {
        match (self, other) {
            (IVMType::Integer { value: lhs }, IVMType::Integer { value: rhs }) => {
                Some(IVMType::Integer { value: lhs * rhs })
            }
            (IVMType::Float { value: lhs }, IVMType::Float { value: rhs }) => {
                Some(IVMType::Float { value: lhs * rhs })
            }
            (IVMType::String { value: lhs }, IVMType::Integer { value: rhs }) => {
                if *rhs < 0 {
                    return None;
                }
                Some(IVMType::String {
                    value: lhs.repeat(*rhs as usize),
                })
            }
            _ => None,
        }
    }

    fn div(&self, other: &IVMType) -> Option<IVMType> {
        match (self, other) {
            (IVMType::Integer { value: lhs }, IVMType::Integer { value: rhs }) => {
                if *rhs == 0 {
                    return None;
                }
                Some(IVMType::Integer { value: lhs / rhs })
            }
            (IVMType::Float { value: lhs }, IVMType::Float { value: rhs }) => {
                if *rhs == 0.0 {
                    return None;
                }
                Some(IVMType::Float { value: lhs / rhs })
            }
            _ => None,
        }
    }

    fn modulo(&self, other: &IVMType) -> Option<IVMType> {
        match (self, other) {
            (IVMType::Integer { value: lhs }, IVMType::Integer { value: rhs }) => {
                if *rhs == 0 {
                    return None;
                }
                Some(IVMType::Integer { value: lhs % rhs })
            }
            _ => None,
        }
    }
}

pub struct VM {
    bytecode: Vec<u8>,
    index: usize,
    memory: HashMap<String, IVMType>,
    stack: Vec<IVMType>,
    labels: HashMap<String, usize>,
    calls: Vec<usize>,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> Self {
        Self {
            bytecode,
            index: 0,
            memory: HashMap::new(),
            stack: Vec::new(),
            labels: HashMap::new(),
            calls: Vec::new(),
        }
    }

    fn can_advance(&self, steps: usize) -> bool {
        self.index + steps <= self.bytecode.len()
    }

    pub fn run(&mut self, resolve: bool) {
        let stdin = std::io::stdin();

        self.index = 0;

        while self.index < self.bytecode.len() {
            let opcode = self.bytecode[self.index];
            self.index += 1;

            match opcode {
                OP_DEBUG => {
                    if resolve {
                        continue;
                    }

                    println!("Stack: {:?}", self.stack);
                    println!("Memory: {:?}", self.memory);
                    println!("Labels: {:?}", self.labels);
                }
                OP_PUSH => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete PUSH instruction");
                        break;
                    }

                    let datatype = self.bytecode[self.index];
                    self.index += 1;

                    match datatype {
                        PUSH_TYPE_INTEGER => {
                            if !self.can_advance(8) {
                                println!("Error: Incomplete Integer data");
                                break;
                            }

                            let int_bytes = &self.bytecode[self.index..self.index + 8];
                            let int_value = i64::from_le_bytes(int_bytes.try_into().unwrap());
                            self.index += 8;

                            if resolve {
                                continue;
                            }

                            self.stack.push(IVMType::Integer { value: int_value });
                        }
                        PUSH_TYPE_FLOAT => {
                            if !self.can_advance(8) {
                                println!("Error: Incomplete Float data");
                                break;
                            }

                            let float_bytes = &self.bytecode[self.index..self.index + 8];
                            let float_value = f64::from_le_bytes(float_bytes.try_into().unwrap());
                            self.index += 8;

                            if resolve {
                                continue;
                            }

                            self.stack.push(IVMType::Float { value: float_value });
                        }
                        PUSH_TYPE_STRING => {
                            if !self.can_advance(4) {
                                println!("Error: Incomplete String length data");
                                break;
                            }

                            let str_len_bytes = &self.bytecode[self.index..self.index + 4];
                            let str_len =
                                u32::from_le_bytes(str_len_bytes.try_into().unwrap()) as usize;
                            self.index += 4;

                            if !self.can_advance(str_len) {
                                println!("Error: Incomplete String data");
                                break;
                            }

                            let str_bytes = &self.bytecode[self.index..self.index + str_len];
                            let str_value = String::from_utf8_lossy(str_bytes).to_string();
                            self.index += str_len;

                            if resolve {
                                continue;
                            }

                            self.stack.push(IVMType::String { value: str_value });
                        }
                        PUSH_TYPE_BOOLEAN => {
                            if !self.can_advance(1) {
                                println!("Error: Incomplete Boolean data");
                                break;
                            }

                            let bool_byte = self.bytecode[self.index];
                            self.index += 1;

                            if resolve {
                                continue;
                            }

                            let bool_value = match bool_byte {
                                0x00 => false,
                                0x01 => true,
                                _ => {
                                    println!("Error: Invalid Boolean value");
                                    break;
                                }
                            };

                            self.stack.push(IVMType::Boolean { value: bool_value });
                        }
                        PUSH_TYPE_INTEGER_POWER => {
                            if !self.can_advance(1) {
                                println!("Error: Incomplete Integer Power data");
                                break;
                            }

                            let power_byte = self.bytecode[self.index];
                            self.index += 1;

                            if resolve {
                                continue;
                            }

                            let int_value = 2i64.pow(power_byte as u32);

                            self.stack.push(IVMType::Integer { value: int_value });
                        }
                        PUSH_TYPE_INTEGER_POWER_SUB => {
                            if !self.can_advance(1) {
                                println!("Error: Incomplete Integer Power Subtraction data");
                                break;
                            }

                            let power_byte = self.bytecode[self.index];
                            self.index += 1;

                            if resolve {
                                continue;
                            }

                            let int_value = 2i64.pow(power_byte as u32) - 1;

                            self.stack.push(IVMType::Integer { value: int_value });
                        }
                        _ => println!("Unknown push data type: 0x{:02X}", datatype),
                    }
                }
                OP_LOAD => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete LOAD instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete LOAD key data");
                        break;
                    }

                    let key_bytes = &self.bytecode[self.index..self.index + str_len];
                    let key = String::from_utf8_lossy(key_bytes).to_string();
                    self.index += str_len;

                    if resolve {
                        continue;
                    }

                    let value = self.memory.get(&key);
                    match value {
                        Some(val) => self.stack.push(val.clone()),
                        None => println!("Error: Key '{}' not found in memory", key),
                    }
                }
                OP_STORE => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete STORE instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete STORE key data");
                        break;
                    }

                    let key_bytes = &self.bytecode[self.index..self.index + str_len];
                    let key = String::from_utf8_lossy(key_bytes).to_string();
                    self.index += str_len;

                    if resolve {
                        continue;
                    }

                    let value = match self.stack.pop() {
                        Some(val) => val,
                        None => {
                            println!("Error: Stack underflow on STORE");
                            break;
                        }
                    };

                    self.memory.insert(key, value);
                }
                OP_DUP => {
                    if resolve {
                        continue;
                    }

                    let value = match self.stack.last() {
                        Some(val) => val.clone(),
                        None => {
                            println!("Error: Stack underflow on DUP");
                            break;
                        }
                    };

                    self.stack.push(value);
                }
                OP_SWAP => {
                    if resolve {
                        continue;
                    }

                    if self.stack.len() < 2 {
                        println!("Error: Stack underflow on SWAP");
                        break;
                    }

                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }
                OP_POP => {
                    if resolve {
                        continue;
                    }

                    if self.stack.is_empty() {
                        println!("Error: Stack underflow on POP");
                        break;
                    }

                    self.stack.pop();
                }
                OP_ADD => {
                    if resolve {
                        continue;
                    }

                    let rhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for ADD");
                            break;
                        }
                    };
                    let lhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for ADD");
                            break;
                        }
                    };

                    let result = lhs.add(&rhs);
                    match result {
                        Some(val) => self.stack.push(val),
                        None => {
                            println!("Error: Incompatible types for ADD");
                            break;
                        }
                    }
                }
                OP_SUB => {
                    if resolve {
                        continue;
                    }

                    let rhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for SUB");
                            break;
                        }
                    };
                    let lhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for SUB");
                            break;
                        }
                    };

                    let result = lhs.sub(&rhs);
                    match result {
                        Some(val) => self.stack.push(val),
                        None => {
                            println!("Error: Incompatible types for SUB");
                            break;
                        }
                    }
                }
                OP_MUL => {
                    if resolve {
                        continue;
                    }

                    let rhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for MUL");
                            break;
                        }
                    };
                    let lhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for MUL");
                            break;
                        }
                    };

                    let result = lhs.mul(&rhs);
                    match result {
                        Some(val) => self.stack.push(val),
                        None => {
                            println!("Error: Incompatible types for MUL");
                            break;
                        }
                    }
                }
                OP_DIV => {
                    if resolve {
                        continue;
                    }

                    let rhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for DIV");
                            break;
                        }
                    };
                    let lhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for DIV");
                            break;
                        }
                    };

                    let result = lhs.div(&rhs);
                    match result {
                        Some(val) => self.stack.push(val),
                        None => {
                            println!("Error: Incompatible types for DIV or division by zero");
                            break;
                        }
                    }
                }
                OP_MOD => {
                    if resolve {
                        continue;
                    }

                    let rhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for MOD");
                            break;
                        }
                    };
                    let lhs = match self.stack.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for MOD");
                            break;
                        }
                    };

                    let result = lhs.modulo(&rhs);
                    match result {
                        Some(val) => self.stack.push(val),
                        None => {
                            println!("Error: Incompatible types for MOD or modulo by zero");
                            break;
                        }
                    }
                }
                OP_STR_GET_SLICE => {
                    if resolve {
                        continue;
                    }

                    let end = match self.stack.pop() {
                        Some(IVMType::Integer { value }) => value as usize,
                        _ => {
                            println!("Error: Expected Integer on stack for STR_GET_SLICE");
                            break;
                        }
                    };

                    let start = match self.stack.pop() {
                        Some(IVMType::Integer { value }) => value as usize,
                        _ => {
                            println!("Error: Expected Integer on stack for STR_GET_SLICE");
                            break;
                        }
                    };

                    let val = match self.stack.pop() {
                        Some(IVMType::String { value }) => value,
                        _ => {
                            println!("Error: Expected String on stack for STR_GET_SLICE");
                            break;
                        }
                    };

                    if start >= val.len() || end > val.len() || start >= end {
                        println!("Error: Invalid slice indices for STR_GET_SLICE");
                        break;
                    }

                    let slice = &val[start..end];
                    self.stack.push(IVMType::String {
                        value: slice.to_string(),
                    });
                }
                OP_STR_LENGTH => {
                    if resolve {
                        continue;
                    }

                    let val = match self.stack.pop() {
                        Some(IVMType::String { value }) => value,
                        _ => {
                            println!("Error: Expected String on stack for STR_LENGTH");
                            break;
                        }
                    };

                    let length = val.len() as i64;
                    self.stack.push(IVMType::Integer { value: length });
                }
                OP_CMP => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete CMP instruction");
                        break;
                    }

                    let cmp_type = self.bytecode[self.index];
                    self.index += 1;

                    if resolve {
                        continue;
                    }

                    let rhs = match self.stack.pop() {
                        Some(val) => val,
                        None => {
                            println!("Error: Stack underflow on CMP");
                            break;
                        }
                    };
                    let lhs = match self.stack.pop() {
                        Some(val) => val,
                        None => {
                            println!("Error: Stack underflow on CMP");
                            break;
                        }
                    };

                    match lhs.compare(&rhs, cmp_type) {
                        Some(result) => self.stack.push(IVMType::Boolean { value: result }),
                        None => {
                            println!("Error: Incompatible types for comparison");
                            break;
                        }
                    }
                }
                OP_LABEL => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete LABEL instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete LABEL name data");
                        break;
                    }

                    let label_bytes = &self.bytecode[self.index..self.index + str_len];
                    let label_name = String::from_utf8_lossy(label_bytes).to_string();
                    self.index += str_len;

                    self.labels.insert(label_name, self.index);
                }
                OP_JUMP => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete JUMP instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete JUMP label data");
                        break;
                    }

                    let label_bytes = &self.bytecode[self.index..self.index + str_len];
                    let label_name = String::from_utf8_lossy(label_bytes).to_string();
                    self.index += str_len;

                    if resolve {
                        continue;
                    }

                    match self.labels.get(&label_name) {
                        Some(&target_index) => self.index = target_index,
                        None => {
                            println!("Error: Label '{}' not found for JUMP", label_name);
                            break;
                        }
                    }
                }
                OP_JUMP_IF_TRUE => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete JUMP_IF_TRUE instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete JUMP_IF_TRUE label data");
                        break;
                    }

                    let label_bytes = &self.bytecode[self.index..self.index + str_len];
                    let label_name = String::from_utf8_lossy(label_bytes).to_string();
                    self.index += str_len;

                    if resolve {
                        continue;
                    }

                    let condition = match self.stack.pop() {
                        Some(IVMType::Boolean { value }) => value,
                        _ => {
                            println!("Error: Expected Boolean on stack for JUMP_IF_TRUE");
                            break;
                        }
                    };

                    if condition {
                        match self.labels.get(&label_name) {
                            Some(&target_index) => self.index = target_index,
                            None => {
                                println!(
                                    "Error: Label '{}' not found for JUMP_IF_TRUE",
                                    label_name
                                );
                                break;
                            }
                        }
                    }
                }
                OP_JUMP_IF_FALSE => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete JUMP_IF_FALSE instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete JUMP_IF_FALSE label data");
                        break;
                    }

                    let label_bytes = &self.bytecode[self.index..self.index + str_len];
                    let label_name = String::from_utf8_lossy(label_bytes).to_string();
                    self.index += str_len;

                    if resolve {
                        continue;
                    }

                    let condition = match self.stack.pop() {
                        Some(IVMType::Boolean { value }) => value,
                        _ => {
                            println!("Error: Expected Boolean on stack for JUMP_IF_FALSE");
                            break;
                        }
                    };

                    if !condition {
                        match self.labels.get(&label_name) {
                            Some(&target_index) => self.index = target_index,
                            None => {
                                println!(
                                    "Error: Label '{}' not found for JUMP_IF_FALSE",
                                    label_name
                                );
                                break;
                            }
                        }
                    }
                }
                OP_CALL => {
                    if !self.can_advance(1) {
                        println!("Error: Incomplete CALL instruction");
                        break;
                    }

                    let str_len = self.bytecode[self.index] as usize;
                    self.index += 1;

                    if !self.can_advance(str_len) {
                        println!("Error: Incomplete CALL label data");
                        break;
                    }

                    let label_bytes = &self.bytecode[self.index..self.index + str_len];
                    let label_name = String::from_utf8_lossy(label_bytes).to_string();
                    self.index += str_len;

                    if resolve {
                        continue;
                    }

                    match self.labels.get(&label_name) {
                        Some(&target_index) => {
                            self.calls.push(self.index);
                            self.index = target_index

                        },
                        None => {
                            println!("Error: Label '{}' not found for CALL", label_name);
                            break;
                        }
                    }
                }
                OP_RETURN => {
                    if resolve {
                        continue;
                    }

                    let return_address = match self.calls.pop() {
                        Some(value) => value,
                        _ => {
                            println!("Error: Expected Integer on stack for RETURN");
                            break;
                        }
                    };

                    self.index = return_address;
                }
                OP_DISPLAY_STDOUT => {
                    if resolve {
                        continue;
                    }

                    let value = match self.stack.pop() {
                        Some(val) => val,
                        None => {
                            println!("Error: Stack underflow on DISPLAY_STDOUT");
                            break;
                        }
                    };

                    match value {
                        IVMType::Integer { value } => println!("{}", value),
                        IVMType::Float { value } => println!("{}", value),
                        IVMType::String { value } => println!("{}", value),
                        IVMType::Boolean { value } => println!("{}", value),
                    }
                }
                OP_DISPLAY_STDERR => {
                    if resolve {
                        continue;
                    }

                    let value = match self.stack.pop() {
                        Some(val) => val,
                        None => {
                            println!("Error: Stack underflow on DISPLAY_STDERR");
                            break;
                        }
                    };

                    match value {
                        IVMType::Integer { value } => eprintln!("{}", value),
                        IVMType::Float { value } => eprintln!("{}", value),
                        IVMType::String { value } => eprintln!("{}", value),
                        IVMType::Boolean { value } => eprintln!("{}", value),
                    }
                }
                OP_INPUT => {
                    if resolve {
                        continue;
                    }

                    let mut input = String::new();
                    match stdin.read_line(&mut input) {
                        Ok(_) => {
                            let input = input.trim_end().to_string();
                            self.stack.push(IVMType::String { value: input });
                        }
                        Err(e) => {
                            println!("Error reading input: {}", e);
                            break;
                        }
                    }
                }
                OP_EXIT => {
                    if resolve {
                        continue;
                    }

                    match self.stack.pop() {
                        Some(IVMType::Integer { value: code }) => {
                            std::process::exit(code as i32);
                        }
                        _ => {
                            println!("Error: Expected Integer on stack for EXIT");
                            break;
                        }
                    }
                }
                _ => {
                    println!("Unknown opcode: 0x{:02X}", opcode);
                    break;
                }
            }
        }
    }
}
