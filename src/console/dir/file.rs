pub struct File {
    name: String,
    atributes: Vec<String>
}

impl File {
    pub fn new(name: String) -> File {
        File { 
            name: name,
            atributes: Vec::new()
        }
    }
}