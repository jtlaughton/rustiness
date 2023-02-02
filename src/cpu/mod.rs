use bitflags::bitflags;

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

#[derive(Copy, Clone)]
struct OpCode {
    pub code: u8,
    pub code_name: &'static str,
    pub match_code: OpCodeCat,
    pub bytes: u16,
    pub cycles: u8,
    pub mode: AddressingMode,
}

#[derive(Clone, Copy)]
pub enum OpCodeCat {
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    LDX,
    LDY,
    TAX,
    LDA,
    TAY,
    STA,
    AND,
    INX,
    BRK,
    NONE,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    Implied,
    Relative,
    NoneAddressing,
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

    fn get_op_code_struct(&self, code: u8) -> OpCode {
        match code {
            /* BRK */
            0x00 => OpCode {
                code: 0x00,
                code_name: "BRK",
                match_code: OpCodeCat::BRK,
                bytes: 1,
                cycles: 7,
                mode: AddressingMode::NoneAddressing,
            },

            /* TAX opcodes*/
            0xAA => OpCode {
                code: 0xAA,
                code_name: "TAX",
                match_code: OpCodeCat::TAX,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* TAY opcodes */
            0xA8 => OpCode {
                code: 0xA8,
                code_name: "TAY",
                match_code: OpCodeCat::TAY,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* lDA opcodes */
            0xA9 => OpCode {
                code: 0xA9,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0xA5 => OpCode {
                code: 0xA5,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0xB5 => OpCode {
                code: 0xB5,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_X,
            },
            0xAD => OpCode {
                code: 0xAD,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0xBD => OpCode {
                code: 0xBD,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_X,
            },
            0xB9 => OpCode {
                code: 0xB9,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_Y,
            },
            0xA1 => OpCode {
                code: 0xA1,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::Indirect_X,
            },
            0xB1 => OpCode {
                code: 0xB1,
                code_name: "LDA",
                match_code: OpCodeCat::LDA,
                bytes: 2,
                cycles: 5, /*+1 if page crossed */
                mode: AddressingMode::Indirect_Y,
            },

            /* STA opcodes */
            0x85 => OpCode {
                code: 0x85,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0x95 => OpCode {
                code: 0x95,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_X,
            },
            0x8D => OpCode {
                code: 0x8D,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0x9D => OpCode {
                code: 0x9D,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 3,
                cycles: 5,
                mode: AddressingMode::Absolute_X,
            },
            0x99 => OpCode {
                code: 0x99,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 3,
                cycles: 5,
                mode: AddressingMode::Absolute_Y,
            },
            0x81 => OpCode {
                code: 0x81,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::Indirect_X,
            },
            0x91 => OpCode {
                code: 0x91,
                code_name: "STA",
                match_code: OpCodeCat::STA,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::Indirect_Y,
            },

            /* AND opcodes */
            0x29 => OpCode {
                code: 0x29,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0x25 => OpCode {
                code: 0x25,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0x35 => OpCode {
                code: 0x35,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_X,
            },
            0x2D => OpCode {
                code: 0x2D,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0x3D => OpCode {
                code: 0x3D,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_X,
            },
            0x39 => OpCode {
                code: 0x39,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_Y,
            },
            0x21 => OpCode {
                code: 0x21,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::Indirect_X,
            },
            0x31 => OpCode {
                code: 0x31,
                code_name: "AND",
                match_code: OpCodeCat::AND,
                bytes: 2,
                cycles: 5, /*+1 if page crossed */
                mode: AddressingMode::Indirect_Y,
            },

            /* ASL opcodes */
            0x0A => OpCode {
                code: 0x0A,
                code_name: "ASL",
                match_code: OpCodeCat::ASL,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Accumulator,
            },
            0x06 => OpCode {
                code: 0x06,
                code_name: "ASL",
                match_code: OpCodeCat::ASL,
                bytes: 2,
                cycles: 5,
                mode: AddressingMode::ZeroPage,
            },
            0x16 => OpCode {
                code: 0x16,
                code_name: "ASL",
                match_code: OpCodeCat::ASL,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::ZeroPage_X,
            },
            0x0E => OpCode {
                code: 0x0E,
                code_name: "ASL",
                match_code: OpCodeCat::ASL,
                bytes: 3,
                cycles: 6,
                mode: AddressingMode::Absolute,
            },
            0x1E => OpCode {
                code: 0x1E,
                code_name: "ASL",
                match_code: OpCodeCat::ASL,
                bytes: 3,
                cycles: 7,
                mode: AddressingMode::Absolute_X,
            },

            /* BCC opcodes */
            0x90 => OpCode {
                code: 0x90,
                code_name: "BCC",
                match_code: OpCodeCat::BCC,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BCS opcodes */
            0xB0 => OpCode {
                code: 0xB0,
                code_name: "BCS",
                match_code: OpCodeCat::BCS,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BEQ opcodes */
            0xF0 => OpCode {
                code: 0xF0,
                code_name: "BEQ",
                match_code: OpCodeCat::BEQ,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BIT opcodes */
            0x24 => OpCode {
                code: 0x24,
                code_name: "BIT",
                match_code: OpCodeCat::BIT,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0x2C => OpCode {
                code: 0x2c,
                code_name: "BIT",
                match_code: OpCodeCat::BIT,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },

            /* BMI opcodes */
            0x30 => OpCode {
                code: 0x30,
                code_name: "BMI",
                match_code: OpCodeCat::BMI,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BNE opcodes */
            0xD0 => OpCode {
                code: 0xD0,
                code_name: "BNE",
                match_code: OpCodeCat::BNE,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BPL opcodes */
            0x10 => OpCode {
                code: 0x10,
                code_name: "BPL",
                match_code: OpCodeCat::BPL,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BVC opcodes */
            0x50 => OpCode {
                code: 0x50,
                code_name: "BVC",
                match_code: OpCodeCat::BVC,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* BVS opcodes */
            0x70 => OpCode {
                code: 0x70,
                code_name: "BVS",
                match_code: OpCodeCat::BVS,
                bytes: 2,
                cycles: 2, /* +1 if success, +2 if new page */
                mode: AddressingMode::Relative,
            },

            /* INX opcodes */
            0xE8 => OpCode {
                code: 0xE8,
                code_name: "INX",
                match_code: OpCodeCat::INX,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* CLC Opcodes */
            0x18 => OpCode {
                code: 0x18,
                code_name: "CLC",
                match_code: OpCodeCat::CLC,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* CLD Opcodes */
            0xD8 => OpCode {
                code: 0xD8,
                code_name: "CLD",
                match_code: OpCodeCat::CLD,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* CLI Opcodes */
            0x58 => OpCode {
                code: 0x58,
                code_name: "CLI",
                match_code: OpCodeCat::CLI,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* CLV Opcodes */
            0xB8 => OpCode {
                code: 0xB8,
                code_name: "CLV",
                match_code: OpCodeCat::CLV,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* CMP Opcodes */
            0xC9 => OpCode {
                code: 0xC9,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0xC5 => OpCode {
                code: 0xC5,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0xD5 => OpCode {
                code: 0xD5,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_X,
            },
            0xCD => OpCode {
                code: 0xCD,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0xDD => OpCode {
                code: 0xDD,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_X,
            },
            0xD9 => OpCode {
                code: 0xD9,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_Y,
            },
            0xC1 => OpCode {
                code: 0xC1,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::Indirect_X,
            },
            0xD1 => OpCode {
                code: 0xD1,
                code_name: "CMP",
                match_code: OpCodeCat::CMP,
                bytes: 2,
                cycles: 5, /*+1 if page crossed */
                mode: AddressingMode::Indirect_Y,
            },

            /* CPX Opcodes */
            0xE0 => OpCode {
                code: 0xE0,
                code_name: "CPX",
                match_code: OpCodeCat::CPX,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0xE4 => OpCode {
                code: 0xE4,
                code_name: "CPX",
                match_code: OpCodeCat::CPX,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0xEC => OpCode {
                code: 0xEC,
                code_name: "CPX",
                match_code: OpCodeCat::CPX,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },

            /* CPY Opcodes */
            0xC0 => OpCode {
                code: 0xC0,
                code_name: "CPY",
                match_code: OpCodeCat::CPY,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0xC4 => OpCode {
                code: 0xC4,
                code_name: "CPY",
                match_code: OpCodeCat::CPY,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0xCC => OpCode {
                code: 0xCC,
                code_name: "CPY",
                match_code: OpCodeCat::CPY,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },

            /* LDX Opcodes */
            0xA2 => OpCode {
                code: 0xA2,
                code_name: "LDX",
                match_code: OpCodeCat::LDX,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0xA6 => OpCode {
                code: 0xA6,
                code_name: "LDX",
                match_code: OpCodeCat::LDX,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0xB6 => OpCode {
                code: 0xB6,
                code_name: "LDX",
                match_code: OpCodeCat::LDX,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_Y,
            },
            0xAE => OpCode {
                code: 0xAE,
                code_name: "LDX",
                match_code: OpCodeCat::LDX,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0xBE => OpCode {
                code: 0xBE,
                code_name: "LDX",
                match_code: OpCodeCat::LDX,
                bytes: 3,
                cycles: 4, /* +1 if page crossed */
                mode: AddressingMode::Absolute_Y,
            },

            /* LDY Opcodes */
            0xA0 => OpCode {
                code: 0xA0,
                code_name: "LDY",
                match_code: OpCodeCat::LDY,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0xA4 => OpCode {
                code: 0xA4,
                code_name: "LDY",
                match_code: OpCodeCat::LDY,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0xB4 => OpCode {
                code: 0xB4,
                code_name: "LDY",
                match_code: OpCodeCat::LDY,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_X,
            },
            0xAC => OpCode {
                code: 0xAC,
                code_name: "LDY",
                match_code: OpCodeCat::LDY,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0xBC => OpCode {
                code: 0xBC,
                code_name: "LDY",
                match_code: OpCodeCat::LDY,
                bytes: 3,
                cycles: 4, /* +1 if page crossed */
                mode: AddressingMode::Absolute_X,
            },

            /* Default case so it still increments the program counter */
            _ => OpCode {
                code: 0xFF,
                code_name: "NONE",
                match_code: OpCodeCat::NONE,
                bytes: 1,
                cycles: 1,
                mode: AddressingMode::NoneAddressing,
            },
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

            let val = self.get_op_code_struct(opscode);

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
