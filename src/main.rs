use std::time::{Instant};

type FunctionPtr = fn(u16, &mut Vec<ValueType>);
type CommandPtr = fn(u16, &mut Vec<ValueType>);

#[derive(Copy, Clone)]
enum ValueType {
    Boolean(bool),
    Integer(i64),
    Real(f64)
}

enum TokenType {
    Command {  id_name : i16,  parameter : u16, ptr : CommandPtr, jump : u32 },
    Function{  id_name : i16, parameter : u16 ,  ptr : FunctionPtr},
    Constant { id_name : i16,  value: ValueType },
    Field {    id_name : i16,  value: ValueType },
    Value { value: ValueType }
}

struct Token {
    line : u16,           // line into the source code where to token is for debugging purpose
    id_token : u16,       // the token number into the program 
    token : TokenType     // the token content 
}

fn add(n: u16, stack: &mut Vec<ValueType>) {
    let p1 = stack.pop();
    let p2 = stack.pop();
    let total = match (p1,p2) {
        (Some(ValueType::Real(v1)), Some(ValueType::Real(v2))) => v1 + v2,
        _ => 0.0
    };
    //println!("{}",total);
    stack.push( ValueType::Real(total));
}

struct Program {
    code : Vec<Token>,
    index : usize,
    stack : Vec<ValueType>,
}

impl Program {
    fn exe(&mut self) {
        let now = Instant::now();
        for x in 1..1000000 {
            for t in &self.code {
                match t.token {
                    TokenType::Function{ id_name, parameter, ptr } => (ptr)(parameter, &mut self.stack),
                    TokenType::Command{ id_name, parameter, ptr, jump } => (ptr)(parameter, &mut self.stack),
                    TokenType::Constant{ id_name, value } => self.stack.push(value),
                    TokenType::Field{ id_name, value } => self.stack.push(value),
                    TokenType::Value{ value } => self.stack.push(value)
                };
            }
        }
        self.stack.clear();
        println!("Elapsed: {} ms", (now.elapsed().as_secs() * 1_000) + (now.elapsed().subsec_nanos() / 1_000_000) as u64)    }
}

fn main() {
    let mut p = Program {
        code : Vec::with_capacity(200),
        index : 0,
        stack : Vec::with_capacity(200)
    };

    let mut f2 = TokenType::Value { value : ValueType::Real(12.12) };
    let mut t2 = Token { line : 0,  id_token : 0,  token: f2  };
    p.code.push(t2);

    f2 = TokenType::Value { value : ValueType::Real(12.12) };
    t2 = Token { line : 0,  id_token : 0,  token: f2  };
    p.code.push(t2);

    f2 = TokenType::Function { id_name : 1,  parameter : 2,  ptr : add  };
    t2 = Token { line : 0,  id_token : 0,   token: f2  };
    p.code.push(t2);
    p.exe();
}