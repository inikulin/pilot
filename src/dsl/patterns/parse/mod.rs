mod byte;
mod sequence;

use syn::parse::{Parse, ParseStream};
use syn::token::Bracket;
use syn::{Error as ParseError, Ident, LitChar, LitInt, LitStr, Result as ParseResult, Token};

use self::byte::BytePattern;
use super::*;

const ERR_UNKNOWN_PATTERN: &str = "unknown pattern";

impl Pattern {
    fn parse_from_ident(input: ParseStream) -> ParseResult<Self> {
        macro_rules! class {
            ($Type:ident) => {
                Ok(Pattern::Class(ClassPattern::$Type))
            };
        }

        macro_rules! input_state_pat {
            ($Type:ident) => {
                Ok(Pattern::InputState(InputStatePattern::$Type))
            };
        }

        let ident = input.parse::<Ident>()?;

        match ident.to_string().as_str() {
            "alnum" => class!(Alnum),
            "alpha" => class!(Alpha),
            "ascii" => class!(Ascii),
            "lower" => class!(Lower),
            "upper" => class!(Upper),
            "digit" => class!(Digit),
            "xdigit" => class!(Xdigit),
            "space" => class!(Space),

            "eoc" => input_state_pat!(Eoc),
            "eof" => input_state_pat!(Eof),

            _ => Err(ParseError::new_spanned(ident, ERR_UNKNOWN_PATTERN)),
        }
    }
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let lookahead = input.lookahead1();

        if parse3_if_present!(input, { - }, { - }, { > }) {
            Ok(Pattern::StateEnter)
        } else if lookahead.peek(LitChar) || lookahead.peek(LitInt) {
            Ok(Pattern::Byte(input.parse::<BytePattern>()?.0))
        } else if lookahead.peek(LitStr) || lookahead.peek(Bracket) {
            Ok(Pattern::Sequence(input.parse::<SequencePattern>()?))
        } else if lookahead.peek(Ident) {
            Ok(Self::parse_from_ident(input)?)
        } else if lookahead.peek(Token! { _ }) {
            input.parse::<Token! { _ }>()?;

            Ok(Pattern::Any)
        } else if lookahead.peek(Token! { if }) {
            input.parse::<Token! { if }>()?;

            Ok(Pattern::Condition(input.parse::<Ident>()?.to_string()))
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($Pattern);

    #[test]
    fn parse_byte_pattern() {
        assert_eq!(parse_ok! { 'a' }, Pattern::Byte(0x61));
        assert_eq!(parse_ok! { 0x61 }, Pattern::Byte(0x61));
    }

    #[test]
    fn parse_seq_pattern() {
        assert_eq!(
            parse_ok! { "FooBar"|i },
            Pattern::Sequence(SequencePattern {
                bytes: vec![0x46, 0x6f, 0x6f, 0x42, 0x61, 0x72],
                ignore_case: true
            })
        );

        assert_eq!(
            parse_ok! { [1, 2, 0x03]|i },
            Pattern::Sequence(SequencePattern {
                bytes: vec![0x01, 0x02, 0x03],
                ignore_case: true
            })
        );
    }

    #[test]
    fn parse_class_pattern() {
        assert_eq!(parse_ok! { alnum }, Pattern::Class(ClassPattern::Alnum));
        assert_eq!(parse_ok! { alpha }, Pattern::Class(ClassPattern::Alpha));
        assert_eq!(parse_ok! { ascii }, Pattern::Class(ClassPattern::Ascii));
        assert_eq!(parse_ok! { lower }, Pattern::Class(ClassPattern::Lower));
        assert_eq!(parse_ok! { upper }, Pattern::Class(ClassPattern::Upper));
        assert_eq!(parse_ok! { digit }, Pattern::Class(ClassPattern::Digit));
        assert_eq!(parse_ok! { xdigit }, Pattern::Class(ClassPattern::Xdigit));
        assert_eq!(parse_ok! { space }, Pattern::Class(ClassPattern::Space));
    }

    #[test]
    fn parse_input_state_pattern() {
        assert_eq!(
            parse_ok! { eoc },
            Pattern::InputState(InputStatePattern::Eoc)
        );
        assert_eq!(
            parse_ok! { eof },
            Pattern::InputState(InputStatePattern::Eof)
        );
    }

    #[test]
    fn parse_condition_pattern() {
        assert_eq!(parse_ok! { if foobar }, Pattern::Condition("foobar".into()));
    }

    #[test]
    fn condition_pattern_without_identifier_error() {
        assert_eq!(parse_err! { if => }, "expected identifier");
    }

    #[test]
    fn parse_any_pattern() {
        assert_eq!(parse_ok! { _ }, Pattern::Any);
    }

    #[test]
    fn parse_state_enter_pattern() {
        assert_eq!(parse_ok! { --> }, Pattern::StateEnter);
    }

    #[test]
    fn unknown_pattern_error() {
        assert_eq!(parse_err! { foobar }, ERR_UNKNOWN_PATTERN);
    }

    #[test]
    fn unexpected_token_error() {
        assert_eq!(
            parse_err! { -3 },
            concat![
                "expected one of: character literal, integer literal, ",
                "string literal, square brackets, identifier, `_`, `if`"
            ]
        );
    }
}