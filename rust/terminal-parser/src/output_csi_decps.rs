//! Semantic owner for DECPS (Play Sound) CSI dispatch.
//!
//! Windows Terminal keeps the actual sound dispatch native, but recognizing
//! `CSI , ~` and preserving its parameter contract is portable protocol logic.
//! R09 owns that decision here so the C++ host can become a narrow dispatch
//! seam rather than a second protocol implementation.

use crate::state_machine::{Parameters, VtId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecpsAction {
    PlaySounds(Parameters),
}

/// Plans the DECPS semantic action without crossing the native dispatch seam.
#[must_use]
pub fn plan(id: VtId, parameters: &Parameters) -> Option<DecpsAction> {
    (id.value() == VtId::from_ascii(",~").value())
        .then(|| DecpsAction::PlaySounds(parameters.clone()))
}

#[cfg(test)]
mod tests {
    use super::{DecpsAction, plan};
    use crate::state_machine::{Parameters, VtId};

    #[test]
    fn decps_recognizes_play_sound_and_preserves_parameters() {
        let parameters = Parameters::from_values(vec![Some(1), Some(3), Some(7)]);

        assert_eq!(
            plan(VtId::from_ascii(",~"), &parameters),
            Some(DecpsAction::PlaySounds(parameters))
        );
    }

    #[test]
    fn decps_preserves_empty_parameters_for_native_defaulting() {
        let parameters = Parameters::default();

        assert_eq!(
            plan(VtId::from_ascii(",~"), &parameters),
            Some(DecpsAction::PlaySounds(parameters))
        );
    }

    #[test]
    fn neighboring_advanced_csi_is_not_claimed_by_decps() {
        assert_eq!(
            plan(VtId::from_ascii(",|"), &Parameters::default()),
            None
        );
    }
}
