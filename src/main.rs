use std::fs::File;
use std::io::Result;
use std::process;
use std::env;
fn main() -> Result<()>{

    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("usage: {} fileName", args[0]);
        process::exit(1);
    }

    let mut f = File::create(args[1].to_string())?; 
    Ok(())
}
