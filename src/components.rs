use crate::layout::{Layout as ILayout, Print, State, Template, Transform};
use std::collections::HashMap;

const WORDS_TEMPLATE: [&str; 3] = ["", "{{row1}}", "{{row2}}"];
const CURRENT_WORD_TEMPLATE: [&str; 3] = ["", "{{word}}", ""];
const SCORE_TEMPLATE: [&str; 3] = [
    "",
    "time: {{time}}s   correct: {{correct}}  incorrect: {{incorrect}}  accuracy: {{accuracy}}%  speed: {{wpm}}wpm",
    ""
];

#[derive(Debug)]
pub enum Component {
    Words { state: HashMap<String, String> },
    Word { state: HashMap<String, String> },
    Score { state: HashMap<String, String> },
}

impl Component {
    pub fn new(id: &str) -> Self {
        let state = HashMap::new();
        match &id {
            &"words" => Component::Words { state },
            &"word" => Component::Word { state },
            &"score" => Component::Score { state },
            _ => Component::Word { state },
        }
    }
}

impl Transform for Component {}

impl Print for Component {}

impl Template for Component {
    fn id(&self) -> &str {
        match self {
            Component::Words { .. } => "words",
            Component::Word { .. } => "word",
            Component::Score { .. } => "score",
        }
    }
    fn template(&self) -> &[&str] {
        match self {
            Component::Words { .. } => &WORDS_TEMPLATE,
            Component::Word { .. } => &CURRENT_WORD_TEMPLATE,
            Component::Score { .. } => &SCORE_TEMPLATE,
        }
    }
}

impl State for Component {
    fn state(&self) -> HashMap<String, String> {
        match self {
            Component::Words { state, .. } => state.clone(),
            Component::Word { state, .. } => state.clone(),
            Component::Score { state, .. } => state.clone(),
        }
    }
    fn set_state(&mut self, new_state: &HashMap<String, String>) {
        match self {
            Component::Words { state, .. } => {
                *state = new_state.clone();
            }
            Component::Word { state, .. } => {
                *state = new_state.clone();
            }
            Component::Score { state, .. } => {
                *state = new_state.clone();
            }
        }
    }
}

pub struct Layout {
    pub layout: Vec<Vec<Component>>,
}

impl Layout {
    pub fn update(&mut self, component_id: &str, (key, val): (&str, &str)) {
        if let Some(component) = self.get_ref_mut(component_id) {
            let mut state = component.state();
            state.insert(key.to_string(), val.to_string());
            component.set_state(&state);
        }
    }
    pub fn replace(&mut self, component_id: &str, state: &HashMap<String, String>) {
        if let Some(component) = self.get_ref_mut(component_id) {
            component.set_state(state);
        }
    }
}

impl ILayout for Layout {
    type Component = Component;
    fn get_row(&self, row: u16) -> Option<&[Self::Component]> {
        if let Some(components) = self.layout.get(row as usize) {
            Some(components)
        } else {
            None
        }
    }
    fn get_ref_mut(&mut self, component_id: &str) -> Option<&mut Self::Component> {
        if let Some(component) = self
            .layout
            .iter_mut()
            .flatten()
            .find(|c| c.id() == component_id)
        {
            Some(component)
        } else {
            None
        }
    }
}
