#[derive(Debug)]
pub struct EventError {
    pub kind: EventErrorType,
}

#[derive(Debug)]
pub enum EventErrorType {}