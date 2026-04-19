pub mod backends;
pub mod middle;
pub mod parser;

use std::env;
use parser::*;
use backends::node;
use std::io::Write;
use std::io::Read;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let code = if args.len() == 3 {
        let input = std::fs::read_to_string(&args[1])?;
        node::transpile_program(&parse(&input))
    } else if args.len() == 2 {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        node::transpile_program(&parse(&input))
    } else {
        eprintln!("Usage: prog [file_in] file_out");
        std::process::exit(1);
    };

    let out_path = args.last().unwrap();
    let mut file = File::create(out_path)?;
    file.write_all(code.as_bytes())?;

    Ok(())
}
