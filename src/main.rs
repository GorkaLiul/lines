use std::fs::{read};
use std::path::{absolute, PathBuf};
use std::process;
use std::str::from_utf8;
use std::error::Error;
use std::env::args;

fn main() -> Result<(), Box<dyn Error>>{
    let args : Vec<String> = args().collect();
    println!("{}", args[0]);
    if args.len() <= 1 {println!("file argument missing!");
        process::exit(0);
    } else if args.len() > 2 {println!("Too many arguments passed!");
        process::exit(0);}

    let  path = absolute(&args[1]).unwrap();
    count_lines(path)?;
    Ok(())
}

fn count_lines(path: PathBuf) -> Result<(), Box<dyn Error>>{

    let abs_path = absolute(path)?;
    println!("{:?}", abs_path);
    let file = read(&abs_path);
    match file {
        Ok(val) => {
            let content = from_utf8(&val)?; 

            let lines = content.lines().count();
            println!(" File: {:?} \n Content: {} \n Lines: {}", val, content, lines); 
        }, 
        Err(e) => eprint!("{e}"),
    }

    Ok(())
}
