use std::fs::File;
use std::process;
use std::env;
fn main(){ 

    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        eprintln!("usage: {} fileName", args[0]);
        process::exit(1);
    }

    match File::create(&args[1]){
        Ok(_)=> println!("The file has been created"),
        Err(e) => eprintln!("Something went wrong {} " , e), 
    } 
    
}
