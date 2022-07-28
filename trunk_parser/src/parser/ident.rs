use crate::Parser;
use trunk_lexer::TokenKind;
use super::{ParseResult, ParseError};

impl Parser {
    /// Expect an unqualified identifier such as Foo or Bar.
    pub(crate) fn ident(&mut self) -> ParseResult<String> {
        Ok(expect!(self, TokenKind::Identifier(i), i, "expected identifier"))
    }

    /// Expect an unqualified or qualified identifier such as Foo, Bar or Foo\Bar.
    pub(crate) fn name(&mut self) -> ParseResult<String> {
        Ok(expect!(self, TokenKind::Identifier(i) | TokenKind::QualifiedIdentifier(i), i, "expected identifier"))
    }

    /// Expect an unqualified, qualified or fully qualified identifier such as Foo, Foo\Bar or \Foo\Bar. 
    pub(crate) fn full_name(&mut self) -> ParseResult<String> {
        Ok(expect!(self, TokenKind::Identifier(i) | TokenKind::QualifiedIdentifier(i) | TokenKind::FullyQualifiedIdentifier(i), i, "expected identifier"))
    }

    pub(crate) fn var(&mut self) -> ParseResult<String> {
        Ok(expect!(self, TokenKind::Variable(v), v, "expected variable name"))
    }
}