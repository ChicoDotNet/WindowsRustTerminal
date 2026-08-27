//! Narrow parser-control operations owned by Adapter semantics.
//!
//! Microsoft's `AdaptDispatch::SetAnsiMode` does not mutate terminal content;
//! it flips the live VT parser between ANSI and VT52 grammar. Keeping that
//! operation here avoids inventing a duplicate parser-mode owner inside the
//! product dispatch aggregate.

use terminal_parser::state_machine::{ParserMode, StateMachine, StateMachineEngine};

/// Applies the Adapter-owned ANSI/VT52 parser mode directly to the live parser.
pub fn set_ansi_mode<E: StateMachineEngine>(machine: &mut StateMachine<E>, enabled: bool) {
    machine.set_parser_mode(ParserMode::Ansi, enabled);
}
