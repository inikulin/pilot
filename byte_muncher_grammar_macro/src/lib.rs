#![deny(clippy::all)]

#[macro_use]
mod helpers;

mod arm;
mod directives;
mod grammar;
mod state;

pub use self::arm::*;
pub use self::directives::*;
pub use self::grammar::*;
pub use self::state::*;
use proc_macro2::TokenStream as TokenStream2;

pub trait Compile {
    fn compile(&self) -> TokenStream2;
}

// TODO
// v0.1.0
// 2. Rename `as in <state>` to `move --> <state>` (epsilon_move to epsilon move)
// 3. dyn state transition
// 4. Initial state
// 5. Generate SM streaming
// 6. module system
// 7. cool_thing POC

// v0.2.0
// 1. Range pattern
// 2. Pattern negation
// 3. Pattern Or(|)
// 4. Errors functional tests
//    a. Transition in --> arm
//    b. Unreachable arms error / arm precedence
//    c. Duplicate state names
//    d. Inconsistent action args
//    e. Inconsistent action error checks
//    f. Reconsume in sequence (?)
// 5. JSON POC

// v0.3.0
// 1. Skip optimisation

// v0.4.0
// 1. GrarphViz
// 2. TracingGraphViz

// v1.0.0
// 1. Other optimisations

// #[derive(Parser)]
// #[grammar(Html)]
// struct Lexer {
//     ctx: LexerCtx
// }
//
// impl Html::Actions for Lexer {
//
// }
