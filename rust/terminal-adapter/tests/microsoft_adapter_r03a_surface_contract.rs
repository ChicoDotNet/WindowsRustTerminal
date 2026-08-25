use terminal_adapter::adapt_dispatch::{AdaptDispatchCore, PageGeometry};
use terminal_parser::output_engine::{
    DcsAction, DeviceAttributesKind, OutputAction, OutputStateMachineEngine, TermDispatch,
};
use terminal_parser::state_machine::{Parameters, StateMachine, VtId};

fn core() -> AdaptDispatchCore {
    AdaptDispatchCore::new(PageGeometry::new(20, 100, 29))
}

fn assert_deferred(action: OutputAction) {
    let mut dispatch = core();
    dispatch.dispatch(action.clone());
    assert_eq!(dispatch.take_deferred_actions(), vec![action]);
}

fn parse(text: &str) -> AdaptDispatchCore {
    let engine = OutputStateMachineEngine::new(core());
    let mut machine = StateMachine::new(engine);
    machine.process_str(text);
    machine.engine().dispatch().clone()
}

#[test]
fn microsoft_adapter_cursor_hide_show_preserves_dectcem_boundary_action() {
    for enabled in [false, true] {
        assert_deferred(OutputAction::SetMode {
            private: true,
            enabled,
            mode: 25,
        });
    }
}

#[test]
fn microsoft_adapter_graphics_base_preserves_sgr_reset_boundary_action() {
    assert_deferred(OutputAction::SetGraphicsRendition(Parameters::default()));
}

#[test]
fn microsoft_adapter_graphics_single_preserves_single_sgr_parameter_boundary_action() {
    for parameter in [0, 1, 4, 7, 22, 24, 27, 30, 31, 37, 39, 40, 47, 49, 90, 97, 100, 107] {
        assert_deferred(OutputAction::SetGraphicsRendition(Parameters::from_values(vec![
            Some(parameter),
        ])));
    }
}

#[test]
fn microsoft_adapter_graphics_single_with_subparams_preserves_parser_shape() {
    let dispatch = parse("\u{1b}[4:3m");
    let actions = dispatch.deferred_actions();
    assert_eq!(actions.len(), 1);
    let OutputAction::SetGraphicsRendition(parameters) = &actions[0] else {
        panic!("expected SGR action");
    };
    assert_eq!(parameters.at(0), Some(4));
    assert_eq!(parameters.sub_params_for(0), &[Some(3)]);
}

#[test]
fn microsoft_adapter_graphics_push_pop_preserves_stack_boundary_actions_in_order() {
    let mut dispatch = core();
    let push = OutputAction::PushGraphicsRendition(Parameters::from_values(vec![Some(1), Some(10)]));
    dispatch.dispatch(push.clone());
    dispatch.dispatch(OutputAction::PopGraphicsRendition);
    assert_eq!(
        dispatch.take_deferred_actions(),
        vec![push, OutputAction::PopGraphicsRendition]
    );
}

#[test]
fn microsoft_adapter_graphics_persist_brightness_preserves_sgr_ordering_boundary() {
    let mut dispatch = core();
    let actions = [
        OutputAction::SetGraphicsRendition(Parameters::from_values(vec![Some(34)])),
        OutputAction::SetGraphicsRendition(Parameters::from_values(vec![Some(1)])),
        OutputAction::SetGraphicsRendition(Parameters::from_values(vec![Some(32)])),
    ];
    for action in actions.clone() {
        dispatch.dispatch(action);
    }
    assert_eq!(dispatch.take_deferred_actions(), actions);
}

#[test]
fn microsoft_adapter_device_status_operating_status_preserves_dsr_boundary() {
    assert_deferred(OutputAction::DeviceStatusReport {
        private: false,
        status: 5,
        id: None,
    });
}

#[test]
fn microsoft_adapter_device_status_cursor_position_preserves_cpr_boundary() {
    assert_deferred(OutputAction::DeviceStatusReport {
        private: false,
        status: 6,
        id: None,
    });
}

#[test]
fn microsoft_adapter_device_status_extended_cursor_position_preserves_decxcpr_boundary() {
    assert_deferred(OutputAction::DeviceStatusReport {
        private: true,
        status: 6,
        id: None,
    });
}

#[test]
fn microsoft_adapter_device_status_macro_space_preserves_private_62_boundary() {
    assert_deferred(OutputAction::DeviceStatusReport {
        private: true,
        status: 62,
        id: None,
    });
}

#[test]
fn microsoft_adapter_device_status_memory_checksum_preserves_private_63_and_id_boundary() {
    assert_deferred(OutputAction::DeviceStatusReport {
        private: true,
        status: 63,
        id: Some(56),
    });
}

#[test]
fn microsoft_adapter_device_status_private_status_preserves_all_microsoft_status_codes() {
    let mut dispatch = core();
    let statuses = [15, 25, 26, 55, 56, 75, 85];
    let expected = statuses
        .into_iter()
        .map(|status| OutputAction::DeviceStatusReport {
            private: true,
            status,
            id: None,
        })
        .collect::<Vec<_>>();
    for action in expected.clone() {
        dispatch.dispatch(action);
    }
    assert_eq!(dispatch.take_deferred_actions(), expected);
}

#[test]
fn microsoft_adapter_primary_device_attributes_preserves_primary_da_boundary() {
    assert_deferred(OutputAction::DeviceAttributes(DeviceAttributesKind::Primary));
}

#[test]
fn microsoft_adapter_secondary_device_attributes_preserves_secondary_da_boundary() {
    assert_deferred(OutputAction::DeviceAttributes(DeviceAttributesKind::Secondary));
}

#[test]
fn microsoft_adapter_tertiary_device_attributes_preserves_tertiary_da_boundary() {
    assert_deferred(OutputAction::DeviceAttributes(DeviceAttributesKind::Tertiary));
}

#[test]
fn microsoft_adapter_request_displayed_extent_preserves_decrqde_boundary() {
    assert_deferred(OutputAction::RequestDisplayedExtent);
}

#[test]
fn microsoft_adapter_request_terminal_parameters_preserves_permission_parameter() {
    for permission in [0, 1] {
        assert_deferred(OutputAction::RequestTerminalParameters(permission));
    }
}

#[test]
fn microsoft_adapter_request_settings_preserves_decrqss_dcs_boundary() {
    assert_deferred(OutputAction::DcsBegin(DcsAction::RequestSetting));
}

#[test]
fn microsoft_adapter_request_standard_mode_preserves_decrqm_boundary() {
    for mode in [4, 20] {
        assert_deferred(OutputAction::RequestMode {
            private: false,
            mode,
        });
    }
}

#[test]
fn microsoft_adapter_request_private_mode_preserves_dec_private_decrqm_boundary() {
    for mode in [1, 3, 5, 6, 7, 8, 12, 25, 40, 66, 67, 69, 117, 1000, 1002, 1003, 1004, 1005, 1006, 1007, 1049, 2004, 9001] {
        assert_deferred(OutputAction::RequestMode {
            private: true,
            mode,
        });
    }
}

#[test]
fn microsoft_adapter_request_permanent_mode_preserves_2027_boundary() {
    assert_deferred(OutputAction::RequestMode {
        private: true,
        mode: 2027,
    });
}

#[test]
fn microsoft_adapter_request_checksum_report_preserves_decrqcra_advanced_csi_boundary() {
    assert_deferred(OutputAction::AdvancedCsi {
        id: VtId::from_ascii("*y"),
        parameters: Parameters::from_values(vec![
            Some(7),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
        ]),
    });
}

#[test]
fn microsoft_adapter_color_table_report_preserves_terminal_state_report_boundary() {
    for color_model in [1, 2] {
        assert_deferred(OutputAction::AdvancedCsi {
            id: VtId::from_ascii("$u"),
            parameters: Parameters::from_values(vec![Some(2), Some(color_model)]),
        });
    }
}
