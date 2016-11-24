
#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
   pub opcode : OpCode,
   pub values : Vec<f64>
}

#[derive(Debug, Clone, PartialEq)]
pub enum SExpression {
    atomic(f64),
    op(Operation)
}

impl SExpression {
    pub fn eval(&self) -> f64 {
        match *self {
            SExpression::atomic(f) => f,
            SExpression::op(ref operation) => {
                match operation.opcode {
                    OpCode::Add => operation.values.iter().fold(0f64, |sum, &val| sum + val),
                    OpCode::Subtract => operation.values.iter().fold(0f64, |diff, &val| diff - val),
                    _ => unimplemented!()
                }
            }
        }
    } 


}