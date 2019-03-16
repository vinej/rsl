use std::time::{Instant};
use std::collections::HashMap;

type FunctionPtr = fn(u16, &mut Vec<ValueType>);
type CommandPtr = fn(u16, &mut Vec<ValueType>);

#[derive(Copy, Clone)]
enum ValueType {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    Index(i64)
}

enum TokenType {
    Command(u16, CommandPtr, u32),
    Function(u16 , FunctionPtr),
    Field( i16, ValueType, bool ),
    Value(ValueType)
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

fn fornext(n: u16, 
        stack: &mut Vec<ValueType>, 
        fields: &mut HashMap<i64, ( u32, ValueType, bool )>,
        jump: u32) -> u32 {
    let f = stack.pop();
    let min = stack.pop();
    let max = stack.pop();
    let inc = stack.pop();
    let jump = match f {
        Some(ValueType::Index(ref idx)) => {
            match fields.get(idx) {
                Some((idx, value, false)) => {
                    let min_value = match value { &ValueType::Integer(v) => v, _ => 0 };
                    fields.insert(*idx, (*idx, ValueType::Index(min_value), true));
                    0 },

                Some((idx, value, true)) => {
                    let f_value = match value { 
                        &ValueType::Integer(v) => v,
                        _ => 0
                    };

                    match (max,inc) {   
                        (Some(ValueType::Integer(max_value)), Some(ValueType::Integer(inc_value))) => {
                            if f_value >= max_value {
                                fields.insert(*idx, (*idx, 0, false));
                                jump                        
                            } else {
                                let total = f_value + inc_value;
                                fields.insert(*idx, (*idx, ValueType::Integer(total), true));
                                0
                            }},
                        _ => 0
                    }
                },
                _ => 0
            } 
        }
    };
    0
}   

struct Program {
    code : Vec<TokenType>,
    stack : Vec<ValueType>,
    fields : HashMap<u16,( u32, ValueType, bool )>
}

impl Program {
    fn exe(&mut self) {
        let now = Instant::now();
        for x in 1..100000 {
            for t in &self.code {
                match t {
                    &TokenType::Function(n, ptr) => (ptr)(n, &mut self.stack),
                    &TokenType::Command(n, ptr, jump) => (ptr)(n, &mut self.stack),
                    &TokenType::Field(id_name, value, false) => self.stack.push(value),
                    &TokenType::Value(value) => self.stack.push(value)
                };
            }
            self.stack.clear();
        }
        println!("Elapsed: {} ms", (now.elapsed().subsec_nanos() as f64 / 1000000.0) as f64);    
    }
}

fn main() {
    println!("start");
    let mut p = Program {
        code : Vec::with_capacity(200),
        stack : Vec::with_capacity(200),
        fields : HashMap::new()
    };

    p.code.push(TokenType::Value(ValueType::Inteter(1)));
    p.code.push(TokenType::Command(1, set, 1));

    p.code.push(TokenType::Field(ValueType::Inteter(1));
    p.code.push(TokenType::Value(ValueType::Inteter(1)));
    p.code.push(TokenType::Value(ValueType::Inteter(1000000)));
    p.code.push(TokenType::Command(3, for, 9));

    p.code.push(TokenType::Value(ValueType::Real(12.12)));
    p.code.push(TokenType::Value(ValueType::Real(12.12)));
    p.code.push(TokenType::Function(2, add));

    p.code.push(TokenType::Command(1, set, 0);
    p.code.push(TokenType::Function(1, set));

    p.code.push(TokenType::Command(2, next, 4));

    p.exe();
    println!("end");
}