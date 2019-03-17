use std::time::{Instant};

type FunctionPtr = fn(u16, &mut Vec<f64>);
type CommandPtr = fn(&mut Context, u16, usize) -> usize;

enum TokenType {
    Command(u16, CommandPtr, usize),
    Function(u16 , FunctionPtr),
    Value(f64),
    FieldGet(f64)
}

fn add(n: u16, stack: &mut Vec<f64>) {
    assert_eq!(n,2);
    let p1 = stack.pop().unwrap();
    let p2 = stack.pop().unwrap();
    stack.push(p1+p2);
}

fn print(n: u16, stack: &mut Vec<f64>) {
    assert_eq!(n,1);
    println!("{}",stack.pop().unwrap());
}

fn get(context :&mut Context, n: u16, jump: usize) -> usize {
    assert_eq!(n,1);
    let stack = &mut context.stack;
    let fields = &mut context.fields;

    let mut idx = stack.pop().unwrap() as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    let value = fields[idx as usize];
    stack.push(value);
    //println!("get");
    jump
}

fn set(context :&mut Context, n: u16, jump: usize) -> usize {
    assert_eq!(n,2);
    let stack = &mut context.stack;
    let fields = &mut context.fields;

    let v = stack.pop().unwrap();
    let mut idx = stack.pop().unwrap() as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    fields[idx as usize] = v;
    //println!("set");
    jump
}

fn cfor(context :&mut Context, n: u16, jump: usize) -> usize {
    assert_eq!(n,4);
    let stack = &mut context.stack;
    let fields = &mut context.fields;

    let inc = stack.pop().unwrap();
    let max = stack.pop().unwrap();
    let min = stack.pop().unwrap();
    let mut idx = stack.pop().unwrap() as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    let fv = fields[idx as usize];
    
    if fv == -1.0 {
        fields[idx as usize] = min;
        0
    } else {
        if fv >= max {
            jump
        } else {
            fields[idx as usize] = fv + inc;
            0
        }
    }
}

fn next(_ : &mut Context, n: u16, jump: usize) -> usize {
    assert_eq!(n,0);
    jump
}   

fn push(context : &mut Context, value: f64) {
    let mut idx = value as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    context.stack.push(context.fields[idx as usize]); 
}

impl Program {

    pub fn exe(&mut self) {
        let now = Instant::now();
        let mut i = 0usize;
        let len = self.code.len();
        let mut jump_i = 0;
        let mut exit_loop = false;
        while exit_loop == false {
            exit_loop = true;
            for t in &self.code[i..len] {
                match t {
                    &TokenType::Value(value) => self.context.stack.push(value),
                    &TokenType::Function(n, ptr) => (ptr)(n, &mut self.context.stack),
                    &TokenType::Command(n, ptr, jump) => { jump_i = (ptr)(&mut self.context, n, jump); }
                    &TokenType::FieldGet(value) => { push(&mut self.context, value) }
                }
                if jump_i != 0 {
                    //println!("jump {}",jump_i);
                    i = jump_i;
                    jump_i = 0;
                    exit_loop = false;
                    break;
                }
            }
        }
        println!("Elapsed: {} ms", (now.elapsed().subsec_nanos() as f64 / 1000000.0) as f64);    
    }
}

struct Context {
    local_idx : i32,
    stack : Vec<f64>,
    fields : Box<[f64;200]>,
}

struct Program {
    code : Vec<TokenType>,
    context : Context,
}

fn main() {
    println!("start");
    let mut p = Program {
        code : Vec::with_capacity(200),
        context : Context {
            stack : Vec::with_capacity(200),
            fields : Box::new([0.0;200]),
            local_idx : 0
        }
    };

    // j = 1
    // 0
    p.code.push(TokenType::Value(1.0));   //  1 == j            
    // 1
    p.code.push(TokenType::Value(1.0));
    // 2
    p.code.push(TokenType::Command(2, set, 0)); 

    // 3
    p.code.push(TokenType::Value(2.0));   //  1 == j            
    // 4
    p.code.push(TokenType::Value(-1.0));
    // 5
    p.code.push(TokenType::Command(2, set, 0)); 

    // for(i,1,100000,1)
    // i = 1
    // 6
    p.code.push(TokenType::Value(2.0));  // 2 = i              
    // 7
    p.code.push(TokenType::Value(1.0));    
    // 8
    p.code.push(TokenType::Value(1000000.0));    
    // 9
    p.code.push(TokenType::Value(1.0));    
    // 10
    p.code.push(TokenType::Command(4, cfor, 17));             

    // j = j + i
    // 11
    p.code.push(TokenType::Value(1.0));  //  1 == j         
    // 12
    p.code.push(TokenType::FieldGet(1.0));  //  1 == j         
    // 13
    p.code.push(TokenType::FieldGet(2.0));  //  2 == i         
    // 14
    p.code.push(TokenType::Function(2, add));              
    // 15
    p.code.push(TokenType::Command(2, set, 0));             

    // next
    // 16
    p.code.push(TokenType::Command(0, next, 6));           

    // print(j)
    // 17
    p.code.push(TokenType::Value(1.0));  //  1 == j     
    // 18
    p.code.push(TokenType::Command(1, get, 0));             
    // 19
    p.code.push(TokenType::Function(1, print));            

    p.exe();
    println!("end");
}