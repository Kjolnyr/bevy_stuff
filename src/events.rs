use crate::prelude::*;

#[derive(Debug)]
pub struct InputEvent {
    pub cmd: String,
    pub args: Vec<String>
}