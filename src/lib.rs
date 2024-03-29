#![deny(clippy::all)]

#[macro_use]
mod helpers;

#[macro_use]
mod compile;

mod arm;
mod directives;
mod grammar;
mod state;

pub use self::arm::*;
pub use self::compile::Compile;
pub use self::directives::*;
pub use self::grammar::*;
pub use self::state::*;

// TODO
// v0.1.0
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

// Kiss:
// 1. Generate StateMachine trait
// 2. Generate Ctx struct
// 3. Return to start - end pins
// 4. Ctx stores pins and inner state
// 5. expose run_parsing_loop method
// 6. Don't buffer
