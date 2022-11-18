// yrs
// std only implementation of gnu 'yes'
use std::env;
use std::process::exit;

fn main() {
    let version: String = String::from("0.01a");
    let mut args: Vec<String> = env::args().collect(); 
    let mut msg: String = String::from("y");
    if args.len() > 1 {
        // don't need the first one, 
        // yes I realize I should use .skip()
        args.remove(0); 
        // paruse the items, probably could just use an iter here 
        // but old habbits and have old nuns.
        for i in 0..args.len() { 
            // this should probably be a match statement
            if args[i].eq("-h") || args[i].eq("--help") {
                println!("yrs: rust version of 'yes'");
                println!("returns 'y' or input string\n");
                println!("-h, --help: this help");
                println!("-v, --version: version");
                exit(0);
            }
            if args[i].eq("-v") || args[i].eq("--version") {
                println!("{version}"); // fancy new rust toys.
                exit(0);
            }
        }
        msg = args.join(" ");
    }
    loop { 
    // loop until the heat death of the universe 
    // or of this poor rock, we've tricked into counting...
        println!("{msg}");
    }
} 

// License: MIT

