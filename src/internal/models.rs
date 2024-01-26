use std::fmt::Display;

use sysinfo::Pid;

#[derive(Debug)]
pub struct App {
    pub pid: Pid,
    pub name: Option<String>,
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "PID: {}\nProcess Name: {}", self.pid, name),
            None => write!(f, "{}", self.pid),
        }
    }
}
