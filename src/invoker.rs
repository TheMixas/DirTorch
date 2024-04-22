use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use lazy_static::lazy::Lazy;
use crate::command::Command;

pub struct Invoker{
    commands: Arc<Mutex<HashMap<String, Box<dyn Command>>>>
}

