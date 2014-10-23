#![feature(globs)]
#![feature(quote)]

extern crate syntax;

use std::str;
use std::io::{stdout,stderr};
use std::io::fs::File;
use std::os;
use translate::{compile_grammar};

mod translate;
mod grammar;
mod rustast;
mod fake_extctxt;

fn main() {
	let args = os::args();
	let source_utf8 = File::open(&Path::new(args[1].as_slice())).read_to_end().unwrap();
	let source = str::from_utf8(source_utf8.as_slice()).unwrap();
	let grammar_def = grammar::grammar(source);

	match grammar_def {
		Ok(grammar) => {
			fake_extctxt::with_fake_extctxt(|e| {

				let ast = compile_grammar(e, &grammar);
				let mut out = stdout();

				out.write_line("// Generated by rust-peg. Do not edit.").unwrap();
				out.write_line("#![allow(non_snake_case, unused)]").unwrap();

				for item in ast.view_items.iter() {
					out.write_line(rustast::view_item_to_string(item).as_slice()).unwrap();
				}

				for item in ast.items.iter() {
					out.write_line(rustast::item_to_string(&**item).as_slice()).unwrap();
				}
			})
		}

		Err(msg) => {
			(writeln!(&mut stderr() as &mut Writer, "Error parsing language specification: {}", msg)).unwrap();
			os::set_exit_status(1);
		}
	}
}
