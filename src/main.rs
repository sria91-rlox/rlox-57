mod chunk;
use chunk::{Chunk, OpCode};
mod value;
pub use value::Value;

fn main() {
    println!("ret {:#?}", OpCode::OpReturn);
    let mut c = Chunk::new();
    let offset = c.write_const(17_f64);
    c.write_op(OpCode::OpConstant(offset), 1);
    c.write_op(OpCode::OpReturn, 2);
    let offset = c.write_const(-42_f64);
    c.write_op(OpCode::OpConstant(offset), 3);
    let offset = c.write_const(0.3_f64);
    c.write_op(OpCode::OpConstant(offset), 3);
    c.write_op(OpCode::OpReturn, 3);
    c.disassemble();
}
