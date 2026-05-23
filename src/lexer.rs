pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn identifier_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<identifier>[a-zA-Z_]\w*)\b").unwrap()
    }

    pub fn constant_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<constant>[0-9]+)\b").unwrap()
    }

    pub fn kw_int_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<int>int)\b").unwrap()
    }

    pub fn kw_void_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<void>void)\b").unwrap()
    }

    pub fn kw_return_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<return>return)\b").unwrap()
    }

    pub fn kw_open_par_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<openpar>\()").unwrap()
    }
    pub fn kw_close_par_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<closepar>\))").unwrap()
    }

    pub fn kw_open_brace_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<openbrace>\{)").unwrap()
    }
    pub fn kw_close_brace_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<closebrace>})").unwrap()
    }

    pub fn kw_semi_colon_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<semicolon>;)").unwrap()
    }

    pub fn tokenize(&self) -> Result<Vec<String>, String> {
        if self.input.len() == 0 {
            return Err("Input is empty".to_string());
        }
        let tokens = Vec::new();
        let mut i = 0;

        while i < self.input.len() {
            let sub_str = &self.input[i..];
            if sub_str.starts_with(" ") {
                i += 1;
                continue;
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test]
    fn new_lexer_returns_lexer() {
        let lexer = Lexer::new("foo".to_string());
        assert_eq!(lexer.input, "foo");
    }
    #[test_case("abc", "abc")]
    #[test_case("abc1;123", "abc1")]
    #[test_case("_a_", "_a_")]
    #[test_case("1_a_", "xxx")]
    #[test_case("123", "xxx")]
    fn identifier_regex(value: &str, expected: &str) {
        let re = Lexer::identifier_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("identifier").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }
    #[test_case("abc", "xxx")]
    #[test_case("abc1;123", "xxx")]
    #[test_case("_a_", "xxx")]
    #[test_case("1_a_", "xxx")]
    #[test_case("1;2;3", "1")]
    #[test_case("123", "123")]
    fn constant_regex(value: &str, expected: &str) {
        let re = Lexer::constant_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("constant").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }
    
    #[test_case("int", "int")]
    #[test_case("aint", "xxx")]
    #[test_case("_int_", "xxx")]
    #[test_case("integer", "xxx")]
    #[test_case("int a;", "int")]
    fn int_regex(value: &str, expected: &str) {
        let re = Lexer::kw_int_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("int").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("void", "void")]
    #[test_case("avoid", "xxx")]
    #[test_case("void_empty", "xxx")]
    #[test_case("integer", "xxx")]
    #[test_case("void f() { };", "void")]
    fn void_regex(value: &str, expected: &str) {
        let re = Lexer::kw_void_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("void").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("return", "return")]
    #[test_case("returning", "xxx")]
    #[test_case("return_empty", "xxx")]
    #[test_case("int f() { return 5; };", "xxx")]
    fn return_regex(value: &str, expected: &str) {
        let re = Lexer::kw_return_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("return").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", "(", "(")]
    #[test_case("b", " ( ", "xxx")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn open_par_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_open_par_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("openpar").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", ")", ")")]
    #[test_case("b", " ) ", "xxx")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    #[test_case("d", ")))", ")")]
    fn close_par_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_close_par_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("closepar").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", "{", "{")]
    #[test_case("b", "{{ ", "{")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn open_brace_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_open_brace_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("openbrace").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", "}", "}")]
    #[test_case("b", "}}", "}")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn close_brace_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_close_brace_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("closebrace").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", ";", ";")]
    #[test_case("b", ";;;;", ";")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn close_semicolon_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_semi_colon_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) =>  caps.name("semicolon").unwrap().as_str()
        };
        assert_eq!(x, expected);
    }

}