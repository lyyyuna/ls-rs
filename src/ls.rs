pub struct Ls<'a> {
    pub args: Vec<&'a str>,
}

impl<'a> Ls<'a> {
    pub fn new() -> Self {
        Ls{
            args: vec!["."],
        }
    }

    pub fn run(&mut self) {
        let mut files = Vec::new();

        for file_path in &self.args {
            files.push(file_path)
        }
    }

    fn print_dirs(&mut self) {
        
    }
}