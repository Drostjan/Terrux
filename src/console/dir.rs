
#![allow(dead_code)]

pub mod file;

use file::File;

pub struct Dir {
    pub name: String,
    pub atributes: Vec<String>,
    pub dirs: Vec<Dir>,
    pub files: Vec<File>

}

impl Dir {
    pub fn new(name: &str) -> Dir {
        Dir { 
            name: name.to_owned(), 
            atributes: Vec::new(),
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }

    pub fn dir_add(&mut self, name: &str) {
        self.dirs.push(Dir::new(name));
    }

    pub fn dir_remove(&mut self, name: String) {
        for i in 0..self.dirs.len() {
            if self.dirs[i].name == name{
                self.dirs.remove(i);
            }
            
        }
        
    }

    pub fn in_dir(&mut self, name: &str) -> bool{
        let mut res = false;
        for dir in &self.dirs {
            if dir.name == name  {
                res = true;
            }
            
        }

        res
    }

    pub fn element(&self){
        let mut res = "|_";
        let mut t = String::new();
        for d in &self.dirs{
            t = res.to_string()+"/"+d.get_name().as_str();
            println!("{}",t);
        }
            
    }
    
    pub fn file_add(&mut self, name: String) {
        self.files.push(File::new(name));
    }
}