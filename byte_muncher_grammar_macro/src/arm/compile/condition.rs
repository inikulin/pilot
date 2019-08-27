use crate::{Arm, ClassPattern, InputStatePattern, Pattern};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Ident, IntSuffix, LitInt};

fn compile_class_pattern(pattern: ClassPattern) -> TokenStream2 {
    use ClassPattern::*;

    match pattern {
        Alnum => quote! { Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'0'..=b'9') },
        Alpha => quote! { Some(b'a'..=b'z') | Some(b'A'..=b'Z') },
        Ascii => quote! { Some(0x00..=0x7f) },
        Lower => quote! { Some(b'a'..=b'z') },
        Upper => quote! { Some(b'A'..=b'Z') },
        Digit => quote! { Some(b'0'..=b'9') },
        Xdigit => quote! { Some(b'0'..=b'9') | Some(b'a'..=b'f') | Some(b'A'..=b'F') },
        Space => {
            quote! { Some(b' ') | Some(b'\n') | Some(b'\r') | Some(b'\t') | Some(b'\x0C') }
        }
    }
}

fn compile_input_state_pattern(pattern: InputStatePattern) -> TokenStream2 {
    use InputStatePattern::*;

    match pattern {
        Eoc => quote! { None if !input.is_last() },
        Eof => quote! { None },
    }
}

fn compile_state_enter_prelude(rhs: TokenStream2) -> TokenStream2 {
    // TODO move internal variables to lazy_static constants
    quote! {
        if self.__state_enter {
            self.__state_enter = false;
            #rhs
        }
    }
}

// TODO: collect condition name
fn compile_condition_pattern(condition: &str) -> TokenStream2 {
    let condition = Ident::new(condition, Span::call_site());

    quote! { Some(b) if self.#condition(b) }
}

impl Arm {
    fn compile_condition(&self, rhs: TokenStream2) -> TokenStream2 {
        use Pattern::*;

        macro_rules! match_arm {
            ($pattern:expr) => {{
                let pattern = $pattern;
                quote! { #pattern => { #rhs } }
            }};
        }

        match self.pattern {
            StateEnter => compile_state_enter_prelude(rhs),
            Byte(b) => match_arm!(LitInt::new(b.into(), IntSuffix::U8, Span::call_site())),
            Class(c) => match_arm!(compile_class_pattern(c)),
            InputState(s) => match_arm!(compile_input_state_pattern(s)),
            Condition(ref c) => match_arm!(compile_condition_pattern(c)),
            Any => match_arm!(quote! { _ }),
            _ => unreachable!(),
        }
    }
}

// Whole length available:
//   match - enter |
//   skip

// Part available:
// if last chunk - skip
// match - block / return |
// skip

#[cfg(test)]
mod tests {
    use super::*;

    curry_parse_macros!($Arm);

    macro_rules! compile {
        ($($t:tt)*) => {
            parse_ok!($($t)*)
                .compile_condition(quote! { __RHS__ })
                .to_string()
        };
    }

    #[test]
    fn compile_any_pattern_arm() {
        assert_eq!(
            compile! {
                _ => __RHS__.
            },
            code_str! {
                _ => { __RHS__ }
            }
        );
    }

    #[test]
    fn compile_byte_pattern_arm() {
        assert_eq!(
            compile! {
                'a' => __RHS__.
            },
            code_str! {
                97u8 => { __RHS__ }
            }
        );
    }

    #[test]
    fn compile_condition_pattern_arm() {
        assert_eq!(
            compile! {
                if foo => __RHS__.
            },
            code_str! {
                Some(b) if self.foo(b) => { __RHS__ }
            }
        );
    }

    #[test]
    fn compile_class_pattern_arm() {
        assert_eq!(
            compile! {
                alnum => __RHS__.
            },
            code_str! {
                Some(b'a'..=b'z') | Some(b'A'..=b'Z') | Some(b'0'..=b'9') => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                alpha => __RHS__.
            },
            code_str! {
                Some(b'a'..=b'z') | Some(b'A'..=b'Z') => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                ascii => __RHS__.
            },
            code_str! {
                Some(0x00..=0x7f) => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                lower => __RHS__.
            },
            code_str! {
                Some(b'a'..=b'z')  => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                upper => __RHS__.
            },
            code_str! {
                Some(b'A'..=b'Z')  => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                digit => __RHS__.
            },
            code_str! {
                Some(b'0'..=b'9')  => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                xdigit => __RHS__.
            },
            code_str! {
                Some(b'0'..=b'9') | Some(b'a'..=b'f') | Some(b'A'..=b'F') => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                space => __RHS__.
            },
            code_str! {
                Some(b' ') | Some(b'\n') | Some(b'\r') | Some(b'\t') | Some(b'\x0C') => { __RHS__ }
            }
        );
    }

    #[test]
    fn compile_state_enter() {
        assert_eq!(
            compile! {
                --> => __RHS__.
            },
            code_str! {
                if self.__state_enter {
                    self.__state_enter = false;
                    __RHS__
                }
            }
        );
    }

    #[test]
    fn compile_input_state_pattern() {
        assert_eq!(
            compile! {
                eof => __RHS__.
            },
            code_str! {
                None => { __RHS__ }
            }
        );

        assert_eq!(
            compile! {
                eoc => __RHS__.
            },
            code_str! {
                None if !input.is_last() => { __RHS__ }
            }
        );
    }
}
