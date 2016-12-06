enum S_Expression {
    Atom(Atom),
    Pair(Pair)
}

enum Atom {
    Opcode(OpCode),
    Value(f32)
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

