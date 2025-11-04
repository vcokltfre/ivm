def debug() -> bytes:
    return bytes([0x00])

def push_int(value: int) -> bytes:
    return bytes([0x01, 0x01]) + value.to_bytes(8, byteorder='little', signed=True)

def push_float(value: float) -> bytes:
    import struct
    return bytes([0x01, 0x02]) + struct.pack('<d', value)

def push_string(value: str) -> bytes:
    encoded = value.encode('utf-8')
    length = len(encoded)
    return bytes([0x01, 0x03]) + length.to_bytes(4, byteorder='little') + encoded

def push_bool(value: bool) -> bytes:
    return bytes([0x01, 0x04, 0x01 if value else 0x00])

def push_integer_power(value: int) -> bytes:
    return bytes([0x01, 0x05]) + value.to_bytes(1, byteorder='little', signed=False)

def push_integer_power_sub(value: int) -> bytes:
    return bytes([0x01, 0x06]) + value.to_bytes(1, byteorder='little', signed=False)

def load(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0x02]) + length.to_bytes(1, byteorder='little') + encoded

def store(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0x03]) + length.to_bytes(1, byteorder='little') + encoded

def dup() -> bytes:
    return bytes([0x04])

def swap() -> bytes:
    return bytes([0x05])

def pop() -> bytes:
    return bytes([0x06])

def free(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0x07]) + length.to_bytes(1, byteorder='little') + encoded

def add() -> bytes:
    return bytes([0x10])

def sub() -> bytes:
    return bytes([0x11])

def mul() -> bytes:
    return bytes([0x12])

def div() -> bytes:
    return bytes([0x13])

def mod() -> bytes:
    return bytes([0x14])

def get_str_slice() -> bytes:
    return bytes([0x20])

def str_length() -> bytes:
    return bytes([0x21])

def cast_itos() -> bytes:
    return bytes([0x30, 0x01])

def cast_stoi() -> bytes:
    return bytes([0x30, 0x02])

def cmp_equal() -> bytes:
    return bytes([0xD0, 0x01])

def cmp_not_equal() -> bytes:
    return bytes([0xD0, 0x02])

def cmp_less() -> bytes:
    return bytes([0xD0, 0x03])

def cmp_greater() -> bytes:
    return bytes([0xD0, 0x04])

def cmp_less_equal() -> bytes:
    return bytes([0xD0, 0x05])

def cmp_greater_equal() -> bytes:
    return bytes([0xD0, 0x06])

def label(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0xE0]) + length.to_bytes(1, byteorder='little') + encoded

def jump(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0xE1]) + length.to_bytes(1, byteorder='little') + encoded

def jump_if_true(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0xE2]) + length.to_bytes(1, byteorder='little') + encoded

def jump_if_false(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0xE3]) + length.to_bytes(1, byteorder='little') + encoded

def call(name: str) -> bytes:
    encoded = name.encode('utf-8')
    length = len(encoded)
    return bytes([0xE4]) + length.to_bytes(1, byteorder='little') + encoded

def ret() -> bytes:
    return bytes([0xE5])

def display_stdout() -> bytes:
    return bytes([0xF0])

def display_stderr() -> bytes:
    return bytes([0xF1])

def get_input() -> bytes:
    return bytes([0xF2])

def halt(code: int | None = None) -> bytes:
    if code is not None:
        return bytes([*push_int(code), 0xFF])
    return bytes([0xFF])

def construct(*parts: bytes) -> bytes:
    return b''.join(parts)
