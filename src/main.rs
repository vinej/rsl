use std::time::{Instant};
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum ValueType {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Field(i32),
}

type FunctionPtr = fn(u16, &mut Vec<ValueType>);
type CommandPtr = fn(&mut HashMap<i32,ValueType>, &mut Vec<ValueType>, u16, usize) -> usize;

enum TokenType {
    Command(u16, CommandPtr, usize),
    Function(u16 , FunctionPtr),
    Value(ValueType),
}

fn add(n_args: u16, stack: &mut Vec<ValueType>) {
    let p1 = stack.pop();
    let p2 = stack.pop();

    let total = match (p1,p2) {
        (Some(ValueType::Real(v1)), Some(ValueType::Real(v2))) => v1 + v2,
        (Some(ValueType::Integer(v1)), Some(ValueType::Real(v2))) => v1 as f64 + v2,
        (Some(ValueType::Integer(v1)), Some(ValueType::Integer(v2))) => v1 as f64 + v2 as f64,
        (Some(ValueType::Real(v1)), Some(ValueType::Integer(v2))) => v1 + v2 as f64,
        _ => 0.0
    };
    //println!("{}",total);
    stack.push( ValueType::Real(total));
}

fn ge(n_args: u16, stack: &mut Vec<ValueType>) {
    let p1 = stack.pop();
    let p2 = stack.pop();

    let total = match (p1,p2) {
        (Some(ValueType::Real(v1)), Some(ValueType::Real(v2))) => v2 >= v1,
        (Some(ValueType::Integer(v1)), Some(ValueType::Real(v2))) => v2 >=  v1 as f64,
        (Some(ValueType::Integer(v1)), Some(ValueType::Integer(v2))) => v2 as f64 >= v1 as f64,
        (Some(ValueType::Real(v1)), Some(ValueType::Integer(v2))) => v2 as f64 >= v1,
        _ => true
    };
    //println!("{}",total);
    stack.push( ValueType::Boolean(total));
}

fn print(n_args: u16, stack: &mut Vec<ValueType>) {
    let p1 = stack.pop().unwrap();

    match p1 {
        ValueType::Real(v1) => println!("{}",v1),
        ValueType::Boolean(v1) => println!("{}",v1),
        ValueType::Integer(v1) => println!("{}",v1),
        ValueType::Field(v1) => println!("{}",v1),
    };
}

struct Program {
    code : Vec<TokenType>,
    stack : Vec<ValueType>,
    local_idx : i32,
    fields : HashMap<i32,ValueType>
}

fn get(fields: &mut HashMap<i32,ValueType>, stack: &mut Vec<ValueType>, n_agrs: u16, jump: usize) -> usize {
    let f = stack.pop().unwrap();
    let mut idx = match f { ValueType::Field(x) => x, _ => 0 };
    if idx < 0 {
        idx = idx + 0;
    }
    let value = fields.get(&idx).unwrap();
    stack.push(*value);
    //println!("{}",value);
    0
}

fn set(fields: &mut HashMap<i32,ValueType>, stack: &mut Vec<ValueType>, n_agrs: u16, jump: usize) -> usize {
    let v = stack.pop().unwrap();
    let f = stack.pop().unwrap();
    let idx = match f { ValueType::Field(idx) => idx, _ => 0 };
    fields.insert(idx, v );
    //println!("{}",v);
    0
}

fn jump(fields: &mut HashMap<i32,ValueType>, stack: &mut Vec<ValueType>, n_agrs: u16, jump: usize) -> usize {
    let v = stack.pop().unwrap();
    let is_jump = match v { ValueType::Boolean(b) => b, _ => false };
    //println!("{}",jump);

    if is_jump == true {
        jump
    } else {
        0
    }
}    

fn next(fields: &mut HashMap<i32,ValueType>, stack: &mut Vec<ValueType>, n_agrs: u16, jump: usize) -> usize {
    //println!("{}",jump);
    jump
}   

fn push(fields: &mut HashMap<i32,ValueType>, stack: &mut Vec<ValueType>, value: ValueType) {
    //println!("{}",value);
    stack.push(value);
}   

impl Program {
    pub fn exe(&mut self) {
        let now = Instant::now();
        let mut i = 0usize;
        while i < self.code.len() {
            let t = &self.code[i];
            let next_i = i + 1;
            let mut jump_i = next_i;
            {
                match t {
                    &TokenType::Function(n_agrs, ptr) => (ptr)(n_agrs, &mut self.stack),
                    &TokenType::Command(n_agrs, ptr, jump) => { jump_i = (ptr)(&mut self.fields, &mut self.stack, n_agrs, jump); }
                    &TokenType::Value(value) => push(&mut self.fields, &mut self.stack, value)
                }
            }
            if jump_i != 0 {
                i = jump_i;
            } else {
                i = next_i;
            }
        }
        println!("Elapsed: {} ms", (now.elapsed().subsec_nanos() as f64 / 1000000.0) as f64);    
    }
}

fn main() {
    println!("start");
    let mut p = Program {
        code : Vec::with_capacity(200),
        stack : Vec::with_capacity(200),
        fields : HashMap::new(),
        local_idx : 0
    };

    // j = 1
    // 0
    p.code.push(TokenType::Value(ValueType::Field(1)));   //  1 == j            
    // 1
    p.code.push(TokenType::Value(ValueType::Integer(1)));
    // 2
    p.code.push(TokenType::Command(2, set, 0)); 

    // for(i,1,100000,1)
    // i = 1
    // 3
    p.code.push(TokenType::Value(ValueType::Field(2)));  // 2 = i              
    // 4
    p.code.push(TokenType::Value(ValueType::Integer(1)));    
    // 5
    p.code.push(TokenType::Command(2, set, 0));             

    // if i >= 1000000 jump after next
    // 6
    p.code.push(TokenType::Value(ValueType::Field(2)));  // 2 = i              
    // 7
    p.code.push(TokenType::Command(1, get, 0));             
    // 8
    p.code.push(TokenType::Value(ValueType::Integer(1000000)));
    // 9
    p.code.push(TokenType::Function(2, ge));               
    // 10
    p.code.push(TokenType::Command(1, jump, 25));              

    // i = i + 1
    // 11
    p.code.push(TokenType::Value(ValueType::Field(2)));  // 2 = i              
    // 12
    p.code.push(TokenType::Value(ValueType::Field(2)));  // 2 = i              
    // 13
    p.code.push(TokenType::Command(1, get, 0));               
    // 14
    p.code.push(TokenType::Value(ValueType::Integer(1)));     
    // 15
    p.code.push(TokenType::Function(2, add));                 
    // 16
    p.code.push(TokenType::Command(2, set, 0));               

    // j = j + i
    // 17
    p.code.push(TokenType::Value(ValueType::Field(1)));  //  1 == j         
    // 18
    p.code.push(TokenType::Value(ValueType::Field(1)));  //  1 == j         
    // 19
    p.code.push(TokenType::Command(1, get, 0));               
    // 20
    p.code.push(TokenType::Value(ValueType::Field(2)));  //  2 == i         
    // 21
    p.code.push(TokenType::Command(1, get, 0));               
    // 22
    p.code.push(TokenType::Function(2, add));              
    // 23
    p.code.push(TokenType::Command(1, set, 0));             

    // next
    // 24
    p.code.push(TokenType::Command(0, next, 6));           

    // print(j)
    // 25
    p.code.push(TokenType::Value(ValueType::Field(1)));  //  1 == j     
    // 26
    p.code.push(TokenType::Command(1, get, 0));             
    // 27
    p.code.push(TokenType::Function(1, print));            

    p.exe();
    println!("end");
}