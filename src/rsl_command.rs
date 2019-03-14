
mod rsl_command;

mod rsl_command {

	pub enum EnumCommand {
		For,
		Break,
		Continue,
		Next,
		If,
		Else,
		End,
		Exe,
		Def,
		Enddef,
	}

	fn for(context: &Program, stack : &mut Vec<Value>) -> Resultu32 {

	}

	fn break(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn continue(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn next(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn if(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn else(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn end(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn exe(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn def(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	fn enddef(context: &Program, stack : &mut Vec<Value>) -> u32 {
		
	}

	pub struct Command {
		id_command : EnumCommand,
		parameter : u16,         // number of parameters of the command
		ptr : CommandPtr,        // p64 function pointer
		id_jump : u16,		     // command can jump to another token in the program	
	}

	impl Command {
		pub fn exe(&self, context: &Program, stack: &mut Vec<Value>) {
			match self.id_command {
				EnumCommand::For => for(context, stack),
				EnumCommand::Break => break(context, stack),
				EnumCommand::Continue => continue(context, stack),
				EnumCommand::Next => next(context, stack),
				EnumCommand::If => if(context, stack),
				EnumCommand::Else => else(context, stack),
				EnumCommand::End => end(context, stack),
				EnumCommand::Exe => exe(context, stack),
				EnumCommand::Def => def(context, stack),
				EnumCommand::Enddef => enddef(context, stack)
			}
		}
	}
}