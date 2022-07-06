
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use core_derive::ComponentLike;
use phf::phf_map;

use crate::state_variable_setup::*;

use crate::{ObjectTraitName, TextLikeComponent, NumberLikeComponent,ComponentLike, ComponentSpecificBehavior, ComponentChild};


#[derive(Debug, ComponentLike)]
pub struct Number {
    pub name: String,
    pub parent: RefCell<String>,
    pub children: RefCell<Vec<ComponentChild>>,

    // State variables
    value: StateVar<f64>,
    hide: StateVar<bool>,
}


fn value_return_dependency_instructions(prerequisite_state_values: HashMap<StateVarName, StateVarValue>) -> HashMap<InstructionName, DependencyInstruction> {

    let instruction = DependencyInstruction::Child(ChildDependencyInstruction {
        desired_children: vec![ObjectTraitName::NumberLike],
        desired_state_vars: vec!["value"],
    });

    HashMap::from([("value", instruction)])

}

fn value_determine_state_var_from_dependencies(
    dependency_values: HashMap<InstructionName, Vec<StateVarValue>>
) -> StateVarUpdateInstruction<f64> {


    log!("number dependency values: {:#?}", dependency_values);

    let value = &dependency_values.get("value").unwrap()[0];
    
    if let StateVarValue::String(string_val) = value {
        let num_val = string_val.parse::<f64>().unwrap_or(0.0);
        StateVarUpdateInstruction::SetValue(num_val)
    } else {
        panic!();
    }

}

fn hide_return_dependency_instructions(prerequisite_state_values: HashMap<StateVarName, StateVarValue>) -> HashMap<InstructionName, DependencyInstruction> {
    HashMap::new()
}

fn hide_determine_state_var_from_dependencies(
    dependency_values: HashMap<InstructionName, Vec<StateVarValue>>
) -> StateVarUpdateInstruction<bool> {

    StateVarUpdateInstruction::NoChange
}



impl ComponentSpecificBehavior for Number {

    fn state_variable_instructions(&self) -> &phf::Map<StateVarName, StateVarVariant> {

        
        &phf_map! {
            "value" => StateVarVariant::Number(StateVarDefinition {
                state_vars_to_determine_dependencies: default_state_vars_for_dependencies,
                return_dependency_instructions: value_return_dependency_instructions,
                determine_state_var_from_dependencies: value_determine_state_var_from_dependencies,
            }),

            "hide" => StateVarVariant::Bool(StateVarDefinition { 
                state_vars_to_determine_dependencies: default_state_vars_for_dependencies,
                return_dependency_instructions: hide_return_dependency_instructions,
                determine_state_var_from_dependencies: hide_determine_state_var_from_dependencies,
            }),

        }
        
    }


    fn state_var(&self, name: StateVarName) -> Option<crate::StateVarAccess> {
        use crate::StateVarAccess;

        match name {
            "value" => Option::Some(StateVarAccess::Number(&self.value)),
            "hide" => Option::Some(StateVarAccess::Bool(&self.hide)),
            _ => Option::None,
        }
    }


    fn get_trait_names(&self) -> Vec<ObjectTraitName> {
        vec![ObjectTraitName::NumberLike, ObjectTraitName::TextLike]
    }

    fn get_component_type(&self) -> &'static str {
        "Number"
    }


}



impl TextLikeComponent for Number {
    fn text_value(&self) -> String {
        let val = *self.value.borrow();
        val.to_string()
    }
}
impl NumberLikeComponent for Number {
    fn add_one(&self) -> f64 {
        *self.value.borrow() + 1.0
    }
}



impl Number {
    pub fn create(name: String, parent: String) -> Rc<Number> {
        Rc::new(Number {
            name,
            parent: RefCell::new(parent),
            children: RefCell::new(vec![]),
            
            value: StateVar::new(0.0),
            hide: StateVar::new(false)
        })
    }
}
