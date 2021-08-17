use twilight_model::{
    application::interaction::{ApplicationCommand, MessageComponentInteraction},
    id::UserId,
};

pub trait InteractionAuthor {
    fn interaction_author(&self) -> UserId;
}

impl InteractionAuthor for ApplicationCommand {
    fn interaction_author(&self) -> UserId {
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

impl InteractionAuthor for MessageComponentInteraction {
    fn interaction_author(&self) -> UserId {
        self.author_id().unwrap_or_default()
    }
}
