#[derive(Copy, Clone)]
pub struct OpCode {
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
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INY,
    JMP_ABS,
    JMP_IND,
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
    Indirect,
    Indirect_X,
    Indirect_Y,
    Implied,
    Relative,
    NoneAddressing,
}

pub struct EmmulationHelpers {}

impl EmmulationHelpers {
    pub fn get_op_code_struct(code: u8) -> OpCode {
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

            /* DEC opcodes */
            0xC6 => OpCode {
                code: 0xC6,
                code_name: "DEC",
                match_code: OpCodeCat::DEC,
                bytes: 2,
                cycles: 5,
                mode: AddressingMode::ZeroPage,
            },
            0xD6 => OpCode {
                code: 0xC6,
                code_name: "DEC",
                match_code: OpCodeCat::DEC,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::ZeroPage_X,
            },
            0xCE => OpCode {
                code: 0xC6,
                code_name: "DEC",
                match_code: OpCodeCat::DEC,
                bytes: 3,
                cycles: 6,
                mode: AddressingMode::Absolute,
            },
            0xDE => OpCode {
                code: 0xC6,
                code_name: "DEC",
                match_code: OpCodeCat::DEC,
                bytes: 3,
                cycles: 7,
                mode: AddressingMode::Absolute_X,
            },

            /* DEX opcodes */
            0xCA => OpCode {
                code: 0xCA,
                code_name: "DEX",
                match_code: OpCodeCat::DEX,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
            },

            /* DEY opcodes */
            0x88 => OpCode {
                code: 0x88,
                code_name: "DEY",
                match_code: OpCodeCat::DEY,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Implied,
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

            /* EOR opcodes */
            0x49 => OpCode {
                code: 0x49,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 2,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },
            0x45 => OpCode {
                code: 0x45,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 2,
                cycles: 3,
                mode: AddressingMode::ZeroPage,
            },
            0x55 => OpCode {
                code: 0x55,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 2,
                cycles: 4,
                mode: AddressingMode::ZeroPage_X,
            },
            0x4D => OpCode {
                code: 0x4D,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 3,
                cycles: 4,
                mode: AddressingMode::Absolute,
            },
            0x5D => OpCode {
                code: 0x5D,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_X,
            },
            0x59 => OpCode {
                code: 0x59,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 3,
                cycles: 4, /*+1 if page crossed */
                mode: AddressingMode::Absolute_Y,
            },
            0x41 => OpCode {
                code: 0x41,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::Indirect_X,
            },
            0x51 => OpCode {
                code: 0x51,
                code_name: "EOR",
                match_code: OpCodeCat::EOR,
                bytes: 2,
                cycles: 5, /*+1 if page crossed */
                mode: AddressingMode::Indirect_Y,
            },

            /* INC opcodes */
            0xE6 => OpCode {
                code: 0xE6,
                code_name: "INC",
                match_code: OpCodeCat::INC,
                bytes: 2,
                cycles: 5,
                mode: AddressingMode::ZeroPage,
            },
            0xF6 => OpCode {
                code: 0xF6,
                code_name: "INC",
                match_code: OpCodeCat::INC,
                bytes: 2,
                cycles: 6,
                mode: AddressingMode::ZeroPage_X,
            },
            0xEE => OpCode {
                code: 0xEE,
                code_name: "INC",
                match_code: OpCodeCat::INC,
                bytes: 3,
                cycles: 6,
                mode: AddressingMode::Absolute,
            },
            0xFE => OpCode {
                code: 0xFE,
                code_name: "INC",
                match_code: OpCodeCat::INC,
                bytes: 3,
                cycles: 7,
                mode: AddressingMode::Absolute_X,
            },

            /* INY Opcodes */
            0xC8 => OpCode {
                code: 0xC8,
                code_name: "INY",
                match_code: OpCodeCat::INY,
                bytes: 1,
                cycles: 2,
                mode: AddressingMode::Immediate,
            },

            /* JMP Absolute opcodes */
            0x4C => OpCode {
                code: 0x4C,
                code_name: "JMP",
                match_code: OpCodeCat::JMP_ABS,
                bytes: 3,
                cycles: 3,
                mode: AddressingMode::Absolute,
            },

            /* JMP Indirect opcodes */
            0x6c => OpCode {
                code: 0x6C,
                code_name: "JMP",
                match_code: OpCodeCat::JMP_IND,
                bytes: 3,
                cycles: 5,
                mode: AddressingMode::Indirect,
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
}
