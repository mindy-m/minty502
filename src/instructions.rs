use super::*;

impl Registers {
    pub fn lda<AM: ReadableAddressingMode>(&mut self, memory: &mut Memory) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.a = self.status_nz(value);
    }

    pub fn ldx<AM: ReadableAddressingMode>(&mut self, memory: &mut Memory) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.x = self.status_nz(value);
    }

    pub fn ldy<AM: ReadableAddressingMode>(&mut self, memory: &mut Memory) {
        let addressing_mode = AM::new(self, memory);
        let value = addressing_mode.read(self, memory);
        self.y = self.status_nz(value);
    }

    //
}
