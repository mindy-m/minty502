struct Registers {
    a: u8,     // A is for Accumulator
    x: u8,     // X is for indeX (pre-index)
    y: u8,     // Y is for whYYYYYYYYYY (post-index)
    sp: u8,    // SP is for Stack Pointer
    pc: u16,   // PC is for Program Counter
    flags: u8, // flags is for NV-BDIZC
}

// C is for Carry: the last addition operation carried over
// Z is for Zero: the last addition operation resulted in zero
// I is for Interrupts: whether interrupts are disabled or not
// D is for Decimal / Dumb: whether we pretend to be a decimal processor instead of a binary one (gross!)
// B is for Br[ea]k [the program]: whether the last interrupt was actually a BR[ea]K[ the program] instruction in disguise
// - is for a wire that connects directly to the +5V rail and therefore is always a one (???)
// V is for oVerflow / Violence / Very strong feelings: FUCK THIS BIT
// N is for Negative: whether the lasst operation resulted in a negative number

// 2 bytes
const RESET_VECTOR: u16 = 0xFFFC;

impl Registers {
    fn new_chippy(memory: &mut Memory) -> Registers {
        return Registers {
            // In general, giving default values is optional - not really in this case though
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: u16::from_le_bytes([
                memory.read_memory(RESET_VECTOR),
                memory.read_memory(RESET_VECTOR + 1),
            ]),
            flags: 0,
        };
    }
    fn read_program_byte(&mut self, memory: &mut Memory) -> u8 {
        let thing_i_read = memory.read_memory(self.pc);
        self.pc += 1;
        // putting the variable name at the end returns it
        thing_i_read
    }

    fn push(&mut self, memory: &mut Memory, thing_to_push: u8) {
        // An address on the stack looks like:
        // 00000001_SSSSSSSS
        // We are using this: little endian = least significant byte first (backwards)
        // Solra really prefers this (but I don't care): big endian = most significant byte first (frontwards)
        // but Chuck Peddle didn't so WE ARE STUCK FOREVER THANKS CHUCK
        let address = u16::from_le_bytes([
            self.sp, // SSSSSSSS in 00000001_SSSSSSSS
            0x01,    // 00000001 in 00000001_SSSSSSSS
        ]);
        memory.write_memory(address, thing_to_push);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn step(&mut self, memory: &mut Memory) {
        // let opcode = read_memory(self.pc);
        // self.pc += 1;
        let opcode = self.read_program_byte(memory);

        // Like a switch, but better, also don't need to put in break
        match opcode {
            0x20 => {
                // JSR
                // Jump to SubRoutine
                let address_to_jump_to = u16::from_le_bytes([
                    self.read_program_byte(memory),
                    self.read_program_byte(memory),
                ]);
                // ??? remember where we came from so we can come back ???
                self.pc = address_to_jump_to;
            }
            0x48 => {
                // PHA
                // (PusH Accumulator)
                self.push(memory, self.a);
                /*
                let address = u16::from_le_bytes([
                    self.sp,
                    0x01,
                ]);
                memory.write_memory(address, self.a);
                self.sp = self.sp.wrapping_sub(1);
                */
            }
            0x5A => {
                // PHY
                // (PusH Y)
                self.push(memory, self.y);
            }
            0x85 => {
                // STA zp
                // (STore Accumulator Zero Page)
                // on the 6502, the "page" is just the upper byte of the
                // address. Not to be confused with "paging" on modern CPUs.
                let address_to_store_at = u16::from_le_bytes([
                    self.read_program_byte(memory), // le sad 😿
                    0,
                ]);
                memory.write_memory(address_to_store_at, self.a);
            }
            0x8E => {
                // STX abs
                // STore X (ABSolute address)
                // Take the value that's in X and store it at the given address
                let address_to_store_at = u16::from_le_bytes([
                    self.read_program_byte(memory),
                    self.read_program_byte(memory),
                ]);
                memory.write_memory(address_to_store_at, self.x);
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
                    self.read_program_byte(memory),
                    self.read_program_byte(memory),
                ]);
                memory.write_memory(address_to_store_at, 0);
            }
            0xA0 => {
                // LDY #imm
                // (LoaD Y IMMediate)
                let value_to_put_in_y = self.read_program_byte(memory);
                eprintln!("We are putting a value in Y! And it is: 0x{value_to_put_in_y:02X}");
                self.y = value_to_put_in_y;
            }
            0xA2 => {
                // LDX #imm
                // (LoaD X IMMediate)
                // Read a value from the program and store that value in X
                let value_to_put_in_x = self.read_program_byte(memory);
                eprintln!("We are putting a value in X! And it is: 0x{value_to_put_in_x:02X}");
                self.x = value_to_put_in_x;
            }
            0xA9 => {
                // LDA #imm
                // (LoaD A IMMediate)
                let value_to_put_in_a = self.read_program_byte(memory);
                eprintln!("We are putting a value in A! And it is: 0x{value_to_put_in_a:02X}");
                self.a = value_to_put_in_a;
            }
            0xB1 => {
                // LDA ind,Y
                // (LoaD Accumulator, zero page INDirect Y-indexed)
                // Read the next byte. It is the address of...
                // Also Rust is a pain and makes you use "as" to change sizes
                let address_of_pointer = self.read_program_byte(memory) as u16;
                // ...a two-byte pointer, which we read...
                let the_real_correct_pointer = u16::from_le_bytes([
                    memory.read_memory(address_of_pointer),
                    memory.read_memory(address_of_pointer + 1),
                ]);
                // ...and add Y to. THIS is the value that we ACTUALLY read.
                self.a = memory
                    .read_memory(the_real_correct_pointer + self.y as u16);
            }
            0xE8 => {
                // INC X
                // (INCrement X)
                // Add 1 to the value in X. Easy, right? ...right? 🦈
                self.x = self.x.wrapping_add(1);
            }
            0xF0 => {
                // BEQ offset
                // (Branch if EQual)

                // For real though
                panic!("Oh fuck we had flags this whole time oh god oh god oh god")
            }
            _ => {
                todo!("Still learning what opcode 0x{opcode:02X} is...");
            }
        }
    }
}

// Memory Mario - plumbing, because Rust doesn't* have globals
// (*mostly)
struct Memory {
    ram_bytes: [u8; 0x4000],
    // &'static ← This reference lives forever
    // [u8] ← some contiguous number of u8s. .. 0 to a bunch
    rom_bytes: &'static [u8],
}

impl Memory {
    // Example
    // fn read_program_byte(&mut self) -> u8 {
    //     let thing_i_read = read_memory(self.pc);
    //     self.pc += 1;
    //     // putting the variable name at the end returns it
    //     thing_i_read
    // }

    fn write_memory(
        &mut self,
        address_we_want_to_store: u16,
        byte_to_store: u8,
    ) {
        /*
        0000 to 3FFF: RAM
        4000 to 7FFF: IO
        8000 to FFFF: ROM
        */

        // address_to_store_at, self.x

        if (0x0000..=0x3FFF).contains(&address_we_want_to_store) {
            // Rust makes you use usize here - gross.  "as" is used when converting one type of integer to another
            self.ram_bytes[address_we_want_to_store as usize] = byte_to_store;
            // println! prints to stdout
            // eprintln! prints to stderr
            println!("Address is 0x{address_we_want_to_store:04X} and data is 0x{byte_to_store:02X}.");
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
            debug_assert!(
                (0x8000..=0xFFFF).contains(&address_we_want_to_store)
            );
            panic!("You can't write to ROM! Not even to address 0x{address_we_want_to_store:04X}! Not even with byte 0x{byte_to_store:04X}!!!");
        }
    }
    // Don't forget fn when defining a function.... and say what it returns with -> cool data type to return
    fn read_memory(&mut self, address: u16) -> u8 {
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
            self.ram_bytes[address as usize]
        } else if (0x4000..=0x7FFF).contains(&address) {
            todo!("This is totally where IO is, address is 0x{address:04X}.");
        } else {
            // All that's left SHOULD be the ROM range, but check just in case
            debug_assert!((0x8000..=0xFFFF).contains(&address));
            eprintln!(
                "Reading ROM at 0x{address:04X}, result is 0x{:02X}",
                self.rom_bytes[address as usize - 0x8000]
            );
            return self.rom_bytes[address as usize - 0x8000];
        }
    }
}

fn main() {
    let mut memory = Memory {
        // mirrors the syntax [u8; 0x4000] that defined the type
        ram_bytes: [0; 0x4000],
        rom_bytes: include_bytes!("rom.bin"),
    };
    // Need to use Registers:: to use new_chippy
    let mut registers = Registers::new_chippy(&mut memory);
    // Rust is loopy.
    loop {
        registers.step(&mut memory);
    }
}

// fooのbar
// Solraのsandwich
// の = ::
