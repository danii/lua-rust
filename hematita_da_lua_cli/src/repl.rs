use self::super::{compile, compile_expression, handle_io};
use hematita_da_lua::{
	ast::parser::Error,
	lua_lib::print,
	vm::{VirtualMachine, value::{NillableValue::{Nil, NonNil}, Table, Value}}
};
use std::io::{BufRead, Write, stdin, stdout};

pub fn repl(vm: VirtualMachine) {
	loop {
		let mut code = String::new();

		match loop {
			code.push_str(&{
				let stdin = stdin();
				let stdout = stdout();
				let mut stdin = stdin.lock();
				let mut stdout = stdout.lock();

				let prompt: &[u8] = if code.is_empty() {b"> "} else {b">> "};
				handle_io(stdout.write_all(prompt));
				handle_io(stdout.flush());

				let mut line = String::with_capacity(80);
				handle_io(stdin.read_line(&mut line));
				line
			});

			match compile_expression(&code).or_else(|_| compile(&code)) {
				Err(Error(None)) => (),
				Err(Error(Some(error))) => break Err(error),
				Ok(function) => break Ok(function)
			}
		} {
			Err(error) => eprintln!("syntax error: unexpected {}", error),
			Ok(function) => match vm.execute(&function, Table::default().arc()) {
				Ok(output) => match output.index(&Value::Integer(1)) {
					NonNil(_) => {print(output, vm.global.clone()).unwrap();},
					Nil => ()
				},
				Err(error) => eprintln!("runtime error: {}", error)
			}
		}
	}
}