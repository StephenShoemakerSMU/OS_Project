use crate::process_management::pcb;
extern crate priority_queue;
extern crate json;


use priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Processq{
    line : PriorityQueue<usize,usize>,
    pid_map : HashMap<usize, pcb::Pcb>,
    verbose : bool,
    name: String
}

pub fn new() -> Processq{
    return Processq{
        line : PriorityQueue::<usize,usize>::new(),
        pid_map: HashMap::<usize,pcb::Pcb>::new(),
        verbose: false,
        name: String::from("")
    }
}

impl Processq{
    
    pub fn add_pcb(&mut self, block: pcb::Pcb){
        if self.line.get(&block.get_pid()) != None{
            
        } else {
            if self.verbose {
                println!("{}: ",self.name);
                println!("Inserting Block:");
                println!("{}", block.to_json().dump());

            }
            self.line.push(block.get_pid(), block.get_priority());
            self.pid_map.insert(block.get_pid(), block.clone());
        }
    }

    pub fn delete_pid(&mut self, pid : usize){
        if self.verbose {
            println!("{}: ",self.name);
            println!("Removing {}",pid);
            println!("Block Info: {}",self.pid_map.get(&pid).unwrap().to_json())
        }
        self.line.remove(&pid);
        self.pid_map.remove(&pid);
    }

    pub fn pop_front(&mut self) -> pcb::Pcb{
        if self.line.len() == 0 {
            panic!();
        } else {

            let pid = self.line.pop().unwrap().0;
            if self.verbose {
                println!("{}: ",self.name);
                println!("Popping {}",pid);
                println!("Block Info: {}",self.pid_map.get(&pid).unwrap().to_json())
            }
            let output = self.pid_map.get(&pid).unwrap().clone();
            self.pid_map.remove(&pid);
            return output;
        }
    }

    pub fn peek(&self) -> pcb::Pcb{
        if self.line.len() == 0 {
            panic!();
        } else {

            let pid = self.line.peek().unwrap().0;
            if self.verbose {
                println!("{}: ",self.name);
                println!("Peeking {}",pid);
                println!("Block Info: {}",self.pid_map.get(&pid).unwrap().to_json())
            }
            let output = self.pid_map.get(&pid).unwrap().clone();
            return output;
        }
    }

    pub fn contains_pid(&self, pid:usize) -> bool{
        return self.line.get(&pid) != None;
    }

    pub fn pop_pid(&mut self, pid: usize) -> pcb::Pcb{
        self.line.remove(&pid);
        if self.verbose {
            println!("{}: ",self.name);
            println!("Popping {}",pid);
            println!("Block Info: {}",self.pid_map.get(&pid).unwrap().to_json())
        }
        let output = self.pid_map.get(&pid).unwrap().clone();
        self.pid_map.remove(&pid);
        return output;
    }

    pub fn peek_pid(&mut self, pid: usize) -> pcb::Pcb {
        if self.verbose {
            println!("{}: ",self.name);
            println!("Peeking {}",pid);
            println!("Block Info: {}",self.pid_map.get(&pid).unwrap().to_json())
        }
        let output = self.pid_map.get(&pid).unwrap().clone();
        
        return output;
    }


    pub fn to_json(&mut self) -> json::JsonValue {
        let mut output = Vec::<json::JsonValue>::new();
        let q = self.clone();
        let vector = q.line.into_vec();
        for key in vector {
            output.push(self.pid_map.get(&key).unwrap().to_json());
        }

        let mut object = json::object!{};
        object["array"] = json::JsonValue::Array(output);
        return object;
    }
    pub fn set_verbose(& mut self, name: String){
        self.verbose = true;
        self.name = name.clone();
    }

}

#[test]
fn use_pcb() {
    let block = pcb::new(0,0,0,0);
    assert_eq!(block.get_pid(),0);
}

#[test]
fn create_q(){
    let q = new();
    assert_eq!(q.line.len(),0);
    assert_eq!(q.pid_map.len(),0);

}

#[test]
fn add_to_q() {
    let mut q = new();
    let pcb1 = pcb::new(1,1,1,1);
    let pcb2 = pcb::new(2,2,2,2);

    q.add_pcb(pcb1.clone());
    assert_eq!(q.line.len(),1);
    assert_eq!(q.pid_map.len(),1);

    q.add_pcb(pcb1);
    assert_eq!(q.line.len(),1);
    assert_eq!(q.pid_map.len(),1);

    q.add_pcb(pcb2);
    assert_eq!(q.line.len(),2);
    assert_eq!(q.pid_map.len(),2);
}

#[test]
fn remove() {
    let mut q = new();
    let pcb1 = pcb::new(1,1,1,1);
    let pcb2 = pcb::new(2,2,2,2);
    q.add_pcb(pcb1);

    q.add_pcb(pcb2);

    q.delete_pid(1);
    
    assert_eq!(q.line.len(),1);
    assert_eq!(q.pid_map.len(),1);

    q.delete_pid(1);

    assert_eq!(q.line.len(),1);
    assert_eq!(q.pid_map.len(),1);

    q.delete_pid(2);

    assert_eq!(q.line.len(),0);
    assert_eq!(q.pid_map.len(),0);
}

