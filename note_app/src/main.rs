use std::env;
use std::fs::OpenOptions;
use std::fs;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect(); //   taking argumensts

    let mut _vec : Vec<String> = Vec::new();    //  createing new Vector to store the list note

    if args.len() > 1 {                         //  match the attribute of the user and take action
        match args[1].as_ref() {
            "add" => {
                    make_list(&args);
            },
            "rm" => {
                    remove_list();
            },
            "ls" => {
                    print_list();
            },
            "help" => println!("d"),
            _=> println!("see help, for attributes"),
        }
    } else {
        println!("To utilise, the application use attributes, See help");
    }
}

pub fn make_list(vec : &Vec<String>) {   // Storing the user data to the text file
    let mut str = String::new();
    for s in 2..vec.len() {
        str.push_str(&vec[s]);
        str.push(' ');
    }
    str.pop();
    let mut file = OpenOptions::new()
            .append(true)
            .open("C:/Users/tejpa/dev_work/bin/list.txt")
            .expect("Failed to open the cache file");
    writeln!(file, "{}", str).expect("Failed to write to file"); 
    println!("note taken...!");
}

pub fn remove_list() {
    println!("removed..!");
}

pub fn print_list() {
    let file = fs::read_to_string("C:/Users/tejpa/dev_work/bin/list.txt").expect("failed the read the cache file");
    let lines: Vec<&str> = file.lines().collect();
    for line in lines {
        println!("{}", line);
    }
}

