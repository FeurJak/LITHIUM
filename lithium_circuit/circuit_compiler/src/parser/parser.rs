use super::ParserError;
use crate::{ast::AST, lang};
use file_map::FileId;
use num_bigint::BigInt;
use reporter::{Location, Span};

pub fn parse(
    src: &str,
    file_id: usize,
    field: &BigInt,
    no_init: bool,
) -> Result<AST, Vec<ParserError>> {
    use lalrpop_util::ParseError::*;
    let mut errors: Vec<ParserError> = Vec::new();
    match preprocess(src, file_id) {
        Ok(src) => {
            match lang::LITHIUMParser::new().parse(file_id, field, no_init, &mut errors, &src) {
                Ok(ast) => Ok(ast),
                Err(parse_error) => match parse_error {
                    InvalidToken { location } => {
                        errors.push(ParserError::illegal_expression(
                            format!("{:?}", parse_error),
                            Location::new(
                                Span::single_char(location as u32),
                                FileId::from(file_id),
                            ),
                        ));
                        Err(errors)
                    }
                    UnrecognizedToken { ref token, .. } => {
                        errors.push(ParserError::illegal_expression(
                            format!("{:?}", parse_error),
                            Location::new(Span::from(token.0..token.2), FileId::from(file_id)),
                        ));
                        Err(errors)
                    }
                    ExtraToken { ref token, .. } => {
                        errors.push(ParserError::illegal_expression(
                            format!("{:?}", parse_error),
                            Location::new(Span::from(token.0..token.2), FileId::from(file_id)),
                        ));
                        Err(errors)
                    }
                    _ => {
                        errors.push(ParserError::illegal_expression(
                            format!("{:?}", parse_error),
                            Location::new(Span::single_char(0), FileId::from(file_id)),
                        ));
                        Err(errors)
                    }
                },
            }
        }
        Err(e) => {
            errors.push(e);
            Err(errors)
        }
    }
}

fn preprocess(pre_src: &str, file_id: usize) -> Result<String, ParserError> {
    let mut src = String::new();
    let mut state: u32 = 0;
    let mut loc: u32 = 0;
    let mut block_start: u32 = 0;

    let mut it = pre_src.chars();
    while let Some(c0) = it.next() {
        loc += 1;
        match (state, c0) {
            (0, '/') => {
                loc += 1;
                match it.next() {
                    Some('/') => {
                        state = 1;
                        src.push(' ');
                        src.push(' ');
                    }
                    Some('*') => {
                        block_start = loc;
                        state = 2;
                        src.push(' ');
                        src.push(' ');
                    }
                    Some(c1) => {
                        src.push(c0);
                        src.push(c1);
                    }
                    None => {
                        src.push(c0);
                        break;
                    }
                }
            }
            (0, _) => src.push(c0),
            (1, '\n') => {
                src.push(c0);
                state = 0;
            }
            (2, '*') => {
                loc += 1;
                let mut next = it.next();
                while next == Some('*') {
                    src.push(' ');
                    loc += 1;
                    next = it.next();
                }
                match next {
                    Some('/') => {
                        src.push(' ');
                        src.push(' ');
                        state = 0;
                    }
                    Some(c) => {
                        src.push(' ');
                        for _i in 0..c.len_utf8() {
                            src.push(' ');
                        }
                    }
                    None => {}
                }
            }
            (_, c) => {
                for _i in 0..c.len_utf8() {
                    src.push(' ');
                }
            }
        }
    }
    if state == 2 {
        return Err::<String, ParserError>(ParserError::unclosed_comment(Location::new(
            Span::single_char(block_start),
            FileId::from(file_id),
        )));
    }
    Ok(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_includes() {
        let src = "
                include a;
                include b;
                include c;
            ";
        match parse(src, 0, &BigInt::from(0), false) {
            Ok(ast) => {
                assert_eq!(ast.items.len(), 3)
            }
            Err(_) => {
                panic!("caught errors")
            }
        }
    }

    #[test]
    fn test_parse_invalid_includes() {
        let src = "
                include a
            ";
        match parse(src, 0, &BigInt::from(0), false) {
            Ok(_) => {
                panic!("should be invalid")
            }
            Err(err) => {
                assert_eq!(err.len(), 1);
                println!("{}", err[0]);
            }
        }
    }
}
