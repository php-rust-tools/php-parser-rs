use crate::lexer::token::OpenTagKind;
use crate::lexer::token::Span;
use crate::lexer::token::TokenKind;
use crate::parser::error::ParseError;
use crate::parser::error::ParseResult;
use crate::parser::state::State;

pub fn skip_semicolon(state: &mut State) -> ParseResult<Span> {
    let end = state.stream.current().span;

    if state.stream.current().kind == TokenKind::SemiColon {
        state.stream.next();
    } else if state.stream.current().kind != TokenKind::CloseTag {
        let found = if state.stream.current().kind == TokenKind::Eof {
            None
        } else {
            Some(state.stream.current().kind.to_string())
        };

        return Err(ParseError::ExpectedToken(
            vec!["`;`".to_string()],
            found,
            end,
        ));
    } else {
        state.stream.next();
    }

    Ok(end)
}

pub fn skip_left_brace(state: &mut State) -> ParseResult<Span> {
    let span = skip(state, TokenKind::LeftBrace)?;
    // A closing PHP tag is valid after a left brace, since
    // that typically indicates the start of a block (control structures).
    if state.stream.current().kind == TokenKind::CloseTag {
        state.stream.next();
    }

    Ok(span)
}

pub fn skip_right_brace(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightBrace)
}

pub fn skip_left_parenthesis(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftParen)
}

pub fn skip_right_parenthesis(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightParen)
}

pub fn skip_left_bracket(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::LeftBracket)
}

pub fn skip_right_bracket(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::RightBracket)
}

pub fn skip_double_arrow(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::DoubleArrow)
}

pub fn skip_double_colon(state: &mut State) -> ParseResult<Span> {
    skip(state, TokenKind::DoubleColon)
}

pub fn skip_colon(state: &mut State) -> ParseResult<Span> {
    let span = skip(state, TokenKind::Colon)?;
    // A closing PHP tag is valid after a colon, since
    // that typically indicates the start of a block (control structures).
    if state.stream.current().kind == TokenKind::CloseTag {
        state.stream.next();
    }
    Ok(span)
}

pub fn skip(state: &mut State, kind: TokenKind) -> ParseResult<Span> {
    if state.stream.current().kind == kind {
        let end = state.stream.current().span;

        state.stream.next();

        Ok(end)
    } else {
        let found = if state.stream.current().kind == TokenKind::Eof {
            None
        } else {
            Some(state.stream.current().kind.to_string())
        };

        Err(ParseError::ExpectedToken(
            vec![format!("`{}`", kind)],
            found,
            state.stream.current().span,
        ))
    }
}

pub fn skip_any_of(state: &mut State, kinds: &[TokenKind]) -> ParseResult<Span> {
    if kinds.contains(&state.stream.current().kind) {
        let end = state.stream.current().span;

        state.stream.next();

        Ok(end)
    } else {
        let found = if state.stream.current().kind == TokenKind::Eof {
            None
        } else {
            Some(state.stream.current().kind.to_string())
        };

        Err(ParseError::ExpectedToken(
            kinds.iter().map(|kind| format!("`{}`", kind)).collect(),
            found,
            state.stream.current().span,
        ))
    }
}

pub fn at_least_one_comma_separated<T>(
    state: &mut State,
    func: &(dyn Fn(&mut State) -> ParseResult<T>),
) -> ParseResult<Vec<T>> {
    let mut result: Vec<T> = vec![];
    loop {
        result.push(func(state)?);

        if state.stream.current().kind != TokenKind::Comma {
            break;
        }

        state.stream.next();
    }

    Ok(result)
}

pub fn skip_close_tag(state: &mut State) -> ParseResult<()> {
    if state.stream.current().kind == TokenKind::CloseTag {
        state.stream.next();
    }

    Ok(())
}

pub fn skip_open_tag(state: &mut State) -> ParseResult<()> {
    if let TokenKind::OpenTag(OpenTagKind::Full) = state.stream.current().kind {
        state.stream.next();
    }

    Ok(())
}
