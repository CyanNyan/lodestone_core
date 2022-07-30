use std::iter::Iterator;

use super::MaybeUnsupported;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

pub trait TServer {
    fn start(&mut self) -> Result<(), super::Error>;
    fn stop(&mut self) -> Result<(), super::Error>;
    fn state(&self) -> State;
    fn send_command(&self, command: &str) -> MaybeUnsupported<Result<(), super::Error>>;
    fn get_stdout(&self) -> Box<dyn Iterator<Item = String>>;
}