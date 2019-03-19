use std::time::{Instant};

type CommandPtr = fn(&mut Context, u16) ;

enum TokenType {
    Command(u16, CommandPtr, usize),
    Value(f64),
    Field(f64)
}

macro_rules! push {
    ($c:expr, $v:expr) => {{
        $c.top += 1;
        $c.stack[$c.top] = $v;
    }};
}

macro_rules! pop {
    ($c:expr) =>  { { let top = $c.top; $c.top -= 1; $c.stack[top] } }
}

/*
macro_rules! peek {
    ($c:expr, $i:expr) =>  { { $c.stack[top - $i] } }
}

macro_rules! dectop {
    ($c:expr, $i:expr) =>  { { $c.top = $c.top - $i } }
}
*/

fn add(context :&mut Context, _: u16) {
    //assert_eq!(n,2);
    let p1 = pop!(context);
    let p2 = pop!(context);
    push!(context, p1+p2);
}

fn print(context :&mut Context, n: u16)  {
    assert_eq!(n,1);
    println!("{}",pop!(context));
}

fn get(context :&mut Context, n: u16)  {
    assert_eq!(n,1);
    let mut idx = pop!(context) as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    push!(context, context.fields[idx as usize]);
}

fn set(context :&mut Context, n: u16)  {
    assert_eq!(n,2);

    let v = pop!(context);
    let mut idx = pop!(context) as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    context.fields[idx as usize] = v;
}

fn ifor(context :&mut Context, n: u16)  {
    assert_eq!(n,4);
    let fields = &mut context.fields;

    let inc = pop!(context);
    let max = pop!(context);
    let min = pop!(context);;
    let mut idx = pop!(context) as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }    
    fields[idx as usize] = min;
    context.jump = 0;
    context.forstk.push( (idx,max,inc) );
}

fn nfor(context :&mut Context, _: u16)  {
    {
        let idxmaxinc = context.forstk.first().unwrap();
        let fields = &mut context.fields;
        let fv = fields[idxmaxinc.0 as usize];
        
        if fv < idxmaxinc.1 {
            fields[idxmaxinc.0 as usize] = fv + idxmaxinc.2;
            context.jump = 0;
            return;
        }
    }
    context.forstk.pop();        
}

fn next(_ : &mut Context, _: u16)  { }   

fn jump(_ : &mut Context, _: u16)  { }   

fn push(context : &mut Context, value: f64) {
    let mut idx = value as i32;
    if idx < 0 {
        idx = idx + context.local_idx;
    }
    push!(context, context.fields[idx as usize]); 
}

impl Program {
    pub fn exe(&mut self) {
        let now = Instant::now();
        let mut i = 0usize;
        let len = self.code.len();
        let mut exit_loop = false;
        while exit_loop == false {
            exit_loop = true;
            for t in &self.code[i..len] {
                match t {
                    &TokenType::Value(value) => push!(self.context, value),
                    &TokenType::Command(n, ptr, jump) => { self.context.jump = jump; (ptr)(&mut self.context, n); }
                    &TokenType::Field(value) => { push(&mut self.context, value) }
                }
                if self.context.jump != 0 {
                    //println!("jump {}",jump_i);
                    i = self.context.jump;
                    self.context.jump = 0;
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
    stack :  Box<[f64;200]>,
    fields : Box<[f64;200]>,
    forstk : Vec<(i32,f64,f64)>,
    top : usize,
    jump : usize
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
            stack : Box::new([0.0;200]),
            fields : Box::new([0.0;200]),
            forstk : Vec::with_capacity(20),
            local_idx : 0,
            top : 0,
            jump : 0
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
    p.code.push(TokenType::Command(4, ifor, 17));
    // 11             
    p.code.push(TokenType::Command(0, jump, 13));             
    // 12
    p.code.push(TokenType::Command(0, nfor, 19));             
    // j = j + i
    // 13
    p.code.push(TokenType::Value(1.0));  //  1 == j         
    // 14
    p.code.push(TokenType::Field(1.0));  //  1 == j         
    // 15
    p.code.push(TokenType::Field(2.0));  //  2 == i         
    // 16
    p.code.push(TokenType::Command(2, add, 0));              
    // 17
    p.code.push(TokenType::Command(2, set, 0));             

    // next
    // 18
    p.code.push(TokenType::Command(0, next, 12));           

    // print(j)
    // 19
    p.code.push(TokenType::Value(1.0));  //  1 == j     
    // 20
    p.code.push(TokenType::Command(1, get, 0));             
    // 21
    p.code.push(TokenType::Command(1, print, 0));            

    p.exe();
    println!("end");
}