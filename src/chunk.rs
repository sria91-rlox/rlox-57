use super::Value;

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub line_numbers: Vec<i64>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
	    line_numbers: Vec::new(),
        }
    }

    pub fn write_op(&mut self, op: OpCode, line: i64) {
        self.code.push(op);
	self.line_numbers.push(line);
    }

    pub fn write_const(&mut self, value: Value) -> usize {
        self.constants.push(value);
	self.constants.len() - 1
    }

    pub fn disassemble(&self) {
        println!("-- Text: --");
        for (i, op) in self.code.iter().enumerate() {
            self.disassemble_instruction(i, op);
        }
        println!("-- Constants: --");
        for (i, val) in self.constants.iter().enumerate() {
            self.disassemble_constant(i, *val);
        }
        println!("-- end --");
    }

    fn disassemble_constant(&self, offset: usize, val: Value) {
        println!("{:04} {}", offset, val);
    }

    fn disassemble_instruction(&self, offset: usize,  op: &OpCode) {
        match op {
            OpCode::OpReturn => {
		if offset > 0 && self.line_numbers[offset] == self.line_numbers[offset-1] {
                    println!("     | {:04} RET", offset);
		} else {
                    println!("{:06} {:04} RET", self.line_numbers[offset], offset);
		}
            }
            OpCode::OpConstant(index) => {
		if offset > 0 && self.line_numbers[offset] == self.line_numbers[offset-1] {
                    println!(
			"     | {:04} LDI {} ; {}",
			offset, index, self.constants[*index]);
		} else {
                    println!(
			"{:06} {:04} LDI {} ; {}",
			self.line_numbers[offset], offset, index, self.constants[*index]);
		}
            }
        }
    }
}
