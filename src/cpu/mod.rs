use bitflags::bitflags;

use crate::helper::AddressingMode;
use crate::helper::EmmulationHelpers;
use crate::helper::OpCodeCat;

bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuFlags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: CpuFlags::from_bits_truncate(0b100100),
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = CpuFlags::from_bits_truncate(0b100100);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let base = self.mem_read(self.program_counter);
                let addr = base.wrapping_add(self.register_x) as u16;
                addr
            }

            AddressingMode::ZeroPage_Y => {
                let base = self.mem_read(self.program_counter);
                let addr = base.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }

            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }

            AddressingMode::Accumulator => 0xffff,

            AddressingMode::Implied => {
                panic!("mode {:?} is not supported", mode);
            }

            AddressingMode::Relative => {
                panic!("mode {:?} is not supported", mode);
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_a(value);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.register_a)
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.set_register_a(self.register_a & value);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);

        let mut value = 0;

        // impossible memory value means we should use register a
        if addr == 0xffff {
            value = self.register_a;
        } else {
            value = self.mem_read(addr);
        }

        if value >> 7 == 1 {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        value = value << 1;

        if addr == 0xffff {
            self.set_register_a(value);
        } else {
            self.mem_write(addr, value);
            self.update_zero_and_negative_flags(value);
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let test = self.register_a & value;

        self.update_zero_and_negative_flags(test);

        if test >> 6 == 1 {
            self.set_overflow_flag();
        } else {
            self.clear_overflow_flag();
        }
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;

        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;

        self.update_zero_and_negative_flags(self.register_y);
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let value = value.wrapping_sub(1);

        self.mem_write(addr, value);

        self.update_zero_and_negative_flags(value);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = self.register_a ^ value;

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let value = value.wrapping_add(1);

        self.mem_write(addr, value);

        self.update_zero_and_negative_flags(value);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn compare(&mut self, mode: &AddressingMode, compare_with: u8) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if value <= compare_with {
            self.set_carry_flag();
        } else {
            self.clear_carry_flag();
        }

        self.update_zero_and_negative_flags(compare_with.wrapping_sub(value));
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);
            self.program_counter = jump_addr;
        }
    }

    fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn set_overflow_flag(&mut self) {
        self.status.insert(CpuFlags::OVERFLOW);
    }

    fn clear_overflow_flag(&mut self) {
        self.status.remove(CpuFlags::OVERFLOW);
    }

    fn set_carry_flag(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }

    fn clear_carry_flag(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }

    fn set_decimal_mode(&mut self) {
        self.status.insert(CpuFlags::DECIMAL_MODE);
    }

    fn clear_decimal_mode(&mut self) {
        self.status.remove(CpuFlags::DECIMAL_MODE);
    }

    fn set_interupt_disable(&mut self) {
        self.status.insert(CpuFlags::INTERRUPT_DISABLE);
    }

    fn clear_interrupt_disable(&mut self) {
        self.status.remove(CpuFlags::INTERRUPT_DISABLE);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.status.insert(CpuFlags::NEGATIVE);
        } else {
            self.status.remove(CpuFlags::NEGATIVE);
        }
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);

            let val = EmmulationHelpers::get_op_code_struct(opscode);

            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            match val.match_code {
                OpCodeCat::LDA => {
                    self.lda(&val.mode);
                }

                OpCodeCat::STA => {
                    self.sta(&val.mode);
                }

                OpCodeCat::ASL => {
                    self.asl(&val.mode);
                }

                OpCodeCat::AND => {
                    self.and(&val.mode);
                }

                OpCodeCat::TAX => {
                    self.tax();
                }

                OpCodeCat::TAY => {
                    self.tay();
                }

                OpCodeCat::INX => {
                    self.inx();
                }

                OpCodeCat::BCC => {
                    self.branch(!self.status.contains(CpuFlags::CARRY));
                }

                OpCodeCat::BCS => {
                    self.branch(self.status.contains(CpuFlags::CARRY));
                }

                OpCodeCat::BEQ => {
                    self.branch(self.status.contains(CpuFlags::ZERO));
                }

                OpCodeCat::BIT => {
                    self.bit(&val.mode);
                }

                OpCodeCat::BMI => {
                    self.branch(self.status.contains(CpuFlags::NEGATIVE));
                }

                OpCodeCat::BNE => {
                    self.branch(!self.status.contains(CpuFlags::ZERO));
                }

                OpCodeCat::BPL => {
                    self.branch(!self.status.contains(CpuFlags::NEGATIVE));
                }

                OpCodeCat::BVC => {
                    self.branch(!self.status.contains(CpuFlags::OVERFLOW));
                }

                OpCodeCat::BVS => {
                    self.branch(self.status.contains(CpuFlags::OVERFLOW));
                }

                OpCodeCat::CLC => {
                    self.clear_carry_flag();
                }

                OpCodeCat::CLD => {
                    self.clear_decimal_mode();
                }

                OpCodeCat::CLI => {
                    self.clear_interrupt_disable();
                }

                OpCodeCat::CLV => {
                    self.clear_overflow_flag();
                }

                OpCodeCat::CMP => {
                    self.compare(&val.mode, self.register_a);
                }

                OpCodeCat::CPX => {
                    self.compare(&val.mode, self.register_x);
                }

                OpCodeCat::CPY => {
                    self.compare(&val.mode, self.register_y);
                }

                OpCodeCat::DEC => {
                    self.dec(&val.mode);
                }

                OpCodeCat::DEX => {
                    self.dex();
                }

                OpCodeCat::DEY => {
                    self.dey();
                }

                OpCodeCat::EOR => {
                    self.eor(&val.mode);
                }

                OpCodeCat::INC => {
                    self.inc(&val.mode);
                }

                OpCodeCat::INY => {
                    self.iny();
                }

                OpCodeCat::LDX => {
                    self.ldx(&val.mode);
                }

                OpCodeCat::LDY => {
                    self.ldy(&val.mode);
                }

                OpCodeCat::BRK => {
                    self.status.insert(CpuFlags::BREAK);
                    return;
                }
                _ => todo!(""),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += val.bytes - 1;
            }
        }
    }
}

#[cfg(test)]
mod test;
