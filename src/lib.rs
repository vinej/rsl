type FunctionPtr = fn(stack : &mut Vec<Field>) -> i32;
type CommandPtr = fn(stack : &mut Vec<Field>) -> u32;

// set, get
enum ValueType {
	Str { value: String },
	Boolean { value: bool },
	Integer { value: i64 },
	Real { value: f64},
	Vector { value: Vec<f64> },
	VectorInt { value: Vec<i64> },
}

struct Scalar {
	value: ValueType 		// a value
}

struct Field {
	id_name : u16,		     // Id of the field name, debugging purpose
	value: ValueType 
}

struct Function {
	id_name : u16,		     // Id of the function name, debugging purpose
	parameter : u16,         // number of parameter of the function/cmd
	ptr : FunctionPtr,       // p64 function pointer
}

struct Command {
	id_name : u16,		     // u32 Id of the function, debugging purpose
	parameter : u16,         // number of parameters of the command
	ptr : CommandPtr,        // p64 function pointer
	id_jump : u16,		     // command can jump to another token in the program	
}

enum TokenType {
	Function { },
	Command { },
	Field { },
	Scalar { },
}

struct Token {
	line : u16,           // line into the source code where to token is for debugging purpose
	id_token : u16,       // the token number into the program 
	token : TokenType     // the token content 
}

struct Program {
	code : Vec<Token>,
	current_id_token : u16,
	stack : Vec<Scalar>,
	is_local : bool
}


