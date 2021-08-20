use super::State;

#[derive(Debug, Clone, Copy)]
pub struct StateHelper<'a> {
    pub state: &'a State,
}

impl<'a> StateHelper<'a> {
    pub const fn new(state: &'a State) -> Self {
        Self { state }
    }
}
