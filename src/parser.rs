use crate::ast_modele::{Constant, Program};

pub struct Parser {
    
}

impl Parser {
    pub fn parse_program(&self, tokens: &Vec<String>) -> Program {
        panic!("Not implemented");
    }

    pub fn parse_constant(&self, tokens: &mut Vec<String>) -> Constant {
        if tokens.len() == 0 {
            panic!("Unexpected end of input");
        } else {
            let cst = tokens.remove(0);
            let value = cst.parse::<i32>().unwrap();
            Constant{value}
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_program_parser() {
        let parser = Parser {};
        let tokens = vec![ "int".to_string(), "main".to_string(), "(".to_string(), "void".to_string(), ")".to_string(), "{".to_string(),
                           "return".to_string(), "2".to_string(), ";".to_string(), "}".to_string()];

        let program = parser.parse_program(&tokens);
    }

    #[test]
    fn test_contant_parser() {
        let parser = Parser {};
        let mut tokens = vec![ "123".to_string()];

        let constant = parser.parse_constant(&mut tokens);
        assert_eq!(constant.value, 123);
    }
    
    #[test]
    #[should_panic]
    fn test_contant_parser_fail_empty() {
        let parser = Parser {};
        let mut tokens = vec![];

        let constant = parser.parse_constant(&mut tokens);
    }
}