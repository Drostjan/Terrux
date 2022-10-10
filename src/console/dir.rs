
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

    pub fn in_dir(&mut self, name: String) -> String{
        let mut res = String::new();
        for dir in &self.dirs {
            if dir.name == name  {
                res = name.to_owned();
            }else{
                res = "".to_owned();
            }
            
        }

        res
    }

    pub fn file_add(&mut self, name: String) {
        self.files.push(File::new(name));
    }
}