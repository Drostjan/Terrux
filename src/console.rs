#![allow(unused_must_use)]
#![allow(unused_assignments)]

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
    
    pub fn run(&mut self){ 
        println!(" TerruX {} ",env!("CARGO_PKG_VERSION"));
        while self.keepalive == true {
            print!("terrux@Saturno:{} ",self.path.replace("\n", ""));
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let split: Vec<&str> = input.split(' ').collect();
            let comm = split[0].replace("\n", "");
            let mut param = String::new();
            //print!("{}",comm.len());
            
            match comm.as_str() {
                "exit"|"q" => self.keepalive = false,
                "help" => self.help(),
                "pwd" => println!("{}",self.path),
                "cd" => {
                    if split.len() == 1 {
                        self.path = self.path.clone();
                    }else if split.len() == 2 {
                        param = split[1].replace("\n", "");
                        self.cd(&param);
                    }else{
                        println!("Terrux: bad lenght command ")
                    }
                },
                "chdir" => {
                    if self.path == "/"{
                        param = split[1].replace("/", "").replace("\n", "");
                        self.root.dir_add(&param);
                    }else{
                        println!("Terrux: '{}' unknown path", param);
                    }
                },
                "tree" => self.tree(),
                _  => println!("Terrux: command not found '{}'",&comm),
            }

        }
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
    
    pub fn clear_screen(&self) {
        print!("{}[2J", 27 as char);
    }
    
    pub fn cd(&mut self, dest: &str) {
        let mut p = String::new();
        let mut t = String::new();
        if dest == "." { 
            p = self.path.clone();
        }else if dest == ".." || dest == "-" {
            if self.path == "/"{
                p = self.path.clone();
            }else{
                let plus : Vec<&str>= self.path.split("/").collect();
                if plus.len() == 2{
                    p = "/".to_string();
                }
                else{
                    t = "/".to_string()+plus[1];
                    p = t;
                }
            }
        }else if dest.contains("/"){
            let plus : Vec<&str>= dest.split("/").collect();
            if self.root.in_dir(plus[1]) == true{
                p = dest.to_string();
            }
            else{
                println!("Terrux: '{}' unknown path",&self.path);
            }
        }else{
            if self.root.in_dir(dest) == true{
                t = "/".to_string()+dest;
                p = t;
            }else{
                println!("Terrux: '{}' unknown path",&self.path);
            }
        }
    
        self.path = p;
    }

    pub fn tree(&self){
        let name = self.root.get_name();
        println!("{}",name);
        self.root.element();
    }

}

