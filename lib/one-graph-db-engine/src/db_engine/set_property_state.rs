use super::{State, StateContext, match_vertex_state::MatchVertexState, utils::prop_value_from_gremlin_value};
use one_graph_core::{graph::traits::GraphContainerTrait, model::Property};
use one_graph_gremlin::gremlin::*;
use super::gremlin_state::*;

pub struct SetPropertyState {
    name: String,
    value: GValue,
}

impl SetPropertyState {
    pub fn new(name: &str, value: &GValue) -> Self {
        SetPropertyState{name: String::from(name), value: value.clone()}
    }
}
impl State for SetPropertyState {
    fn handle_step(&self, step: &GStep, context: &mut StateContext) -> Result<(), StateError> {
        match &context.previous_step {
            GStep::AddV(label) => {
                let pattern = context.patterns.last_mut().ok_or(StateError::Invalid)?;
                if let Some(nid) = &context.node_index {
                    let n = pattern.get_node_mut(nid);
                    let mut prop = Property::new();
                    prop.set_name(&self.name);
                    prop.set_value(Some(prop_value_from_gremlin_value(&self.value)));
                    n.get_properties_mut().push(prop);
                }
                
            },
            _ => {}
        }
        Ok(())
    }

    fn create_state(&self, step: &GStep, context: &mut StateContext) -> Result<Box<dyn State>, StateError> {
        match step {
            GStep::V(vid) => {
                Ok(Box::new(MatchVertexState::new(vid)))
            }
            GStep::Empty => {
                Ok(Box::new(EndState::new()))
            }
            _ => {
                Err(StateError::Invalid)
            }
        }
    }
}
