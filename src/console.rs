mod dir;
mod color;

use dir::Dir;
use color::Color;

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

    pub fn run(&mut self){ 
        println!(" TerruX {} ",env!("CARGO_PKG_VERSION"));
        while self.keepalive {
            println!("terrux@user#{} ",self.path);
            
        }
    }
}