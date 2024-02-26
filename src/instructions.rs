use super::*;

impl Registers {
    pub(crate) fn lda<AM: ReadableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.a = self.status_nz(value);
    }

    pub(crate) fn ldx<AM: ReadableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.x = self.status_nz(value);
    }

    pub(crate) fn ldy<AM: ReadableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.y = self.status_nz(value);
    }

    pub(crate) fn store<AM: WritableAddressingMode>(
        &mut self,
        memory: &mut Memory,
        value: u8,
    ) {
        let addressing_mode = AM::new(self, memory);
        addressing_mode.write(self, memory, value);
    }

    pub(crate) fn ora<AM: ReadableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.a = self.status_nz(value | self.a);
    }

    pub(crate) fn cmp<AM: ReadableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        // We don't care what the resulting value is, we just care about the
        // flag bits getting set.
        let _ = self.evil_add(!value, true);
    }

    //self.branch_if(memory, (self.flags & STATUS_C) == STATUS_C);
    pub(crate) fn branch_if(&mut self, memory: &mut Memory, condition: bool) {
        let branch_target = self.get_branch_target(memory);
        if condition {
            self.pc = branch_target;
        }
    }

    // not public, too evil
    fn evil_add(&mut self, value: u8, carry_in: bool) -> u8 {
        let first_thing = self.a as u16;
        let second_thing = value as u16;
        let third_thing = if carry_in { 1 } else { 0 };
        let resulting_thing = first_thing + second_thing + third_thing;
        self.status_cnz(resulting_thing)
    }

    pub(crate) fn dec<AM: WritableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        // Step 1: Where is the thing?
        let addressing_mode = AM::new(self, memory);
        // Step 2: Get the thing
        let value = addressing_mode.read(self, memory);
        // Step 3: Decrement the thing
        // Shadowing nonsense....  :(
        let value = self.status_nz(value.wrapping_sub(1));
        // Step 4: Put the thing back
        addressing_mode.write(self, memory, value);
    }

    pub(crate) fn adc<AM: ReadableAddressingMode>(
        &mut self,
        memory: &mut Memory,
    ) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.a = self.evil_add(value, (self.flags & STATUS_C) == STATUS_C);
    }
}
