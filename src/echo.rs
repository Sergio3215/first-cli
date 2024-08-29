use std::fmt;

#[derive(Debug)]
pub struct EchoCommands {
    action: String
}

impl EchoCommands {
    pub fn exec(action: String) -> EchoCommands{
        println!("{}",action);
        EchoCommands {action}
    }
}

impl fmt::Display for EchoCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self.action);
        Ok(())
    }
}