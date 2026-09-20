pub fn format_ast(input: String) -> String {
    let mut result = String::new();
    let mut brace_indent : usize= 0;
    let mut bracket_indent: usize = 0;
    let mut new_line = false;
    let indent = "  ";
    let eol = '\n';

    for c in input.chars() {
        match c {
            '{' => {
                if !result.is_empty() && !result.ends_with(eol) {
                    result.push(' ');
                }

                result.push('{');
                brace_indent += 1;
                result.push(eol);
                new_line = true;
            }

            '}' => {
                brace_indent = brace_indent.saturating_sub(1);

                if !result.ends_with(eol) {
                    result.push(eol);
                }

                result.push_str(&indent.repeat(brace_indent + bracket_indent));

                result.push('}');
                new_line = false;
            }

            '[' => {
                result.push('[');
                bracket_indent += 1;
                result.push(eol);
                new_line = true;
            }

            ']' => {
                bracket_indent = bracket_indent.saturating_sub(1);

                if !result.ends_with(eol) {
                    result.push(eol);
                }

                result.push_str(&indent.repeat(brace_indent + bracket_indent));
                result.push(']');
                new_line = false;
            }

            ',' => {
                result.push(',');
                result.push(eol);
                new_line = true;
            }

            ' ' => {
                if !new_line {
                    result.push(' ');
                }
            }

            _ => {
                if new_line {
                    result.push_str(&indent.repeat(brace_indent + bracket_indent)
                    );

                    new_line = false;
                }

                result.push(c);
            }
        }
    }

    result
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::utils::format_ast;

    #[test]
    fn format_ast_test() {
        let txt = "AstProgram { function: AstFunction { identifier: \"main\", body: [Declaration(AstDeclaration { identifier: \"first_variable\", init: Some(Constant { constant: AstConstant { value: 1 } }) }), Declaration(AstDeclaration { identifier: \"second_variable\", init: Some(Constant { constant: AstConstant { value: 2 } }) }), Statement(Return { expression: Binary { left: Var { identifier: \"first_variable\" }, binop: Add, right: Var { identifier: \"second_variable\" } } })] } }";
        let indent_txt = format_ast(String::from(txt));

        assert_eq!(r#"AstProgram  {
  function: AstFunction  {
    identifier: "main",
    body: [
      Declaration(AstDeclaration  {
        identifier: "first_variable",
        init: Some(Constant  {
          constant: AstConstant  {
            value: 1
          }
        })
      }),
      Declaration(AstDeclaration  {
        identifier: "second_variable",
        init: Some(Constant  {
          constant: AstConstant  {
            value: 2
          }
        })
      }),
      Statement(Return  {
        expression: Binary  {
          left: Var  {
            identifier: "first_variable"
          },
          binop: Add,
          right: Var  {
            identifier: "second_variable"
          }
        }
      })
    ]
  }
}"#, indent_txt);
    }
}