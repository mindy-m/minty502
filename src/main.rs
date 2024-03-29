mod addressing_modes;
use addressing_modes::*;
mod instructions;
use instructions::*;

/*
v this is the page number
PPPPPPPP_aaaaaaaa
         ^this is the...rest?

Page 0x00: Zero Page
Page 0x01: Stack Page
Page all the rest of the pages don't have fancy names

and this has nothing to do with the modern sense of the word "paging" (except
    that it's chunks of memory)
*/

struct Registers {
    a: u8,     // A is for Accumulator
    x: u8,     // X is for indeX (pre-index)
    y: u8,     // Y is for whYYYYYYYYYY (post-index)
    sp: u8,    // SP is for Stack Pointer
    pc: u16,   // PC is for Program Counter
    flags: u8, // flags is for NV-BDIZC
}

// C is for Carry: the last addition operation carried over
const STATUS_C: u8 = 0b0000_0001;
// Z is for Zero: the last addition operation resulted in zero
const STATUS_Z: u8 = 0b0000_0010;
// I is for Interrupts: whether interrupts are disabled or not
const STATUS_I: u8 = 0b0000_0100;
// D is for Decimal / Dumb: whether we pretend to be a decimal processor instead of a binary one (gross!)
const STATUS_D: u8 = 0b0000_1000;
// B is for Br[ea]k [the program]: whether the last interrupt was actually a BR[ea]K[ the program] instruction in disguise
const STATUS_B: u8 = 0b0001_0000;
// - is for a wire that connects directly to the +5V rail and therefore is always a one (???)
const STATUS_1: u8 = 0b0010_0000;
// V is for oVerflow / Violence / Very strong feelings: FUCK THIS BIT
const STATUS_V: u8 = 0b0100_0000;
// N is for Negative: whether the last operation resulted in a negative number
const STATUS_N: u8 = 0b1000_0000;

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
            // This is a lie, currently voltage-free, I don't care if it dies / its ability to handle the truth
            flags: STATUS_1,
        };
    }
    /// Read and return the byte that the PC points to. After reading, increment
    /// the PC. Each call to this function returns the next byte of the program.
    fn read_program_byte(&mut self, memory: &mut Memory) -> u8 {
        let thing_i_read = memory.read_memory(self.pc);
        self.pc += 1;
        // putting the variable name at the end returns it
        thing_i_read
    }
    /// Read the next byte that the PC points to, interpret it as a branch
    /// offset, and return the address of the branch target.
    fn get_branch_target(&mut self, memory: &mut Memory) -> u16 {
        let branch_offset = self.read_program_byte(memory) as i8 as u16;
        let branch_target = self.pc.wrapping_add(branch_offset);
        branch_target
    }
    /// Push a given byte (`thing_to_push`) onto the stack.
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
    /// Pop a byte from the stack and return it.
    fn pop(&mut self, memory: &mut Memory) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let address = u16::from_le_bytes([
            self.sp, // SSSSSSSS in 00000001_SSSSSSSS
            0x01,    // 00000001 in 00000001_SSSSSSSS
        ]);
        // "This is an excellent variable name. I have no notes."
        // - New York Times (trust me bruh, the article's just paywalled)
        // "I have always been a great believer in the value, nay, necessity,
        // of using as many words to communicate your point as are possible,
        // permissible, and even remotely applicable." - Sir Oscar Wilde, maybe
        // I concur that to be verbose is to be truely alive - consiseness is for losers, only.  This much is clear.
        // "They pay me two cents a word!" - Charles Dickens
        let the_byte_we_read_from_the_stack_just_now =
            memory.read_memory(address);
        // Not really better than "ugh" (a far superior return variable name) but okay
        return the_byte_we_read_from_the_stack_just_now;
    }

    /// Set the N and Z status flags according to this value, and return it.
    fn status_nz(&mut self, value: u8) -> u8 {
        if value == 0 {
            // set if zero
            self.flags |= STATUS_Z;
        } else {
            // clear if not zero
            self.flags &= !STATUS_Z;
        }
        if value & 0b1000_0000 != 0 {
            // set if negative
            self.flags |= STATUS_N;
        } else {
            // clear if positive
            self.flags &= !STATUS_N;
        }
        value
    }
    /// Set the C, N, and Z status flags and return the value.
    fn status_cnz(&mut self, value: u16) -> u8 {
        if value >= 0b1_0000_0000 {
            // set if the addition carried past 8 bits
            self.flags |= STATUS_C;
        } else {
            // clear if the addition fit perfectly in 8 bits
            self.flags &= !STATUS_C;
        }
        self.status_nz(value as u8)
    }

    fn step(&mut self, memory: &mut Memory) {
        // By calling read_program_byte, we no longer need to have this code:
        //let opcode = read_memory(self.pc);
        //self.pc += 1;
        let opcode = self.read_program_byte(memory);

        // Like a switch, but better, also don't need to put in break
        match opcode {
            0x09 => {
                // ORA
                // OR Memory with Accumulator
                // (Addressing mode is immediate)
                self.ora::<Immediate>(memory);
            }

            0x18 => {
                // CLC
                // Clear Carry Flag
                // Addr: Implied
                self.flags &= !STATUS_C;
            }

            0x20 => {
                // JSR
                // Jump to SubRoutine
                let address_to_jump_to = u16::from_le_bytes([
                    self.read_program_byte(memory),
                    self.read_program_byte(memory),
                ]);
                let pc_bytes = self.pc.to_le_bytes();
                // maybe not accurate to the real 6502, but it will work
                // Also, can only push 1 byte at a time.
                self.push(memory, pc_bytes[0]);
                self.push(memory, pc_bytes[1]);
                // (the real 6502 has a weird rule about having to subtract
                // one...)
                self.pc = address_to_jump_to;
            }

            0x30 => {
                //BMI
                // Branch on Result Minus
                // Addressing is like, relative
                self.branch_if(memory, (self.flags & STATUS_N) == STATUS_N);
            }

            0x3A => {
                // DEA
                // DEcrement A
                self.a = self.status_nz(self.a.wrapping_sub(1));
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

            0x4C => {
                // JMP around
                // Jump to new location
                // Addr: Indirect
                let address_to_jump_to = u16::from_le_bytes([
                    self.read_program_byte(memory),
                    self.read_program_byte(memory),
                ]);

                self.pc = address_to_jump_to;
            }

            0x5A => {
                // PHY
                // (PusH Y)
                // (I am using implied addressing mode)
                self.push(memory, self.y);
            }
            0x60 => {
                // RTS
                // ReTurn from Subroutine
                // (I am also using implied addressing mode)
                let pc_bytes_1 = self.pop(memory);
                let pc_bytes_0 = self.pop(memory);
                let pc_byte_array = [pc_bytes_0, pc_bytes_1];
                // Needs to know endian-ness, and convert to u16
                self.pc = u16::from_le_bytes(pc_byte_array);
                // Going to be the opposite of this:
                //     // JSR
                //     // Jump to SubRoutine
                //     let address_to_jump_to = u16::from_le_bytes([
                //         self.read_program_byte(memory),
                //         self.read_program_byte(memory),
                //     ]);
                //     let pc_bytes = self.pc.to_le_bytes();
                //     // maybe not accurate to the real 6502, but it will work
                //     // Also, can only push 1 byte at a time.
                //     self.push(memory, pc_bytes[0]);
                //     self.push(memory, pc_bytes[1]);
                //     // (the real 6502 has a weird rule about having to subtract
                //     // one...)
                //     self.pc = address_to_jump_to;
                // }
            }

            0x68 => {
                // PLA
                // Pull accumulator from stack
                self.a = self.pop(memory);
            }

            0x69 => {
                // nice
                // ADC
                // Add Memory to Accumulator with Carry
                // Addr: immediate
                self.adc::<Immediate>(memory);
            }

            0x7A => {
                // PLY
                // PuLl Y (pop Y)
                self.y = self.pop(memory);
            }
            0x80 => {
                // BRA offset
                // BRanch Always
                self.branch_if(memory, true);
                // self.pc = self.get_branch_target(memory);
            }
            0x85 => {
                // STA zp
                // (STore Accumulator Zero Page)
                // on the 6502, the "page" is just the upper byte of the
                // address. Not to be confused with "paging" on modern CPUs.
                // let address_to_store_at = u16::from_le_bytes([
                //     self.read_program_byte(memory), // le sad 😿
                //     0,
                // ]);
                // memory.write_memory(address_to_store_at, self.a);

                self.store::<ZeroPage>(memory, self.a);
            }
            0x8D => {
                // STA abs
                // STore Accumulator (ABSolute address)
                self.store::<Absolute>(memory, self.a);
            }
            0x8E => {
                // STX abs
                // STore X (ABSolute address)
                // Take the value that's in X and store it at the given address
                // let address_to_store_at = u16::from_le_bytes([
                //     self.read_program_byte(memory),
                //     self.read_program_byte(memory),
                // ]);
                // memory.write_memory(address_to_store_at, self.x);
                self.store::<Absolute>(memory, self.x);
            }

            0x90 => {
                // BCC
                // Branch on Carry Clear
                // Addr: Relative
                self.branch_if(memory, (self.flags & STATUS_C) != STATUS_C);
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
                self.store::<Absolute>(memory, 0);
            }
            0xA0 => {
                // LDY #imm
                // (LoaD Y IMMediate)
                // let value_to_put_in_y = self.read_program_byte(memory);
                // eprintln!("We are putting a value in Y! And it is: 0x{value_to_put_in_y:02X}");
                // self.y = value_to_put_in_y;
                self.ldy::<Immediate>(memory);
            }
            0xA2 => {
                // LDX #imm
                // (LoaD X IMMediate)
                // Read a value from the program and store that value in X
                // let value_to_put_in_x = self.read_program_byte(memory);
                // eprintln!("We are putting a value in X! And it is: 0x{value_to_put_in_x:02X}");
                // self.x = value_to_put_in_x;
                self.ldx::<Immediate>(memory);
            }

            0xA5 => {
                // LDA
                // Load accumulator with memory
                // (Addressing mode is zero page)
                self.lda::<ZeroPage>(memory);
            }

            0xA9 => {
                // LDA #imm
                // (LoaD A IMMediate)
                self.lda::<Immediate>(memory);
            }

            0xAD => {
                // LDA abs
                // Load accumulator, absolute
                self.lda::<Absolute>(memory);
            }

            0xB0 => {
                // BCS
                // Branch on carry set
                // (addressing mode - relative)
                self.branch_if(memory, (self.flags & STATUS_C) == STATUS_C);
            }

            0xB1 => {
                // LDA ind,Y
                // (LoaD Accumulator, zero page INDirect Y-indexed)
                self.lda::<ZeroPageIndirectYIndexed>(memory);
            }

            0xC6 => {
                // DEC zpg
                // Decrememnt memory by one, zero-page
                self.dec::<ZeroPage>(memory);
            }

            0xC8 => {
                // INY
                // (INcrement Y)
                self.y = self.status_nz(self.y.wrapping_add(1));
            }

            0xC9 => {
                // CMP
                // Compare Memory with Accumulator
                // (Addressing mode is immediate)
                self.cmp::<Immediate>(memory);
            }
            0xCB => {
                // WAI
                // WAit for Interrupt
                // but we don't have interrupts
                // ...
                // ... heh heh
                // (Not needed but I hate this)
            }

            0xD0 => {
                // BNE rel
                // Branch if not equal, relative addressing mode
                self.branch_if(memory, (self.flags & STATUS_Z) != STATUS_Z);
            }

            0xE8 => {
                // INC X
                // (INCrement X)
                // Add 1 to the value in X. Easy, right? ...right? 🦈
                //self.x = self.x.wrapping_add(1);
                // Defeat the shark!
                self.x = self.status_nz(self.x.wrapping_add(1));
            }
            0xF0 => {
                // BEQ offset
                // (Branch if EQual → Branch if the Z bit is *set*)
                // let branch_target = self.get_branch_target(memory);
                // // Doing an AND with STATUS_Z since it only has the bit set that we care about, and we're checking flags only for that bit.
                // if (self.flags & STATUS_Z) == STATUS_Z {
                //     self.pc = branch_target;
                // }
                self.branch_if(memory, (self.flags & STATUS_Z) == STATUS_Z);
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
            //println!("Address is 0x{address_we_want_to_store:04X} and data is 0x{byte_to_store:02X}.");
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
            // Solra promised to explain this later. Well, it's later!
            use std::io::Read;
            let mut buf = [0u8; 1];
            std::io::stdin().read_exact(&mut buf).unwrap();
            buf[0]
        } else {
            // All that's left SHOULD be the ROM range, but check just in case
            debug_assert!((0x8000..=0xFFFF).contains(&address));
            // eprintln!(
            //     "Reading ROM at 0x{address:04X}, result is 0x{:02X}",
            //     self.rom_bytes[address as usize - 0x8000]
            // );
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
