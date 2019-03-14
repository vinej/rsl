type FunctionPtr = fn();
type CommandPtr = fn();

// set, get
enum ValueType {
    Boolean{ value: bool},
    Integer{ value : i64},
    Real{ value : f64}
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

fn add() {
    println!("hello");
}

struct Program {
    code : Vec<Token>,
    index : usize,
    stack : Vec<ValueType>,
}

impl Program {
    fn exe(&self, prg: &Program) {
        for t in &prg.code {
            match t.token {
                TokenType::Function{ id_name, parameter, ptr } => (ptr)(),
                TokenType::Command{ id_name, parameter, ptr, jump } => (ptr)(),
                TokenType::Constant{ id_name, ref value } => self.print(value),
                TokenType::Field{ id_name, ref value } => self.print(value),
                TokenType::Value{ ref value } => self.print(value)
            };
        }
    }

    fn print(&self, v : &ValueType) {
        match v {
            &ValueType::Boolean{ value }  => println!("{}",value),
            &ValueType::Integer{ value }  => println!("{}",value),
            &ValueType::Real{ value }  => println!("{}",value),
        };
    }
}

fn main() {
    let mut p = Program {
        code : Vec::with_capacity(200),
        index : 0,
        stack : Vec::with_capacity(200)
    };

    let f = TokenType::Function { id_name : 1,  parameter : 2,  ptr : add  };
    let t = Token { line : 0,  id_token : 0,   token: f  };

    p.code.push(t);

    let f2 = TokenType::Value { value : ValueType::Real{ value: 12.12 } };
    let t2 = Token { line : 0,  id_token : 0,  token: f2  };

    p.code.push(t2);

    p.exe(&p);
}