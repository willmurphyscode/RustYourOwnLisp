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
   pub values : Vec<Rc<SExpression>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum SExpression {
    atomic(f64),
    op(Operation)
}

impl Operation {
    pub fn eval(&self) -> f64 {
        match self.opcode {
            OpCode::Add => {
                let mut result = 0f64; 
                for boxed_s_expr in self.values.iter() {
                    let current = Rc::try_unwrap(boxed_s_expr).expect("oops");
                    // let temp : f64 = current.eval();
                    // result = result + temp; 
                }
                result
            }
            _ => unimplemented!()
        }
    }
}

impl SExpression {
    pub fn eval(&self) -> f64 {
        match self {
            &SExpression::atomic(val) => val,
            &SExpression::op(operation) => {
                let my_op = operation.clone();
                my_op.eval() 
            }
        }
    } 


}