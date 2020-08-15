use crate::{expect_compiler_error, parse_input, parse_program};
use leo_ast::ParserError;
use leo_compiler::errors::{CompilerError, ExpressionError, FunctionError, StatementError};
use leo_input::InputParserError;

#[test]
#[ignore]
fn test_semicolon() {
    let bytes = include_bytes!("semicolon.leo");
    let error = parse_program(bytes).err().unwrap();

    match error {
        CompilerError::ParserError(ParserError::SyntaxError(_)) => {}
        _ => panic!("test_semicolon failed the wrong expected error, should be a ParserError"),
    }
}

#[test]
fn test_undefined() {
    let bytes = include_bytes!("undefined.leo");
    let program = parse_program(bytes).unwrap();

    let error = expect_compiler_error(program);

    match error {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(error),
        ))) => {
            assert_eq!(
                format!("{}", error),
                vec![
                    "    --> \"/test/src/main.leo\": 2:12",
                    "     |",
                    "   2 |      return a",
                    "     |             ^",
                    "     |",
                    "     = cannot find value `a` in this scope",
                ]
                .join("\n")
            );
        }
        _ => panic!("expected an undefined identifier error"),
    }
}

#[test]
#[ignore]
fn input_syntax_error() {
    let bytes = include_bytes!("input_semicolon.leo");
    let error = parse_input(bytes).err().unwrap();

    match error {
        CompilerError::InputParserError(InputParserError::SyntaxError(_)) => {}
        _ => panic!("input syntax error should be a ParserError"),
    }
}
