use std::collections::HashMap;

pub trait Layout {
    type Component: Template + State + Transform + Print;
    fn get_ref_mut(&mut self, component_id: &str) -> Option<&mut Self::Component>;
    fn get_row(&self, row: u16) -> Option<&[Self::Component]>;
    fn get_row_size(&self, row: u16) -> Option<(u16, u16)> {
        if let Some(r) = self.get_row(row) {
            Some(get_row_size(r))
        } else {
            None
        }
    }
}

pub trait Template {
    fn id(&self) -> &str;
    fn template(&self) -> &[&str];
    fn size(&self) -> (u16, u16) {
        get_unit_size(self.template())
    }
}

pub trait State {
    fn state(&self) -> HashMap<String, String>;
    fn set_state(&mut self, new_state: &HashMap<String, String>);
}

pub trait Transform: Template + State {
    fn transform(&self, row: &str, state: &HashMap<String, String>) -> Option<String> {
        transform_row(row, state)
    }
}

pub trait Print: Transform {
    fn print(&self, row: u16) {
        if let Some(str) = self.template().get(row as usize) {
            if let Some(t_str) = self.transform(str, &self.state()) {
                print!("{}", t_str);
            };
        };
    }
}

fn transform_row(row: &str, state: &HashMap<String, String>) -> Option<String> {
    let mut s_row = String::from(row);
    for (key, value) in state {
        let mut placeholder = String::new();
        placeholder.push_str("{{");
        placeholder.push_str(&key);
        placeholder.push_str("}}");
        s_row = s_row.replace(&placeholder, &value);
    }
    Some(s_row)
}

fn get_unit_size(template: &[&str]) -> (u16, u16) {
    let num_rows = template.len();
    let max_num_cols = template
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    (num_rows as u16, max_num_cols as u16)
}

fn get_row_size(units: &[impl Template]) -> (u16, u16) {
    let max_num_rows = units
        .iter()
        .max_by(|a, b| a.size().0.cmp(&b.size().0))
        .unwrap()
        .size()
        .0;

    let max_num_cols = units.iter().fold(0, |acc, unit| acc + unit.size().1);

    (max_num_rows, max_num_cols)
}
