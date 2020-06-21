extern crate os_project;
extern crate json;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use os_project::process_management::pcb;
use os_project::process_management::process_q;


fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut file = File::open(&args[1])?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    

    println!("{}",contents);
    Ok(())
}