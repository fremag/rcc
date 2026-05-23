pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn tokenize(&self) -> Result<Vec<String>, String> {
        if self.input.len() == 0 {
            return Err("Input is empty".to_string());
        }
        
        Ok(vec![])
    }
}