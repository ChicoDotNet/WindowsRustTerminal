//! Lossless borrowed-parameter preparation for state-machine FFI callbacks.
//!
//! The public v1 callbacks intentionally keep their existing flat parameter
//! contract. This module prepares an additive representation that preserves
//! each main parameter, its presence bit, and its ordered subparameters without
//! changing the callback-table layout.

use terminal_parser::state_machine::Parameters;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParameterView {
    pub(crate) values: Vec<i32>,
    pub(crate) present: Vec<u8>,
    pub(crate) sub_values: Vec<i32>,
    pub(crate) sub_present: Vec<u8>,
    pub(crate) sub_offsets: Vec<usize>,
    pub(crate) sub_counts: Vec<usize>,
}

impl ParameterView {
    pub(crate) fn from_parameters(parameters: &Parameters) -> Self {
        let values = parameters.values();
        let mut raw_values = Vec::with_capacity(values.len());
        let mut present = Vec::with_capacity(values.len());
        let mut sub_values = Vec::new();
        let mut sub_present = Vec::new();
        let mut sub_offsets = Vec::with_capacity(values.len());
        let mut sub_counts = Vec::with_capacity(values.len());

        for (index, value) in values.iter().enumerate() {
            raw_values.push(value.unwrap_or_default());
            present.push(u8::from(value.is_some()));
            sub_offsets.push(sub_values.len());

            let sub_parameters = parameters.sub_params_for(index);
            sub_counts.push(sub_parameters.len());
            for sub_parameter in sub_parameters {
                sub_values.push(sub_parameter.unwrap_or_default());
                sub_present.push(u8::from(sub_parameter.is_some()));
            }
        }

        Self {
            values: raw_values,
            present,
            sub_values,
            sub_present,
            sub_offsets,
            sub_counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use terminal_parser::state_machine::{StateMachine, StateMachineEngine, VtId};

    #[derive(Default)]
    struct Witness {
        csi: Option<ParameterView>,
        dcs: Option<ParameterView>,
        ss3: Option<ParameterView>,
    }

    impl StateMachineEngine for Witness {
        fn action_csi_dispatch(&mut self, _id: VtId, parameters: &Parameters) -> bool {
            self.csi = Some(ParameterView::from_parameters(parameters));
            true
        }

        fn action_dcs_dispatch(&mut self, _id: VtId, parameters: &Parameters) -> bool {
            self.dcs = Some(ParameterView::from_parameters(parameters));
            false
        }

        fn action_ss3_dispatch(&mut self, _code_unit: u16, parameters: &Parameters) -> bool {
            self.ss3 = Some(ParameterView::from_parameters(parameters));
            true
        }
    }

    fn expected() -> ParameterView {
        ParameterView {
            values: vec![1, 4],
            present: vec![1, 1],
            sub_values: vec![2, 0, 3, 5],
            sub_present: vec![1, 0, 1, 1],
            sub_offsets: vec![0, 3],
            sub_counts: vec![3, 1],
        }
    }

    #[test]
    fn lossless_view_preserves_csi_subparameters_and_omissions() {
        let mut machine = StateMachine::new(Witness::default());
        machine.process_utf16(&"\u{1b}[1:2::3;4:5m".encode_utf16().collect::<Vec<_>>());
        assert_eq!(machine.engine().csi.as_ref(), Some(&expected()));
    }

    #[test]
    fn lossless_view_preserves_dcs_subparameters_and_omissions() {
        let mut machine = StateMachine::new(Witness::default());
        machine.process_utf16(&"\u{1b}P1:2::3;4:5q".encode_utf16().collect::<Vec<_>>());
        assert_eq!(machine.engine().dcs.as_ref(), Some(&expected()));
    }

    #[test]
    fn lossless_view_preserves_ss3_subparameters_and_omissions() {
        let mut machine = StateMachine::new_input(Witness::default());
        machine.process_utf16(&"\u{1b}O1:2::3;4:5A".encode_utf16().collect::<Vec<_>>());
        assert_eq!(machine.engine().ss3.as_ref(), Some(&expected()));
    }
}