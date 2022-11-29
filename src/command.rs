pub enum Command {
    Write(String, u64),
    Pause(u64),
}

impl TryFrom<String> for Command {
    type Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
