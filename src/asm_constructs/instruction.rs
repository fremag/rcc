use std::collections::HashMap;

pub trait Instruction: std::fmt::Debug {
    fn to_code(&self) -> String;
    fn fix_pseudo_registers(&mut self, pseudo_registers: &mut StackFrame);
}

pub struct StackFrame {
    items : HashMap<String, usize>
}

impl StackFrame {

    pub fn new() -> Self {
        Self {items: HashMap::new()}
    }

    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&mut self, key: &str) -> usize {
        match self.items.get(key) {
            Some(value) => *value,
            None => {
                let n = self.items.len()+1;
                self.items.insert(key.to_string(), n);
                n
            }
        }
    }
}