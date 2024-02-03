use super::*;

impl Registers {
    pub fn lda<AM: ReadableAddressingMode>(&mut self, memory: &mut Memory) {
        let addressing_mode = AM::new(self, memory);
        self.a = addressing_mode.read(self, memory);
        if self.a == 0 {
            self.flags |= STATUS_Z;
        } else {
            self.flags &= !STATUS_Z;
        }
        if self.a & 0x80 != 0 {
            self.flags |= STATUS_N;
        } else {
            self.flags &= !STATUS_N;
        }
    }
}
