mod rsl_token {
	pub type FunctionPtr = fn() -> i32;
	pub type CommandPtr = fn() -> u32;

	// set, get
	pub enum ValueType {
		Str { value: String },
		Boolean { value: bool },
		Integer { value: i64 },
		Real { value: f64},
		Vector { value: Vec<f64> },
		VectorInt { value: Vec<i64> },
	}

	pub struct Value {
		value: ValueType 
	}

	pub struct Constant {
		id_name : u16,		     // Id of the field name, debugging purpose
		value: ValueType 
	}

	pub struct Field {
		id_name : i16,		     // Id of the field name, debugging purpose
		value: ValueType 
	}

	pub struct Function {
		id_name : u16,		     // Id of the function name, debugging purpose
		parameter : u16,         // number of parameter of the function/cmd
		ptr : FunctionPtr,       // p64 function pointer
	}

	pub struct Command {
		id_name : u8,		     // u32 Id of the function, debugging purpose
		parameter : u8,         // number of parameters of the command
		ptr : CommandPtr,        // p64 function pointer
		id_jump : u32,		     // command can jump to another token in the program	
	}

	pub enum TokenType {
		Function( u8, u8, FunctionPtr),
		Command( u8, u8, CommandPtr, u32),
		Field(u16, ValueType),
		Constant(u16, ValueType),
		Value(ValueType),
	}

	pub struct Token {
		line : u16,           // line into the source code where to token is for debugging purpose
		id_token : u16,       // the token number into the program 
		token : TokenType     // the token content 
	}

	struct Program {
		code : Vec<Token>,
		index : u16,
		next_token : Token,
		stack : Vec<Scalar>,
	}

}

