use super::*;

pub trait ReadableAddressingMode {
    fn read(&self, reggy: &mut Registers, memory: &mut Memory) -> u8;

    fn new(reggy: &mut Registers, memory: &mut Memory) -> Self;
    //let addressing_mode = AM::new(self, memory);
    //self.a = addressing_mode.read(self, memory);
}
pub trait WritableAddressingMode {
    fn write(&self, reggy: &mut Registers, memory: &mut Memory, value: u8);
}

pub struct Immediate {
    value: u8,
}

impl ReadableAddressingMode for Immediate {
    fn new(reggy: &mut Registers, memory: &mut Memory) -> Immediate {
        let value = reggy.read_program_byte(memory);
        Immediate { value }
    }
    fn read(&self, reggy: &mut Registers, memory: &mut Memory) -> u8 {
        self.value
    }
}

pub struct ZeroPageIndirectYIndexed {
    address: u16,
}

impl ReadableAddressingMode for ZeroPageIndirectYIndexed {
    fn read(&self, reggy: &mut Registers, memory: &mut Memory) -> u8 {
        // ...and add Y to. THIS is the value that we ACTUALLY read.
        memory.read_memory(self.address)
    }
    fn new(
        reggy: &mut Registers,
        memory: &mut Memory,
    ) -> ZeroPageIndirectYIndexed {
        // Read the next byte. It is the address of...
        // Also Rust is a pain and makes you use "as" to change sizes
        let address_of_pointer = reggy.read_program_byte(memory) as u16;
        // ...a two-byte pointer, which we read...
        let the_real_correct_pointer = u16::from_le_bytes([
            memory.read_memory(address_of_pointer),
            memory.read_memory(address_of_pointer + 1),
        ]);
        let address = the_real_correct_pointer + reggy.y as u16;
        ZeroPageIndirectYIndexed { address }
    }
}

impl WritableAddressingMode for ZeroPageIndirectYIndexed {
    fn write(&self, reggy: &mut Registers, memory: &mut Memory, value: u8) {
        // ...and add Y to. THIS is the value that we ACTUALLY read.
        memory.write_memory(self.address, value);
    }
}
