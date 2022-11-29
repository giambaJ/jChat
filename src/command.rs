use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("The command provided was invalid. Found {0}")]
    InvalidCommand(String),
    #[error("No command was provided")]
    MissingCommand,
    #[error("No message was provided")]
    MissingMessage,
    #[error("The number provided was invalid")]
    InvalidNumber(#[from] ParseIntError),
    #[error("No number provided")]
    MissingNumber,
}

pub enum Command {
    Write(String, u64),
    Pause(u64),
}

impl TryFrom<String> for Command {
    type Error = CommandError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(command) = value.strip_prefix('/') {
            let split = command.split_whitespace().collect::<Vec<_>>();

            match split.first() {
                Some(&"write") => {
                    let msg = match split.get(1) {
                        Some(v) => Ok(v),
                        None => Err(Self::Error::MissingMessage),
                    }?;

                    let count = match split.get(2) {
                        Some(v) => Ok(v.parse::<u64>()?),
                        _ => Err(Self::Error::MissingNumber),
                    }?;

                    todo!()
                }
                Some(&"pause") => {
                    todo!()
                }
                Some(x) => Err(Self::Error::InvalidCommand(x.to_string())),
                None => Err(Self::Error::MissingCommand),
            }
        } else {
            Ok(Self::Write(value, 1))
        }
    }
}
