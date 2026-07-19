pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn identifier_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>[a-zA-Z_]\w*)\b").unwrap()
    }

    pub fn constant_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>[0-9]+)\b").unwrap()
    }

    pub fn kw_int_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>int)\b").unwrap()
    }

    pub fn kw_void_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>void)\b").unwrap()
    }

    pub fn kw_return_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>return)\b").unwrap()
    }

    pub fn kw_open_par_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>\()").unwrap()
    }
    pub fn kw_close_par_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>\))").unwrap()
    }

    pub fn kw_open_brace_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>\{)").unwrap()
    }
    pub fn kw_close_brace_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>})").unwrap()
    }

    pub fn kw_semi_colon_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>;)").unwrap()
    }
    pub fn kw_bitwise_complement_op_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>~)").unwrap()
    }
    pub fn kw_negation_op_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>-)").unwrap()
    }
    pub fn kw_two_dec_op_regex() -> regex::Regex {
        regex::Regex::new(r"^(?<item>--)").unwrap()
    }

    pub fn tokenize(&self) -> Result<Vec<String>, String> {
        if self.input.len() == 0 {
            return Err("Input is empty".to_string());
        }
        let regexes = vec![
            Lexer::identifier_regex(),
            Lexer::constant_regex(),
            Lexer::kw_int_regex(),
            Lexer::kw_void_regex(),
            Lexer::kw_return_regex(),
            Lexer::kw_open_par_regex(),
            Lexer::kw_close_par_regex(),
            Lexer::kw_open_brace_regex(),
            Lexer::kw_close_brace_regex(),
            Lexer::kw_semi_colon_regex(),
            Lexer::kw_bitwise_complement_op_regex(),
            Lexer::kw_negation_op_regex(),
            Lexer::kw_two_dec_op_regex(),
        ];
        let mut tokens = Vec::new();
        let mut i = 0;

        while i < self.input.len() {
            let sub_str = &self.input[i..];
            if sub_str.starts_with(" ")
                || sub_str.starts_with("\t")
                || sub_str.starts_with("\r")
                || sub_str.starts_with("\n")
            {
                i += 1;
                continue;
            }

            let mut token = String::new();

            for regex in regexes.iter() {
                if let Some(caps) = regex.captures(sub_str) {
                    let captured_token = caps.name("item").unwrap().as_str().to_string();
                    if captured_token.len() > token.len() {
                        token = captured_token;
                    }
                }
            }
            if token.len() == 0 {
                return Err(format!("Invalid token: {sub_str}"));
            }
            i += token.len();
            tokens.push(token);
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
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
            Some(caps) => caps.name("item").unwrap().as_str(),
        };
        assert_eq!(x, expected);
    }

    #[test]
    fn tokenize_returns_tokens() {
        let lexer = Lexer::new("\t\r         int f() \n        { return 5; };".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec!["int", "f", "(", ")", "{", "return", "5", ";", "}", ";"]
        );
    }

    #[test]
    fn tokenize_test_tokens() {
        let lexer = Lexer::new("int main(void) {    return ~(2);}".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                "int", "main", "(", "void", ")", "{", "return", "~", "(", "2", ")", ";", "}"
            ]
        );
    }

    #[test_case("a", "~", "~")]
    #[test_case("b", "~~", "~")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn bitwise_complement_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_bitwise_complement_op_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) => caps.name("item").unwrap().as_str(),
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", "-", "-")]
    #[test_case("b", "--", "-")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn negation_op_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_negation_op_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) => caps.name("item").unwrap().as_str(),
        };
        assert_eq!(x, expected);
    }

    #[test_case("a", "-", "xxx")]
    #[test_case("b", "--", "--")]
    #[test_case("c", "int f() { return 5; };", "xxx")]
    fn two_dec_op_regex(_name: &str, value: &str, expected: &str) {
        let re = Lexer::kw_two_dec_op_regex();
        let x = match re.captures(value) {
            None => "xxx",
            Some(caps) => caps.name("item").unwrap().as_str(),
        };
        assert_eq!(x, expected);
    }
}
