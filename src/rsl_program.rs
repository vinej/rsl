
mod rsl_program;

mod rsl_program {
	use rsl_token::{Program, Token, Scalar };



	impl Program {
		pub fn peek(i: u32)	-> &Token {}

		pub fn next() -> &Token	{}

		pub fn load(file_name: &String) {}

		pub fn exe(&self) {
			match self.next_token.token {
				TokenType::Function( id_name, parameter, ptr) => {}
				TokenType::Command( id_name, parameter, ptr, jmp) => {}
				TokenType::Field(id_name, value) => {}
				TokenType::Constant(id_name, value) => {}
				TokenType::Value(value) =>  {}
			}
		}
	}
}