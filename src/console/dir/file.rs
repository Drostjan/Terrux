pub struct File {
    name: String,
    atributes: Vec<String>
}

impl File {
    pub fn new(name: String) -> File {
        File { 
            name: name,
            atributes: vec!["S".to_string(),"X".to_string(),"W".to_string(),"R".to_string()],
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }
}