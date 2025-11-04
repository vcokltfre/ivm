OP_DEBUG = 0x00
OP_PUSH = 0x01
OP_LOAD = 0x02
OP_STORE = 0x03
OP_DUP = 0x04
OP_SWAP = 0x05
OP_POP = 0x06
OP_FREE = 0x07

OP_ADD = 0x10
OP_SUB = 0x11
OP_MUL = 0x12
OP_DIV = 0x13
OP_MOD = 0x14

OP_STR_GET_SLICE = 0x20
OP_STR_LENGTH = 0x21

OP_CAST = 0x30

OP_CMP = 0xD0

OP_LABEL = 0xE0
OP_JUMP = 0xE1
OP_JUMP_IF_TRUE = 0xE2
OP_JUMP_IF_FALSE = 0xE3
OP_CALL = 0xE4
OP_RETURN = 0xE5

OP_DISPLAY_STDOUT = 0xF0
OP_DISPLAY_STDERR = 0xF1
OP_INPUT = 0xF2
OP_EXIT = 0xFF

PUSH_TYPE_INTEGER = 0x01
PUSH_TYPE_FLOAT = 0x02
PUSH_TYPE_STRING = 0x03
PUSH_TYPE_BOOLEAN = 0x04
PUSH_TYPE_INTEGER_POWER = 0x05
PUSH_TYPE_INTEGER_POWER_SUB = 0x06

CAST_TYPE_ITOS = 0x01
CAST_TYPE_STOI = 0x02

CMP_TYPE_EQUAL = 0x01
CMP_TYPE_NOT_EQUAL = 0x02
CMP_TYPE_LESS_THAN = 0x03
CMP_TYPE_GREATER_THAN = 0x04
CMP_TYPE_LESS_EQUAL = 0x05
CMP_TYPE_GREATER_EQUAL = 0x06

def disassemble(data: bytes) -> None:
    index = 0

    while index < len(data):
        c = data[index]
        index += 1

        if c == OP_DEBUG:
            print(f"{index - 1:08x}: DEBUG")
        elif c == OP_PUSH:
            ptype = data[index]
            index += 1

            if ptype == PUSH_TYPE_INTEGER:
                value = int.from_bytes(data[index:index+8], byteorder='little', signed=True)
                index += 8
                print(f"{index - 5:08x}: PUSH INTEGER {value}")
            elif ptype == PUSH_TYPE_FLOAT:
                import struct
                value = struct.unpack('<d', data[index:index+8])[0]
                index += 8
                print(f"{index - 5:08x}: PUSH FLOAT {value}")
            elif ptype == PUSH_TYPE_STRING:
                strlen = int.from_bytes(data[index:index+4], byteorder='little')
                index += 4
                value = data[index:index+strlen].decode('utf-8')
                index += strlen
                print(f"{index - (2 + strlen):08x}: PUSH STRING \"{value}\"")
            elif ptype == PUSH_TYPE_BOOLEAN:
                value = data[index]
                index += 1
                print(f"{index - 2:08x}: PUSH BOOLEAN {bool(value)}")
            elif ptype == PUSH_TYPE_INTEGER_POWER:
                exponent = data[index]
                index += 1
                print(f"{index - 2:08x}: PUSH INTEGER POWER {exponent}")
            elif ptype == PUSH_TYPE_INTEGER_POWER_SUB:
                exponent = data[index]
                index += 1
                print(f"{index - 2:08x}: PUSH INTEGER POWER SUB {exponent}")
        elif c == OP_LOAD:
            namelen = data[index]
            index += 1
            name = data[index:index+namelen].decode('utf-8')
            index += namelen
            print(f"{index - (1 + namelen):08x}: LOAD {name}")
        elif c == OP_STORE:
            namelen = data[index]
            index += 1
            name = data[index:index+namelen].decode('utf-8')
            index += namelen
            print(f"{index - (1 + namelen):08x}: STORE {name}")
        elif c == OP_DUP:
            print(f"{index - 1:08x}: DUP")
        elif c == OP_SWAP:
            print(f"{index - 1:08x}: SWAP")
        elif c == OP_POP:
            print(f"{index - 1:08x}: POP")
        elif c == OP_FREE:
            namelen = data[index]
            index += 1
            name = data[index:index+namelen].decode('utf-8')
            index += namelen
            print(f"{index - (1 + namelen):08x}: FREE {name}")
        elif c == OP_ADD:
            print(f"{index - 1:08x}: ADD")
        elif c == OP_SUB:
            print(f"{index - 1:08x}: SUB")
        elif c == OP_MUL:
            print(f"{index - 1:08x}: MUL")
        elif c == OP_DIV:
            print(f"{index - 1:08x}: DIV")
        elif c == OP_MOD:
            print(f"{index - 1:08x}: MOD")
        elif c == OP_STR_GET_SLICE:
            print(f"{index - 1:08x}: STR_GET_SLICE")
        elif c == OP_STR_LENGTH:
            print(f"{index - 1:08x}: STR_LENGTH")
        elif c == OP_CAST:
            ctype = data[index]
            index += 1

            if ctype == CAST_TYPE_ITOS:
                ctype_str = "ITOS"
            elif ctype == CAST_TYPE_STOI:
                ctype_str = "STOI"
            else:
                ctype_str = f"UNKNOWN({ctype})"
            print(f"{index - 2:08x}: CAST TYPE {ctype} ({ctype_str})")
        elif c == OP_CMP:
            ctype = data[index]
            index += 1

            if ctype == CMP_TYPE_EQUAL:
                ctype_str = "EQUAL"
            elif ctype == CMP_TYPE_NOT_EQUAL:
                ctype_str = "NOT_EQUAL"
            elif ctype == CMP_TYPE_LESS_THAN:
                ctype_str = "LESS_THAN"
            elif ctype == CMP_TYPE_GREATER_THAN:
                ctype_str = "GREATER_THAN"
            elif ctype == CMP_TYPE_LESS_EQUAL:
                ctype_str = "LESS_EQUAL"
            elif ctype == CMP_TYPE_GREATER_EQUAL:
                ctype_str = "GREATER_EQUAL"
            else:
                ctype_str = f"UNKNOWN({ctype})"
            print(f"{index - 2:08x}: CMP TYPE {ctype} ({ctype_str})")
        elif c == OP_LABEL:
            labellen = data[index]
            index += 1
            label = data[index:index+labellen].decode('utf-8')
            index += labellen
            print(f"{index - (1 + labellen):08x}: LABEL {label}")
        elif c == OP_JUMP:
            labellen = data[index]
            index += 1
            label = data[index:index+labellen].decode('utf-8')
            index += labellen
            print(f"{index - (1 + labellen):08x}: JUMP {label}")
        elif c == OP_JUMP_IF_TRUE:
            labellen = data[index]
            index += 1
            label = data[index:index+labellen].decode('utf-8')
            index += labellen
            print(f"{index - (1 + labellen):08x}: JUMP_IF_TRUE {label}")
        elif c == OP_JUMP_IF_FALSE:
            labellen = data[index]
            index += 1
            label = data[index:index+labellen].decode('utf-8')
            index += labellen
            print(f"{index - (1 + labellen):08x}: JUMP_IF_FALSE {label}")
        elif c == OP_CALL:
            labellen = data[index]
            index += 1
            label = data[index:index+labellen].decode('utf-8')
            index += labellen
            print(f"{index - (1 + labellen):08x}: CALL {label}")
        elif c == OP_RETURN:
            print(f"{index - 1:08x}: RETURN")
        elif c == OP_DISPLAY_STDOUT:
            print(f"{index - 1:08x}: DISPLAY_STDOUT")
        elif c == OP_DISPLAY_STDERR:
            print(f"{index - 1:08x}: DISPLAY_STDERR")
        elif c == OP_INPUT:
            print(f"{index - 1:08x}: INPUT")
        elif c == OP_EXIT:
            print(f"{index - 1:08x}: EXIT")
        else:
            print(f"{index - 1:08x}: UNKNOWN OPCODE {c}")
            break

if __name__ == "__main__":
    import sys

    if len(sys.argv) != 2:
        print("Usage: python dis.py <file.ivm>")
        sys.exit(1)

    filename = sys.argv[1]
    with open(filename, "rb") as f:
        data = f.read()

    disassemble(data)
