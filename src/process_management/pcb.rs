
use std::collections::BTreeSet;
extern crate json;

/*
Struct Definition and methods for Process Control Block

Process Control Block has 6 data members

pid is the Process ID
program_counter is the Program counter
state is the code for the current state
registers is the set of available registers for the program
priority is the code for the priority of the program
memory_address is the set of registers that the program uses

Has a new method which takes the pid, program counter, state, and priority of the program

getter and setters for pid, pc, state, and priority

memory addresses and register have contain, add, and remove methods

to_string returns a string output of the PCB

to_json returns a json object of the PCBs

*/

//PCB struct definition

#[derive(Hash, Eq, PartialEq)]
pub struct Pcb{
    pid: usize,
    program_counter: usize,
    state: usize,
    priority: usize,
    registers: BTreeSet<usize>,
    memory_addresses: BTreeSet<usize>
}


//create a pcb
//Registers and Memory Addresses begin empty
pub fn new(init_pid: usize, init_program_counter:usize, init_state: usize, init_priority: usize) -> Pcb {
    return Pcb {
        pid: init_pid,
        program_counter: init_program_counter,
        state: init_state,
        registers: BTreeSet::<usize>::new(),
        priority: init_priority,
        memory_addresses: BTreeSet::<usize>::new()
    }
}

//Function Definitions for PCB
impl Pcb{

    //Getters
    pub fn get_pid(&self) -> usize {
        return self.pid;
    }

    pub fn get_program_counter(&self)->usize{
        return self.program_counter;
    }

    pub fn get_state(&self)->usize{
        return self.state;
    }

    pub fn get_priority(&self)->usize{
        return self.priority;
    }

    pub fn get_registers_ref(&self)-> &BTreeSet<usize>{
        return &self.registers;
    }

    pub fn get_memory_addresses_ref(&self)->&BTreeSet<usize>{
        return &self.memory_addresses;
    }

    //Setters
    pub fn set_pid(& mut self, pid: usize){
        self.pid = pid;
    }

    pub fn set_program_counter(& mut self, program_counter:usize){
        self.program_counter = program_counter;
    }

    pub fn set_state(& mut self, state:usize){
        self.state = state;
    }

    pub fn set_priority(& mut self, priority:usize){
        self.priority = priority;
    }

    //Adders
    pub fn add_register(& mut self, register:usize){
        self.registers.insert(register);
    }

    pub fn add_memory_address(& mut self, memory_address:usize){
        self.memory_addresses.insert(memory_address);
    }


    //Contains
    pub fn contains_register(&self, register:usize) -> bool{
        return self.registers.contains(&register);
    }

    pub fn contains_memory_address(&self, memory_address:usize)->bool{
        return self.memory_addresses.contains(&memory_address);
    }

    //Removes
    pub fn remove_register(&mut self, register:usize){
        self.registers.remove(&register);
    }

    pub fn remove_memory_address(&mut self, memory_address:usize){
        self.memory_addresses.remove(&memory_address);
    }


    //To String
    pub fn to_string(&self) -> String{
        let mut output: String = String::from("");
        
        //Pusging Process ID onto string
        output.push_str("Process Id: ");
        output.push_str(&self.pid.to_string());

        //Putting PC onto String
        output.push_str(",");
        output.push_str(" Program Counter: ");
        output.push_str(&self.program_counter.to_string());

        //Putting state onto string
        output.push_str(", State: ");
        output.push_str(&self.state.to_string());

        //Putting priority onto string
        output.push_str(", Priority: ");
        output.push_str(&self.priority.to_string());

        //Adding registers
        output.push_str(", Registers: [");

        //Iterating through each register
        let mut register  = self.registers.iter();
        let mut value = register.next();
        if value != None {
            output.push_str(&value.unwrap().to_string());
        }

        value = register.next();
        while value != None{
            output.push_str(", ");
            output.push_str(&value.unwrap().to_string());
            value = register.next();
        }

        output.push_str("]");

        //Adding Memory Addresses
        output.push_str(", Memory Addresses: [");

        //Looping Through Memory Addresses
        let mut memory_address  = self.memory_addresses.iter();
        let mut value = memory_address.next();
        if value != None {
            output.push_str(&value.unwrap().to_string());
        }

        value = memory_address.next();
        while value != None{
            output.push_str(", ");
            output.push_str(&value.unwrap().to_string());
            value = memory_address.next();
        }

        output.push_str("]");



        return output;
    }

    //Converting PCB to json
    pub fn to_json(&self) -> json::JsonValue{
        
        //Converting memory_addresses to vector
        let mut mem_array = Vec::<usize>::new();
        for x in &self.memory_addresses{     
            mem_array.push(*x);
        }

        //Converting registers to vector
        let mut registers = Vec::<usize>::new();
        for x in &self.registers{     
            registers.push(*x);
        }

        //Creating and returning a json object
        return json::object!{
            pid: self.pid,
            program_counter: self.program_counter,
            state: self.state,
            priority: self.priority,
            memory_addresses: mem_array,
            registers: registers
        }
    }
}

impl Clone for Pcb{
    fn clone(&self) -> Pcb{
        return Pcb{
            pid: self.pid,
            program_counter: self.program_counter,
            state: self.state,
            priority: self.priority,
            memory_addresses: self.memory_addresses.clone(),
            registers: self.registers.clone()
        }
    }
}

//Testing initializing
#[test]
fn init_test() {
    let block = new(0, 1, 2, 3);

    assert_eq!(block.pid, 0);
    assert_eq!(block.program_counter,1);
    assert_eq!(block.state,2);
    assert_eq!(block.priority,3);
    assert_eq!(block.memory_addresses.len(),0);
    assert_eq!(block.registers.len(),0);
}

#[test]
fn test_getters() {
    let block = new(0,1,2,3);
    assert_eq!(block.get_pid(), 0);
    assert_eq!(block.get_program_counter(),1);
    assert_eq!(block.get_state(),2);
    assert_eq!(block.get_priority(),3);
    assert_eq!(block.get_memory_addresses_ref().len(),0);
    assert_eq!(block.get_registers_ref().len(),0);
}

#[test]
fn test_setters(){
    let mut block = new(0,1,2,3);
    block.set_pid(12);
    block.set_program_counter(13);
    block.set_state(3);
    block.set_priority(5);

    assert_eq!(block.get_pid(), 12);
    assert_eq!(block.get_program_counter(),13);
    assert_eq!(block.get_state(),3);
    assert_eq!(block.get_priority(),5);
}


#[test]
fn block_sets() {
    let mut block = new(0,0,0,0);
    block.add_memory_address(100);
    assert_eq!(block.get_memory_addresses_ref().len(),1);
    assert_eq!(block.get_memory_addresses_ref().contains(&100), true);

    block.add_memory_address(200);
    assert_eq!(block.get_memory_addresses_ref().len(),2);
    assert_eq!(block.get_memory_addresses_ref().contains(&100), true);
    assert_eq!(block.get_memory_addresses_ref().contains(&200), true);

    block.add_register(1);
    assert_eq!(block.get_registers_ref().len(),1);
    assert_eq!(block.get_registers_ref().contains(&1), true);

    block.add_register(2);
    assert_eq!(block.get_registers_ref().len(),2);
    assert_eq!(block.get_registers_ref().contains(&1), true);
    assert_eq!(block.get_registers_ref().contains(&2), true);

    block.remove_register(2);
    assert_eq!(block.get_registers_ref().len(),1);
    assert_eq!(block.get_registers_ref().contains(&1), true);
    assert_eq!(block.get_registers_ref().contains(&2), false);

    block.remove_memory_address(100);
    assert_eq!(block.get_memory_addresses_ref().contains(&100), false);
    assert_eq!(block.get_memory_addresses_ref().contains(&200), true);
}

#[test]
fn output() {
    let mut block = new(1,2,3,4);
    block.add_memory_address(1);
    block.add_memory_address(2);
    block.add_memory_address(3);

    block.add_register(5);
    block.add_register(6);
    let block_string = block.to_string();
    assert_eq!("Process Id: 1, Program Counter: 2, State: 3, Priority: 4, Registers: [5, 6], Memory Addresses: [1, 2, 3]",block_string);

    let json = block.to_json();

    assert_eq!(json["pid"], 1);
    assert_eq!(json["state"], 3);
    assert_eq!(json["program_counter"], 2);
    assert_eq!(json["priority"], 4);

    assert_eq!(json["memory_addresses"][0], 1);
    assert_eq!(json["memory_addresses"][1], 2);
    assert_eq!(json["memory_addresses"][2], 3);

    assert_eq!(json["registers"][0],5);
    assert_eq!(json["registers"][1],6);
    
}

