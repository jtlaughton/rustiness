#[derive(Copy, Clone)]
struct OpCode {
    pub code: u8,
    pub code_name: &'static str,
    pub match_code: OpCodeCat,
    pub bytes: u16,
    pub cycles: u8,
    pub mode: AddressingMode
}

#[derive(Clone, Copy)]
pub enum OpCodeCat {
    TAX,
    LDA,
    TAY,
    STA,
    AND,
    INX,
    BRK,
    NONE
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
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
   NoneAddressing,
}

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self{
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn get_op_code_struct(&self, code: u8) -> OpCode{
        match code {
            0x00 => OpCode { code: 0x00, code_name: "BRK", match_code: OpCodeCat::BRK, bytes: 1, cycles: 7, mode: AddressingMode::NoneAddressing },

            0xAA => OpCode { code: 0xAA, code_name: "TAX", match_code: OpCodeCat::TAX, bytes: 1, cycles: 2, mode: AddressingMode::Implied },

            0xA8 => OpCode { code: 0xA8, code_name: "TAY", match_code: OpCodeCat::TAY, bytes: 1, cycles: 2, mode: AddressingMode::Implied },

            0xA9 => OpCode { code: 0xA9, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 2, cycles: 2, mode: AddressingMode::Immediate },
            0xA5 => OpCode { code: 0xA5, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 2, cycles: 3, mode: AddressingMode::ZeroPage },
            0xB5 => OpCode { code: 0xB5, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 2, cycles: 4, mode: AddressingMode::ZeroPage_X },
            0xAD => OpCode { code: 0xAD, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 3, cycles: 4, mode: AddressingMode::Absolute },
            0xBD => OpCode { code: 0xBD, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 3, cycles: 4 /*+1 if page crossed */, mode: AddressingMode::Absolute_X },
            0xB9 => OpCode { code: 0xB9, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 3, cycles: 4 /*+1 if page crossed */, mode: AddressingMode::Absolute_Y },
            0xA1 => OpCode { code: 0xA1, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 2, cycles: 6, mode: AddressingMode::Indirect_X },
            0xB1 => OpCode { code: 0xB1, code_name: "LDA", match_code: OpCodeCat::LDA, bytes: 2, cycles: 5 /*+1 if page crossed */, mode: AddressingMode::Indirect_Y },

            0x85 => OpCode { code: 0x85, code_name: "STA", match_code: OpCodeCat::STA, bytes: 2, cycles: 3, mode: AddressingMode::ZeroPage },
            0x95 => OpCode { code: 0x95, code_name: "STA", match_code: OpCodeCat::STA, bytes: 2, cycles: 4, mode: AddressingMode::ZeroPage_X },
            0x8D => OpCode { code: 0x8D, code_name: "STA", match_code: OpCodeCat::STA, bytes: 3, cycles: 4, mode: AddressingMode::Absolute },
            0x9D => OpCode { code: 0x9D, code_name: "STA", match_code: OpCodeCat::STA, bytes: 3, cycles: 5, mode: AddressingMode::Absolute_X },
            0x99 => OpCode { code: 0x99, code_name: "STA", match_code: OpCodeCat::STA, bytes: 3, cycles: 5, mode: AddressingMode::Absolute_Y },
            0x81 => OpCode { code: 0x81, code_name: "STA", match_code: OpCodeCat::STA, bytes: 2, cycles: 6, mode: AddressingMode::Indirect_X },
            0x91 => OpCode { code: 0x91, code_name: "STA", match_code: OpCodeCat::STA, bytes: 2, cycles: 6, mode: AddressingMode::Indirect_Y },

            0x29 => OpCode { code: 0x29, code_name: "AND", match_code: OpCodeCat::AND, bytes: 2, cycles: 2, mode: AddressingMode::Immediate },
            0x25 => OpCode { code: 0x25, code_name: "AND", match_code: OpCodeCat::AND, bytes: 2, cycles: 3, mode: AddressingMode::ZeroPage },
            0x35 => OpCode { code: 0x35, code_name: "AND", match_code: OpCodeCat::AND, bytes: 2, cycles: 4, mode: AddressingMode::ZeroPage_X },
            0x2D => OpCode { code: 0x2D, code_name: "AND", match_code: OpCodeCat::AND, bytes: 3, cycles: 4, mode: AddressingMode::Absolute },
            0x3D => OpCode { code: 0x3D, code_name: "AND", match_code: OpCodeCat::AND, bytes: 3, cycles: 4 /*+1 if page crossed */, mode: AddressingMode::Absolute_X },
            0x39 => OpCode { code: 0x39, code_name: "AND", match_code: OpCodeCat::AND, bytes: 3, cycles: 4 /*+1 if page crossed */, mode: AddressingMode::Absolute_Y },
            0x21 => OpCode { code: 0x21, code_name: "AND", match_code: OpCodeCat::AND, bytes: 2, cycles: 6, mode: AddressingMode::Indirect_X },
            0x31 => OpCode { code: 0x31, code_name: "AND", match_code: OpCodeCat::AND, bytes: 2, cycles: 5 /*+1 if page crossed */, mode: AddressingMode::Indirect_Y },

            0xE8 => OpCode { code: 0xE8, code_name: "INX", match_code: OpCodeCat::INX, bytes: 1, cycles: 2, mode: AddressingMode::Implied },

            _ => OpCode { code: 0xFF, code_name: "NONE", match_code: OpCodeCat::NONE, bytes: 1, cycles: 1, mode: AddressingMode::NoneAddressing }
        }
    }

    fn mem_read(&self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16{
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;

        (hi << 8) | (lo)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16){
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self){
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>){
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
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
            },

            AddressingMode::ZeroPage_Y => {
                let base = self.mem_read(self.program_counter);
                let addr = base.wrapping_add(self.register_y) as u16;
                addr
            },

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            },

            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            },

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
            },

            AddressingMode::Implied => {
                panic!("mode {:?} is not supported", mode);
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self){
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self){
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn inx(&mut self){
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn sta(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        
        self.mem_write(addr, self.register_a)
    }

    fn and(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = self.register_a & value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8){
        if result == 0{
            self.status = self.status | 0b0000_0010;
        }else{
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0{
            self.status = self.status | 0b1000_0000;
        }else{
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn run(&mut self){
        loop {
            let opscode = self.mem_read(self.program_counter);

            let val = self.get_op_code_struct(opscode);
            self.program_counter += 1;

            match val.match_code {
                OpCodeCat::LDA => {
                    self.lda(&val.mode);
                }

                OpCodeCat::STA => {
                    self.sta(&val.mode);
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

                OpCodeCat::BRK => {
                    return;
                }
                _ => todo!("")
            }

            self.program_counter += val.bytes - 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
  
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }
 
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
     }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_tax_status_zero(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_tax_status_negative(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0xFF);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tay_move_a_to_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xa8, 0x00]);

        assert_eq!(cpu.register_y, 10)
    }

    #[test]
    fn test_tay_status_zero(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xa8, 0x00]);

        assert_eq!(cpu.register_y, 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_tay_status_negative(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xFF, 0xa8, 0x00]);

        assert_eq!(cpu.register_y, 0xFF);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

   #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
   fn test_lda_from_memory() {
       let mut cpu = CPU::new();
       cpu.mem_write(0x10, 0x55);

       cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

       assert_eq!(cpu.register_a, 0x55);
   }

   #[test]
   fn test_lda_b5(){
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xb5, 0x0f, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
   }

   #[test]
   fn test_lda_ad(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x1000, 0x55);

        cpu.load_and_run(vec![0xad, 0x00, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
   }

   #[test]
   fn test_lda_bd(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x1000, 0x55);

        cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xbd, 0xff, 0x0f, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
   }

   #[test]
   fn test_lda_b9(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x1000, 0x55);

        cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xb9, 0xff, 0x0f, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_lda_a1(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x10, 0xCCCC);
        cpu.mem_write_u16(0xCCCC, 0x0a);

        cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa1, 0x0f, 0x00]);

        assert_eq!(cpu.register_a, 0x0a);
    }

    #[test]
    fn test_lda_b1(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x10, 0xCCCB);
        cpu.mem_write_u16(0xCCCC, 0x0a);

        cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xb1, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x0a);
    }

   #[test]
   fn test_sta_85(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0x85, 0xFF, 0x00]);

        let value = cpu.mem_read(0xFF);

        assert_eq!(value, 0x0a);
   }

   #[test]
   fn test_sta_95(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0x0a, 0x95, 0xFE, 0x00]);

        let value = cpu.mem_read(0xFF);

        assert_eq!(value, 0x0a);
   }

   #[test]
   fn test_sta_8d(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0x8D, 0xFF, 0x10, 0x00]);

        let value = cpu.mem_read(0x10FF);

        assert_eq!(value, 0x0a);
    }

   #[test]
   fn test_sta_9d(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0x0a, 0x9D, 0xFE, 0x10, 0x00]);

        let value = cpu.mem_read_u16(0x10FF);

        assert_eq!(value, 0x0a);
   }

   #[test]
   fn test_sta_99(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xa9, 0x0a, 0x99, 0xFE, 0x10, 0x00]);

        let value = cpu.mem_read_u16(0x10FF);

        assert_eq!(value, 0x0a);
   }

   #[test]
   fn test_sta_81(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x10, 0xCCCC);
        cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xa9, 0x0a, 0x81, 0x0F, 0x00]);

        let value = cpu.mem_read_u16(0xCCCC);

        assert_eq!(value, 0x0a);
   }

   #[test]
   fn test_sta_91(){
        let mut cpu = CPU::new();
        cpu.mem_write_u16(0x10, 0xCCCB);
        cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xa9, 0x0a, 0x91, 0x10, 0x00]);

        let value = cpu.mem_read_u16(0xCCCC);

        assert_eq!(value, 0x0a);
   }

//    #[test]
//    fn test_lda_b9(){
//         let mut cpu =  CPU::new();
//         cpu.mem_write_u16(0x1000, 0x55)
//    }
 }