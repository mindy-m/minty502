struct Registers {
    a: u8,     // A is for Accumulator
    x: u8,     // X is for indeX
    y: u8,     // Y is for whYYYYYYYYYY
    sp: u8,    // SP is for Stack Pointer
    pc: u16,   // PC is for Program Counter
    flags: u8, // flags is for NV-BDIZC
}

// 2 bytes
const RESET_VECTOR: u16 = 0xFFFC;

impl Registers {
    fn new_chippy() -> Registers {
        return Registers {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: u16::from_le_bytes([
                read_memory(RESET_VECTOR),
                read_memory(RESET_VECTOR + 1),
            ]),
            flags: 0,
        };
    }
    fn read_program_byte(&mut self) -> u8 {
        let thing_i_read = read_memory(self.pc);
        self.pc += 1;
        // putting the variable name at the end returns it
        thing_i_read
    }

    fn step(&mut self) {
        // let opcode = read_memory(self.pc);
        // self.pc += 1;
        let opcode = self.read_program_byte();

        // Like a switch, but better, also don't need to put in break
        match opcode {
            0x85 => {
                // STA zp
                // (STore Accumulator Zero Page)
                // on the 6502, the "page" is just the upper byte of the
                // address. Not to be confused with "paging" on modern CPUs.
                let address_to_store_at = u16::from_le_bytes([
                    self.read_program_byte(), // le sad 😿
                    0,
                ]);
                write_memory(address_to_store_at, self.a);
            }
            0x8E => {
                // STX abs
                // STore X (ABSolute address)
                // Take the value that's in X and store it at the given address
                let address_to_store_at = u16::from_le_bytes([
                    self.read_program_byte(),
                    self.read_program_byte(),
                ]);
                write_memory(address_to_store_at, self.x);
            }
            0x9A => {
                // TXS
                // Transfer X to Stack pointer
                //
                // self.  self.  to do the thing w/ the stuff
                self.sp = self.x;
            }

            // Not on the originial 6502
            0x9C => {
                // STZ abs
                // (STore Zero ABSolute)
                let address_to_store_at = u16::from_le_bytes([
                    self.read_program_byte(),
                    self.read_program_byte(),
                ]);
                write_memory(address_to_store_at, 0);
            }
            0xA0 => {
                // LDY #imm
                // (LoaD Y IMMediate)
                let value_to_put_in_y = self.read_program_byte();
                eprintln!("We are putting a value in Y! And it is: 0x{value_to_put_in_y:02X}");
                self.y = value_to_put_in_y;
            }
            0xA2 => {
                // LDX #imm
                // (LoaD X IMMediate)
                // Read a value from the program and store that value in X
                let value_to_put_in_x = self.read_program_byte();
                eprintln!("We are putting a value in X! And it is: 0x{value_to_put_in_x:02X}");
                self.x = value_to_put_in_x;
            }
            0xA9 => {
                // LDA #imm
                // (LoaD A IMMediate)
                let value_to_put_in_a = self.read_program_byte();
                eprintln!("We are putting a value in A! And it is: 0x{value_to_put_in_a:02X}");
                self.a = value_to_put_in_a;
            }
            0xE8 => {
                // INC X
                // (INCrement X)
                // Add 1 to the value in X. Easy, right? ...right? 🦈
                self.x = self.x.wrapping_add(1);
            }
            _ => {
                todo!("Still learning what opcode 0x{opcode:02X} is...");
            }
        }
    }
}

const ROM_BYTES: &[u8] = include_bytes!("rom.bin");

// Don't forget fn when defining a function.... and say what it returns with -> cool data type to return
fn read_memory(address: u16) -> u8 {
    /*
    0000 to 3FFF: RAM
    4000 to 7FFF: IO
    8000 to FFFF: ROM
    */
    // Possibility 1:
    //if address >= 0x0000 && address <= 0x3FFF
    // Possibility 2:
    //if address < 0x4000
    // Possibility 3:
    //if (0x0000 ..= 0x3FFF).contains(&address)
    if (0x0000..=0x3FFF).contains(&address) {
        todo!("Something something RAM, address is 0x{address:04X}.");
    } else if (0x4000..=0x7FFF).contains(&address) {
        todo!("This is totally where IO is, address is 0x{address:04X}.");
    } else {
        // All that's left SHOULD be the ROM range, but check just in case
        debug_assert!((0x8000..=0xFFFF).contains(&address));
        eprintln!(
            "Reading ROM at 0x{address:04X}, result is 0x{:02X}",
            ROM_BYTES[address as usize - 0x8000]
        );
        return ROM_BYTES[address as usize - 0x8000];
    }
}

fn write_memory(address_we_want_to_store: u16, byte_to_store: u8) {
    /*
    0000 to 3FFF: RAM
    4000 to 7FFF: IO
    8000 to FFFF: ROM
    */

    // address_to_store_at, self.x

    if (0x0000..=0x3FFF).contains(&address_we_want_to_store) {
        todo!("Something something RAM, address is 0x{address_we_want_to_store:04X} and data is 0x{byte_to_store:02X}.");
    } else if (0x4000..=0x7FFF).contains(&address_we_want_to_store) {
        // & in this case means bitwise ANDing
        if (byte_to_store & 0b1000_0000) != 0 {
            // High bit is set. Clear the screen
            println!("\n\n--- (pretend the screen just cleared) ---\n");
        } else if (byte_to_store) == 0 {
            // we don't have a keyboard buffer, but if we did, this is where
            // we would clear it. :)
        } else {
            // (currently this is magic)
            use std::io::Write;
            std::io::stdout().write_all(&[byte_to_store]).unwrap();
        }

        //todo!("This is totally where IO is, address is 0x{address_we_want_to_store:04X} and data is 0x{byte_to_store:02X}.");
    } else {
        // All that's left SHOULD be the ROM range, but check just in case
        debug_assert!((0x8000..=0xFFFF).contains(&address_we_want_to_store));
        panic!("You can't write to ROM! Not even to address 0x{address_we_want_to_store:04X}! Not even with byte 0x{byte_to_store:04X}!!!");
    }
}

fn main() {
    // Need to use Registers:: to use new_chippy
    let mut registers = Registers::new_chippy();
    // Rust is loopy.
    loop {
        registers.step();
    }
}
