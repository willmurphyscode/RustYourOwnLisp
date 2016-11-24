
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Add,
    Subtract,
    Multiply,
    Divide
}
impl OpCode {
    pub fn get_opcode(input: char) -> Option<OpCode> {
        match input {
            '+' => Some(OpCode::Add),
            '-' => Some(OpCode::Subtract),
            '*' => Some(OpCode::Multiply),
            '/' => Some(OpCode::Divide),
            _ => None
        }
    }

    pub fn get_char(input: OpCode) -> char {
        match input {
            OpCode::Add => '+',
            OpCode::Subtract => '-',
            OpCode::Multiply => '*',
            OpCode::Divide => '/'
        }
    }

}

#[derive(Debug, Clone)]
pub struct Operation {
   pub opcode : OpCode,
   pub values : Vec<f64>
}

#[derive(Debug, Clone)]
pub enum SExpression {
    atomic(f64),
    op(Operation)
}