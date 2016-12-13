use std::rc::Rc;

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
   pub values : Vec<Box<SExpression>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum SExpression {
    atomic(f64),
    op(Operation)
}

pub struct Visitor; 

impl Visitor {
    pub fn visit_s_expression(exp : &SExpression) -> f64 {
        match *exp {
            SExpression::atomic(val) => val,
            SExpression::op(ref operation) => {
                match operation.opcode {
                    OpCode::Add => {
                        let floats: Vec<_> = operation.values.iter().map(|val| Visitor::visit_s_expression(&val)).collect(); 
                        floats.iter().fold(0f64, |sum, x| sum + x)
                    },
                    OpCode::Subtract => {
                        let floats: Vec<_> = operation.values.iter().map(|val| Visitor::visit_s_expression(&val)).collect(); 
                        floats.iter().fold(0f64, |diff, x| diff - x)
                    },
                    OpCode::Multiply => {
                        let floats: Vec<_> = operation.values.iter().map(|val| Visitor::visit_s_expression(&val)).collect(); 
                        floats.iter().fold(1f64, |prod, x| prod * x)
                    },
                    OpCode::Divide => {
                        let floats: Vec<_> = operation.values.iter().map(|val| Visitor::visit_s_expression(&val)).collect(); 
                        floats.iter().fold(1f64, |div, x| div / x)
                    }

                }
            }
        }
    }
}
