use std::env;
use std::fs::{File};
use std::io::{self, BufRead, BufReader};
use std::os::unix::fs::FileExt;

fn read_path(path_ :&str) -> io::Result<()>{
    let file = File::open(path_)?;
    let reader = BufReader::new(file);

    for line in reader.lines(){
        println!("{}", line?);
    }
    Ok(()) 
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Please enter a file name");
        return Ok(());

    }
    let file_name = &args[1];
    read_path(file_name)?;


    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);
    Ok(())
}
