#![allow(unused_must_use)]

mod dir;
mod color;

use dir::Dir;
use color::Color;

use std::io;
use std::io::Write;

pub struct Cmd {
    pub rainbow: Color,
    pub root: Dir,
    pub path: String,
    pub keepalive: bool,
}

impl Cmd {
    pub fn new() -> Cmd {
        Cmd { 
            rainbow: Color::new(),
            root: Dir::new("/"),
            path: String::new(),
            keepalive: true,
            
        }
    }

    pub fn clear_screen() {
        print!("{}[2J", 27 as char);
    }

    pub fn init(&mut self, name: &str, path: &str){
        self.root.name = name.to_owned();
        self.root.dir_add("etc");
        self.root.dir_add("bin");
        self.root.dir_add("usr");
        self.root.dir_add("tmp");
        self.root.dir_add("var");
        self.root.dir_add("opt");
        self.root.dir_add("home");
        self.path = path.to_owned();
    }

    pub fn help(&self){
        println!("");   
        println!("=================== HELP MENU TERRUX ==================== ");
        println!("q|exit        - Exit of the prompt ");
        println!("pwd           - Return the path where you're located      ");
        println!("cd            - Change the directory where you're located ");
        println!("chdir         - Create a new directory  ");
        println!("rmdir         - Delete a directory       ");
        println!("chmod         - Change atributes    ");
        println!("clear         - Clear the screen     ");
        println!("");
    }

    pub fn run(&mut self){ 
        println!(" TerruX {} ",env!("CARGO_PKG_VERSION"));
        while self.keepalive == true {
            print!("terrux@Saturno:{}",self.path.replace("\n", ""));
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let split: Vec<&str> = input.split(' ').collect();
            let comm = split[0].replace("\n", "");
            //print!("{}",comm.len());
            
            match comm.as_str() {
                "exit"|"q" => self.keepalive = false,
                "help" => self.help(),
                "pwd" => println!("{}",self.path),
                _  => println!("Terrux: command not found '{}'",&comm),
            }


        }
    }
}