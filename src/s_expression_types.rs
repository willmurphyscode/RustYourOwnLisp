enum S_Expression {
    Atom(Atom),
    Pair(Pair)
}

enum Atom {
    Opcode(OpCode),
    Value(f32),
    Nil
}

struct Pair {
    left: Option<Box<S_Expression>>,
    right: Option<Box<S_Expression>>,
}

enum OpCode {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl S_Expression {
    pub fn eval(&mut self, &mut Box<f32> accumulator) {
        match(self) {
            Atom(a) => match(a) {
                
            }
        }
    }
}
