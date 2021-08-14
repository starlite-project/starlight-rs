use twilight_model::{
    application::interaction::{ApplicationCommand, MessageComponentInteraction},
    id::UserId,
};

pub trait GetUserId {
    fn user_id(&self) -> UserId;
}

impl GetUserId for ApplicationCommand {
    fn user_id(&self) -> UserId {
        if let Some(member) = &self.member {
            if let Some(user) = &member.user {
                return user.id;
            }
        }

        if let Some(user) = &self.user {
            return user.id;
        }

        UserId::default()
    }
}

impl GetUserId for MessageComponentInteraction {
    fn user_id(&self) -> UserId {
        self.author_id().unwrap_or_default()
    }
}
