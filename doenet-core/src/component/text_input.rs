use std::collections::HashMap;

use lazy_static::lazy_static;


use crate::prelude::*;
use super::*;
use crate::state_variables::*;


lazy_static! {

    pub static ref MY_STATE_VAR_DEFINITIONS: HashMap<StateVarName, StateVarVariant> = {

        use StateVarUpdateInstruction::*;

        let mut state_var_definitions = HashMap::new();

        state_var_definitions.insert("value", StateVarVariant::String(StateVarDefinition {
            return_dependency_instructions: USE_ESSENTIAL_DEPENDENCY_INSTRUCTION,
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            ..Default::default()
        }));


        state_var_definitions.insert("expanded", StateVarVariant::Boolean(StateVarDefinition {
            for_renderer: true,
            determine_state_var_from_dependencies: |_| Ok(SetValue(false)),
            ..Default::default()
        }));


        state_var_definitions.insert("size", StateVarVariant::Number(StateVarDefinition {


            determine_state_var_from_dependencies: |_| {
                Ok(SetValue(10.0))
            },
            for_renderer: true,
            ..Default::default()
        }));




        state_var_definitions.insert("width", StateVarVariant::Number(StateVarDefinition {
            for_renderer: true,
            determine_state_var_from_dependencies: |_| Ok(SetValue(600.0)),
            ..Default::default()
        }));


        state_var_definitions.insert("immediateValue", StateVarVariant::String(StateVarDefinition {
            for_renderer: true,
            return_dependency_instructions: USE_ESSENTIAL_DEPENDENCY_INSTRUCTION,
            determine_state_var_from_dependencies: DETERMINE_FROM_ESSENTIAL,
            request_dependencies_to_update_value: REQUEST_ESSENTIAL_TO_UPDATE,
            ..Default::default()
        }));


        state_var_definitions.insert("hidden", HIDDEN_DEFAULT_DEFINITION());
        state_var_definitions.insert("disabled", DISABLED_DEFAULT_DEFINITION());


        return state_var_definitions
    };


}



lazy_static! {
    pub static ref MY_ATTRIBUTE_DEFINITIONS: HashMap<AttributeName, AttributeDefinition> = {
        let mut attribute_definitions = HashMap::new();

        attribute_definitions.insert("hide", AttributeDefinition::Component("boolean"));

        attribute_definitions
    };
}


lazy_static! {
    pub static ref MY_COMPONENT_DEFINITION: ComponentDefinition = ComponentDefinition {
        attribute_definitions: &MY_ATTRIBUTE_DEFINITIONS,

        state_var_definitions: &MY_STATE_VAR_DEFINITIONS,

        // primary_output_traits: || vec![PrimaryOutputTrait::TextLike],

        action_names: || vec!["updateImmediateValue", "updateValue"],

        on_action: |action_name, args, resolve_and_retrieve_state_var| {
            match action_name {
                "updateImmediateValue" => {
                    // Note: the key here is whatever the renderers call the new value
                    let new_val = args.get("text").expect("No text argument");

                    HashMap::from([(
                        StateVarReference::Basic("immediateValue"),
                        new_val.clone()
                    )])
                },

                "updateValue" => {

                    let new_val = resolve_and_retrieve_state_var(&StateVarReference::Basic("immediateValue")).try_into().unwrap();
                    let new_val = StateVarValue::String(new_val);

                    HashMap::from([(
                        StateVarReference::Basic("value"),
                        new_val
                    )])

                }

                _ => panic!("Unknown action '{}' called on textInput", action_name)
            }
        },

        ..Default::default()
    };
}
