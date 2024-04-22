use std::collections::HashMap;

#[derive(Debug ,Clone)]
pub struct CLIActionParams {
    pub(crate) action_name: String,
    pub(crate) flags: HashMap<String, Vec<String>>,
    pub(crate) parameters: Vec<String>,
}
impl CLIActionParams {
    pub fn new(action_name: String, flags: HashMap<String, Vec<String>>, parameters: Vec<String>) -> CLIActionParams {
        CLIActionParams {
            action_name,
            flags,
            parameters,
        }
    }
}