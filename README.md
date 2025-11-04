# ivm

A bytecode VM.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Instructions

### `DEBUG` - `0x00`

Prints the current state of the VM for debugging purposes.

### `PUSH` - `0x01`

Pushes a value onto the stack. The next byte indicates the type of the value:

- `0x01`: Integer (8 bytes)
- `0x02`: Float (8 bytes)
- `0x03`: String (length-prefixed)
- `0x04`: Boolean (1 byte, `0x00` for false, `0x01` for true)
- `0x05`: Integer Power of 2 (exponent as 1 byte)
- `0x06`: Integer Power of 2 Sub 1 (exponent as 1 byte)

### `LOAD` - `0x02`

Loads a value from memory onto the stack. The next byte indicates the length of the variable name, followed by the variable name itself.

### `STORE` - `0x03`

Stores a value from the stack into memory. The next byte indicates the length of the variable name, followed by the variable name itself.

### `DUP` - `0x04`

Duplicates the top value on the stack.

### `SWAP` - `0x05`

Swaps the top two values on the stack.

### `POP` - `0x06`

Removes the top value from the stack.

### `FREE` - `0x07`

Free a variable from memory. The next byte indicates the length of the variable name, followed by the variable name itself.

### `LOAD_REF` - `0x08`

Loads a reference to a variable from memory onto the stack.

### `STORE_REF` - `0x09`

Stores a reference from the stack into memory.

### `ADD` - `0x10`

Adds the top two values on the stack and pushes the result back onto the stack.

### `SUB` - `0x11`

Subtracts the top two values on the stack and pushes the result back onto the stack.

### `MUL` - `0x12`

Multiplies the top two values on the stack and pushes the result back onto the stack.

### `DIV` - `0x13`

Divides the top two values on the stack and pushes the result back onto the stack.

### `MOD` - `0x14`

Calculates the modulus of the top two integer values on the stack and pushes the result back onto the stack.

### `STR_GET_SLICE` - `0x20`

Gets a substring from the string at the top of the stack (excluding start and end indices). The next two values on the stack are the start (top-1) and end (top) indices (integers). Pushes the resulting substring back onto the stack.

### `STR_LENGTH` - `0x21`

Calculates the length of the string at the top of the stack and pushes the length (integer) back onto the stack.

### `CAST` - `0x30`

Casts the top value on the stack to a different type. The next byte indicates the target type:

- `0x01`: Integer to String
- `0x02`: String to Integer

### `CMP` - `0xD0`

Compares the top two values on the stack. The next byte indicates the type of comparison:

- `0x01`: Equal
- `0x02`: Not Equal
- `0x03`: Greater Than
- `0x04`: Less Than
- `0x05`: Greater Than or Equal
- `0x06`: Less Than or Equal

### `LABEL` - `0xE0`

Defines a label at the current instruction index. The next byte indicates the length of the label name, followed by the label name itself.

### `JMP` - `0xE1`

Jumps to the instruction index of the specified label. The next byte indicates the length of the label name, followed by the label name itself.

### `JMP_IF_TRUE` - `0xE2`

Jumps to the instruction index of the specified label if the top value on the stack is true. The next byte indicates the length of the label name, followed by the label name itself.

### `JMP_IF_FALSE` - `0xE3`

Jumps to the instruction index of the specified label if the top value on the stack is false. The next byte indicates the length of the label name, followed by the label name itself.

### `CALL` - `0xE4`

Calls a function at the instruction index of the specified label. The next byte indicates the length of the label name, followed by the label name itself.

### `RET` - `0xE5`

Returns from the current function call.

### `DISPLAY_STDOUT` - `0xF0`

Prints the top value on the stack to standard output.

### `DISPLAY_STDERR` - `0xF1`

Prints the top value on the stack to standard error.

### `INPUT` - `0xF2`

Reads a line of input from standard input and pushes it onto the stack as a string.

### `EXIT` - `0xFF`

Exits the program. The top value on the stack is used as the exit code (integer).
