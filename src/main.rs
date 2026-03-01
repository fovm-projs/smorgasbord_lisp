pub mod backends;
pub mod parser;

use std::env;
use parser::*;
use backends::node;
use std::io::Write;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let exm = "(write 0 (get_environment \"0\")) ; уже есть комментарии";
    println!("{:?}", exm);
    let code = node::transpile_program(&parse(exm));
    println!("{:?}", code);

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut file = File::create(args[1].clone())?;
        file.write_all(&code.into_bytes());
    }

    Ok(())
}
