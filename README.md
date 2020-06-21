OS_Project
This is the first part of my os Project


Each PCB is represented by a struct with the folowing members: 
**usize is an integer with a length determined by the operating system**

pid: usize,
program_counter: usize,
state: usize,
priority: usize,
registers: BTreeSet<usize>,
memory_addresses: BTreeSet<usize>

A process Q has the following members:

line : PriorityQueue<usize,usize>,
pid_map : HashMap<usize, pcb::Pcb>,
verbose : bool,
name: String

the processQ stores a list of PIDs ordered by their priority
It then uses a hash map to map the PID to the specific process block
Verbose shows excecution of what is happening
name is the name of the queue when it is verbose

Main has two functionalities:
When there is no args, main functions as a CLI to simulate creating, moving, popping, and removing PCBS
There are six main commands in main

Add creates a block and adds it to a queue
Peek lets you look at the front of a list
Create allows you to create a Queue
Find locates a specific process and prints it out
Remove removes a specific process
Move puts a process into a different queue

To get better comprehension and see what the code is doing, use Verbose to set all queues and future queues to verbose

When main is supplied a path to a file, it will assume it is json and attempt to read it in as an array of PCBs. Then you can operate on it with the command line format
if you use the flag -v after the path to the file, it will add the files in verbose mode

The format should be as follows

[
    {
        "pid": 0,
        "program_counter": 0,
        "state": 0,
        "priority": 0,
        "memory_addresses": [],
        "registers": [],
        "queue": "ready"
    },
    {
        "pid": 5,
        "program_counter": 5,
        "state": 5,
        "priority": 5,
        "memory_addresses": [],
        "registers": [],
        "queue": "ready"
    },
    {
        "pid": 10,
        "program_counter": 10,
        "state": 10,
        "priority": 10,
        "memory_addresses": [],
        "registers": [],
        "queue": "banana"
    }
]

I wrote this assignment on a System 76 laptop running Pop_os 20.4
I compiled it using the rust package manager cargo
Cargo version was cargo 1.44.1 (88ba85757 2020-06-11)

to download it follow this guide: https://doc.rust-lang.org/cargo/getting-started/installation.html

To build the program run "cargo build"
To make sure it running properly try "cargo test"
To run the program run "cargo run"
To run the program with a input file run "cargo run <path>"

Explaination of algorithims:
Adding a PCB:
    1: check to make sure PID does not exist already in list
    2: push the PID and Priority onto the priority q
    3: insert the PID and PCB onto the pid_hash map

Deleting a PCB by ID:
    1: Remove pid and its priority from Priority Queue
    2: remove pid and its block from pid_map

Deleting the first PCB:
    1: pop the first pid from the priority queue
    2: remove the pid and its PCB from the pid_map

I wrote this in VS code, and ran everything from the vs bash terminal