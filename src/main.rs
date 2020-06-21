extern crate os_project;
extern crate json;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use os_project::process_management::pcb;
use os_project::process_management::process_q;
extern crate text_io;

use std::collections::HashMap;

fn user_input_pcb(pid_set :& mut HashMap<usize, String>) -> pcb::Pcb {
    let mut block = pcb::new(0,0,0,0);
    
    println!("Enter PID (positive integer)");
    let mut pid_string : String = text_io::read!();
    let mut pid = match pid_string.parse::<usize>() {
        Ok(i) => i,
        Err(_e) => {
            pid_string = "invalid".to_string();
            0
      }};
    
      while pid_string == "invalid" || pid_set.contains_key(&pid) {
        if pid_string == "invalid" {
            println!("Please enter a positive integer");
            println!("{}", pid_string);
            pid_string = text_io::read!();
            pid = match pid_string.parse::<usize>() {
                Ok(i) => i,
                Err(_e) => {
                    pid_string = "invalid".to_string();
                    0
            }};
        } else {
            println!("Pid Already Exists Please Enter Another");
            pid_string = text_io::read!();
            pid = match pid_string.parse::<usize>() {
                Ok(i) => i,
                Err(_e) => {
                    pid_string = "invalid".to_string();
                    0
            }};
        }
    }

    block.set_pid(pid);

    println!("Enter State (positive int)");
    let mut state_string = "invalid".to_string();
    let mut state: usize = 0;
    while state_string == "invalid" {
        state_string = text_io::read!();
        state = match state_string.parse::<usize>(){
            Ok(i) => i,
            Err(_e) => {
                println!("Please enter a positive integer");
                state_string = "invalid".to_string();
                0
            }
        };
    }   

    block.set_state(state);

    println!("Enter Priority (positive int)");
    state_string = "invalid".to_string();
    state = 0;
    while state_string == "invalid" {
        state_string = text_io::read!();
        state = match state_string.parse::<usize>(){
            Ok(i) => i,
            Err(_e) => {
                println!("Please enter a positive integer");
                state_string = "invalid".to_string();
                0
            }
        };
    }   
    block.set_priority(state);

    println!("Enter Program Counter(int)");
    state_string = "invalid".to_string();
    state = 0;
    while state_string == "invalid" {
        state_string = text_io::read!();
        state = match state_string.parse::<usize>(){
            Ok(i) => i,
            Err(_e) => {
                println!("Please enter a positive integer");
                state_string = "invalid".to_string();
                0
            }
        };
    }   
    block.set_program_counter(state);

    println!("Add Memory Addresses (y/n)");
    let mut choice : String = text_io::read!();
    while choice == "y".to_string() {
        println!("Add Memory Address (int)");
        state_string = "invalid".to_string();
        state = 0;
        while state_string == "invalid" {
            state_string = text_io::read!();
            state = match state_string.parse::<usize>(){
                Ok(i) => i,
                Err(_e) => {
                    println!("Please enter a positive integer");
                    state_string = "invalid".to_string();
                    0
                }
        };}
      
        block.add_memory_address(state);

        println!("Continue Adding Memory Addresses (y/n)");
        choice = text_io::read!();
    }

    println!("Add Register (y/n)");
    let mut choice : String = text_io::read!();
    while choice == "y".to_string() {
        println!("Add Register (int)");
        state_string = "invalid".to_string();
        state = 0;
        while state_string == "invalid" {
            state_string = text_io::read!();
            state = match state_string.parse::<usize>(){
                Ok(i) => i,
                Err(_e) => {
                    println!("Please enter a positive integer");
                    state_string = "invalid".to_string();
                    0
                }
        };}
      
        block.add_memory_address(state);

        println!("Continue Adding Registers (y/n)");
        choice = text_io::read!();
    }

    return block;
    
}

fn add(q_set : & mut HashMap<String, process_q::Processq>, pid_set :& mut HashMap<usize,String>){
    let mut input : String = "".to_string();
    while input != "back" {
        if q_set.len() != 0 {
            println!("What queue do you want to add too?");
            for qname in q_set.keys() {
                println!("{}",qname);
                }
            input = text_io::read!();
            if q_set.contains_key(&input){
                let pcb = user_input_pcb(pid_set);
                pid_set.insert(pcb.get_pid(), input.to_string());
                q_set.get_mut(&input).unwrap().add_pcb(pcb);
            } else {
                println!("Queue Doesn't Exist");
            }
        } else {
            println!("There is no queue right now")
        }
        println!("Type back to go back, Type Continue to keep adding");
        input = text_io::read!();
    }
}

fn peek(q_set : & mut HashMap<String, process_q::Processq>){
    if q_set.len()!=0{
    let mut input : String = "".to_string();
    while input != "back" {
            println!("What queue do you want to peek the top?");
            for qname in q_set.keys() {
                println!("{}",qname);
                }
            input = text_io::read!();
            if q_set.contains_key(&input) {
                println!("{}", q_set.get_key_value(&input).unwrap().1.peek().to_string());
            }
         
        println!("Type back to go back, Type Continue to keep peeking");
        input = text_io::read!();
        }
    } else {
        println!("There are no queues");
    }
}

fn create (q_set : & mut HashMap<String, process_q::Processq>,verbose : bool) {
    let mut input : String = "y".to_string();
    while input == "y" {
        println!("Enter a queue name");
        input = text_io::read!();
        if q_set.contains_key(&input){
            println!("Queue already exists");
        } else {
            let mut q = process_q::new();
            if verbose {
                q.set_verbose(input.clone());
            }
            q_set.insert(input, q);
        }
        println!("Continue (y/n)");
        input = text_io::read!();
    }
}
fn set_verbose(q_set : & mut HashMap<String, process_q::Processq>) {
    for (key,value) in q_set{
        println!("Setting {} to verbose", key);
        value.set_verbose(key.to_string());
    }
}
fn find(q_set : & mut HashMap<String, process_q::Processq>, pid_set :& mut HashMap<usize,String>) {
    if pid_set.len()!=0{
        println!("Enter a PID (positive integer)");
        let mut input : String = text_io::read!();
        let mut input_usize : usize =match input.parse::<usize>() {
            Ok(i) => i,
            Err(_e) => {
                println!("Enter a PID (positive integer)");
                input = "invalid".to_string();
                0
        }};
        while input == "invalid" ||  !pid_set.contains_key(&input_usize) {
            input = text_io::read!();
            input_usize = match input.parse::<usize>() {
                Ok(i) => i,
                Err(_e) => {
                    println!("Enter a PID (positive integer)");
                    input = "invalid".to_string();
                    0
            }};
        }
        println!("In {}", pid_set.get(&input_usize).unwrap());
        println!("{}", q_set.get_mut(pid_set.get(&input_usize).unwrap()).unwrap().peek_pid(input_usize).to_string());
    } else {
        println!("There is no process currently");
    }    
}

fn remove(q_set : & mut HashMap<String, process_q::Processq>, pid_set :& mut HashMap<usize,String>) {
    if pid_set.len()!=0 {
        println!("Enter a PID (positive integer)");
        let mut input : String = text_io::read!();
        let mut input_usize : usize =match input.parse::<usize>() {
            Ok(i) => i,
            Err(_e) => {
                println!("Enter a PID (positive integer)");
                input = "invalid".to_string();
                0
        }};
        while input == "invalid" ||  !pid_set.contains_key(&input_usize) {
            input = text_io::read!();
            input_usize = match input.parse::<usize>() {
                Ok(i) => i,
                Err(_e) => {
                    println!("Enter a PID (positive integer)");
                    input = "invalid".to_string();
                    0
            }};
        }
        q_set.get_mut(pid_set.get(&input_usize).unwrap()).unwrap().pop_pid(input_usize);
        pid_set.remove(&input_usize);
    } else {
        println!("There are no processes currently");
    }   
}

fn move_process(q_set : & mut HashMap<String, process_q::Processq>, pid_set :& mut HashMap<usize,String>) {
    if pid_set.len()!= 0 {
        println!("Enter a PID (positive integer)");
        let mut input : String = text_io::read!();
        let mut input_usize : usize =match input.parse::<usize>() {
            Ok(i) => i,
            Err(_e) => {
                println!("Enter a PID (positive integer)");
                input = "invalid".to_string();
                0
        }};
        while input == "invalid" ||  !pid_set.contains_key(&input_usize) {
            input = text_io::read!();
            input_usize = match input.parse::<usize>() {
                Ok(i) => i,
                Err(_e) => {
                    println!("Enter a PID (positive integer)");
                    input = "invalid".to_string();
                    0
            }};
        }
        let pcb = q_set.get_mut(pid_set.get(&input_usize).unwrap()).unwrap().pop_pid(input_usize).clone();

        
        while input != "back" {
            if q_set.len() != 0 {
                println!("What queue do you want to move the block into");
                for qname in q_set.keys() {
                    println!("{}",qname);
                    }
                input = text_io::read!();
                if q_set.contains_key(&input) {
                    q_set.get_mut(&input).unwrap().add_pcb(pcb.clone());
                    
                    pid_set.remove(&input_usize);
                    pid_set.insert(input_usize, input);
                }
            } else {
                println!("There is no queue right now")
            }
            println!("Type back to go back, Type Continue to keep adding");
            input = text_io::read!();
        } }
    else {
        println!("There are no processes currenlty");
    }
     
}

fn print_q(q_set : & mut HashMap<String, process_q::Processq>){
    for (key,value) in q_set{
        println!("Name: {}", key);
        println!("{}",json::stringify_pretty(value.to_json(), 4));
    }
}

fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    

    let mut q_set = HashMap::<String, process_q::Processq>::new();
    let mut pid_set = HashMap::<usize, String>::new();
    
    let mut verbose = false;
    if args.len() > 1 && args[1] != "-v" {
        let mut file = File::open(&args[1])?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let json_contents = json::parse(&contents).unwrap_or(json::JsonValue::Null);
        

        if json_contents!= json::JsonValue::Null {
        let mut index = 0;
        let length = json_contents.len();
        
        if args.len() > 2 && &args[2] == "-v"{
            verbose = true;
        }

        

        while index != length {
            let mut pcb = pcb::new(0,0,0,0);
            if pid_set.contains_key(&json_contents[index]["pid"].as_usize().unwrap()){
                println!("ERROR PID {} Already Exists",json_contents[index]["pid"] );
            } else {
                pcb.set_pid(json_contents[index]["pid"].as_usize().unwrap());
                pcb.set_priority(json_contents[index]["priority"].as_usize().unwrap());
                pcb.set_program_counter(json_contents[index]["state"].as_usize().unwrap());
                pcb.set_state(json_contents[index]["program_counter"].as_usize().unwrap());

                if !q_set.contains_key(json_contents[index]["queue"].as_str().unwrap()) {
                    let mut new = process_q::new();
                    if verbose {
                        new.set_verbose(json_contents[index]["queue"].as_str().unwrap().to_string());
                    }
                    q_set.insert(json_contents[index]["queue"].as_str().unwrap().to_string(), new);
                }
                

                if q_set.contains_key(json_contents[index]["queue"].as_str().unwrap()){
                    pid_set.insert(json_contents[index]["pid"].as_usize().unwrap(),json_contents[index]["queue"].as_str().unwrap().to_string());
                    q_set.get_mut(json_contents[index]["queue"].as_str().unwrap()).unwrap().add_pcb(pcb);
                }
            }

            index+=1;
            }}
    }
        
        let mut input : String = "".to_string();
        
        while input != "quit"{
            
            println!("add to add a process control block");
            println!("peek view the top of a queue");
            println!("create to create a queue");
            println!("verbose to change to verbose mode to see excecution results");
            println!("find to see print out a certain PID");
            println!("remove to remove a certain PID");
            println!("move to move a process into a different Q");
            println!("print to print a specific Q");
            println!("quit to quit");
            
            input = text_io::read!();
            if input == "add" {
                add(& mut q_set,& mut pid_set);
            } else if input == "peek" {
                peek(& mut q_set);
            } else if input == "create" {
                create(& mut q_set, verbose);
            } else if input == "verbose" {
                set_verbose(& mut q_set);
                verbose = true;
            } else if input == "find" {
                find(& mut q_set, & mut pid_set);
            } else if input == "remove" {
                remove(& mut q_set, & mut pid_set);
            } else if input == "move" {
                move_process(& mut q_set, & mut pid_set);
            } else if input == "print" {
                print_q(&mut q_set);
            }
        }
        

        Ok(())
}