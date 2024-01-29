const INSTRUCTION_SIZE: usize = 4;

#[derive(Debug)]
enum OPCODES {
    ADD,
    SUB,
    LOAD,
    STORE,
    IDLE,
    BREAK,
}

pub struct Processor {
    registers: [u32; 32],
    pc: usize,
    memory: Vec<u8>,
    opcode: OPCODES,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: [0; 32],
            pc: 0,
            memory: vec![0; 512],
            opcode: OPCODES::IDLE,
        }
    }

    fn extract_bits(&mut self, value: u32, start: usize, end: usize) -> usize {
        // Adjust the start and end to extract bits from the higher-order bits
        let adjusted_start = 32 - end;
        let adjusted_end = 32 - start;
    
        // Create a mask from the adjusted start to end
        let mask = ((1 << (adjusted_end - adjusted_start)) - 1) << adjusted_start;
    
        // Shift the bits to the right to align with the lower-order bits
        return ((value & mask) >> adjusted_start) as usize;
    }

    fn decode_opcode(&mut self, opcode_bits: u32) -> OPCODES {
        match opcode_bits {
            0b110011 => OPCODES::ADD,
            0b110000 => OPCODES::SUB,
            0b000011 => OPCODES::LOAD,
            0b100011 => OPCODES::STORE,
            0 => OPCODES::BREAK,
            _ => panic!("Неизвестный опкод: {:b}", opcode_bits),
        }
    }

    fn execute_instruction(&mut self) -> u8 {
        // Читаем инструкцию из памяти
        let instruction = u32::from_le_bytes([
            self.memory[self.pc],
            self.memory[self.pc + 1],
            self.memory[self.pc + 2],
            self.memory[self.pc + 3],
        ]);

        // Извлекаем поля из инструкци
        //
        // 110011_0000_00001_00011_01010101010
        // ^^^^^^ ^^^^ ^^^^^ ^^^^^ ^^^^^^^^^^^
        // opcode  rg   rs1   rs2      imm
        //
        
        let opcode = self.extract_bits(instruction, 0, 7);
        let rd = self.extract_bits(instruction, 8, 11);
        let rs1 = self.extract_bits(instruction, 12, 16);
        let rs2 = self.extract_bits(instruction, 17, 21);
        let imm = self.extract_bits(instruction, 22, 32) as i32;

        // Декодируем опкод
        self.opcode = self.decode_opcode(opcode.try_into().unwrap());

        if opcode != 0 { 
            println!("Opcode: {:?} ({:0b})", self.opcode, opcode); 
        }

        // В зависимости от опкода, вызываем соответствующую функцию выполнения инструкции
        match self.opcode {
            OPCODES::ADD => self.add(rd, rs1),
            OPCODES::SUB => self.sub(rd, rs1),
            OPCODES::LOAD => self.load(rd, rs1, imm),
            OPCODES::STORE => self.store(rs1, rs2, imm),
            OPCODES::IDLE => (),
            OPCODES::BREAK => return 0
            // Другие опкоды...
        }

        // Перейти к следующей инструкции
        self.pc += INSTRUCTION_SIZE;
        return 1;
    }

    // Функция для извлечения битов из значения
    

    fn add(&mut self, rd: usize, rs1: usize) {
        self.registers[rd] += rs1 as u32;
    }

    fn sub(&mut self, rd: usize, rs1: usize) {
        self.registers[rd] -= rs1 as u32;
    }

    fn load(&mut self, rd: usize, rs1: usize, imm: i32) {
        let address = (rs1 as i32 + imm) as usize;
        // Предполагаем, что у нас есть 32-битные слова в памяти
        self.registers[rd] = u32::from_le_bytes([
            self.memory[address],
            self.memory[address + 1],
            self.memory[address + 2],
            self.memory[address + 3],
        ]);
    }

    fn store(&mut self, rs1: usize, rs2: usize, imm: i32) {
        let address = (rs1 as i32 + imm) as usize;
        let value = self.registers[rs2];

        let bytes = value.to_le_bytes();
        self.memory[address] = bytes[0];
        self.memory[address + 1] = bytes[1];
        self.memory[address + 2] = bytes[2];
        self.memory[address + 3] = bytes[3];
    }

    pub fn load_program(&mut self, program: &[u32]) {
        for (i, &instruction) in program.iter().enumerate() {
            let address = i * INSTRUCTION_SIZE;
            let bytes = instruction.to_le_bytes();
            println!("[{}] Loading instruction {:08b} in {}", i, instruction, address);
        
            // Use from_le_bytes to convert little-endian bytes to u32
            self.memory[address..address + INSTRUCTION_SIZE].copy_from_slice(&bytes);
        }
    }

    pub fn execute(&mut self) {
        while self.pc < self.memory.len() {
            println!();
    
            let result = self.execute_instruction();
            
            if result == 0 {
                break;
            }
    
            println!("pc: {}", self.pc - INSTRUCTION_SIZE);
            println!("regs: {:?}", self.registers);
        }
    
        println!();
    
        println!("final regs: {:?}", self.registers);
    }
}

