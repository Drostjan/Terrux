
#![allow(dead_code)]

#[derive(Clone)]
pub struct Color {
    pub black:String,
    pub red:String,
    pub green:String,
    pub orange:String,
    pub blue:String,
    pub purple:String,
    pub cyan:String,
    pub yellow:String,
    pub white:String,
    pub nocolor:String, 
    pub clear_screen:String, 
}

impl Color {
    pub fn new() -> Color {
        Color{
            black:"\x1b[0;30m".to_string(),
            red:"\x1b[0;31m".to_string(),
            green:"\x1b[0;32m".to_string(),
            orange:"\x1b[0;33m".to_string(),
            blue:"\x1b[0;34m".to_string(),
            purple:"\x1b[0;35m".to_string(),
            cyan:"\x1b[0;36m".to_string(),
            yellow:"\x1b[1;33m".to_string(),
            white:"\x1b[1;37m".to_string(),
            nocolor:"\x1b[0m".to_string(),
            clear_screen:"\x1bc".to_string(),
        }
    }

    pub fn disable(&mut self) {
        self.black = "".to_string();
        self.red = "".to_string();
        self.green = "".to_string();
        self.orange = "".to_string();
        self.blue = "".to_string();
        self.purple = "".to_string();
        self.cyan = "".to_string();
        self.yellow = "".to_string();
        self.white = "".to_string();
        self.nocolor = "".to_string();
    }
}