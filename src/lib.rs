enum BaseToken {
	Function { },
	Field { }
}

struct Token {
	line : u32,           // debug purpose
	id : u32,             // need for loop control 
	token : BaseToken     // a token type
}

type Program Vec<Token>

type FunctionPtr = fn(context, stack : mut &Vec<Field>) -> i32;

struct Function {
	Id : u32,		       // Id of the function, debugging purpose
	parameter usize,       // nomber of parameter of the function/cmd
	FunctionPtr ptr,       // function pointer
	JumpId u32		       // function can jump to another	
}

enum ValuePtr {
	Integer { value: i64 },
	Real { value: f64},
}

struct Field {
	Id : u32,		   // Id of the function, debugging purpose
	ValuePtr value
}

type Stack Vec<Field>

