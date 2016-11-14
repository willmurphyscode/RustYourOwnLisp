

pub use self::opcode::OpCode;

pub mod opcode {

    #[derive(Debug, Copy, Clone)]
    enum OpCode {
        Add,
        Subtract,
        Multiply,
        Divide
    }
    impl OpCode {
        fn get_opcode(input: char) -> Option<OpCode> {
            match input {
                '+' => Some(OpCode::Add),
                '-' => Some(OpCode::Subtract),
                '*' => Some(OpCode::Multiply),
                '/' => Some(OpCode::Divide),
                _ => None
            }
        }

        fn get_char(input: OpCode) -> char {
            match input {
                OpCode::Add => '+',
                OpCode::Subtract => '-',
                OpCode::Multiply => '*',
                OpCode::Divide => '/'
            }
        }

    }
}