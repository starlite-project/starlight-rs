use crate::{
    repository::{GetEntityFuture, ListEntitiesFuture, Repository},
    utils, Backend, Entity,
};
use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{ChannelType, Group},
    id::{ApplicationId, ChannelId, MessageId, UserId},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupEntity {
    pub application_id: Option<ApplicationId>,
    pub icon: Option<String>,
    pub id: ChannelId,
    pub kind: ChannelType,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    pub name: Option<String>,
    pub owner_id: UserId,
    pub recipient_ids: Vec<UserId>,
}

impl From<Group> for GroupEntity {
    fn from(group: Group) -> Self {
        let recipient_ids = group.recipients.into_iter().map(|user| user.id).collect();

        Self {
            application_id: group.application_id,
            icon: group.icon,
            id: group.id,
            kind: group.kind,
            last_message_id: group.last_message_id,
            last_pin_timestamp: group.last_pin_timestamp,
            name: group.name,
            owner_id: group.owner_id,
            recipient_ids,
        }
    }
}

impl Entity for GroupEntity {
    type Id = ChannelId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait GroupRepository<B: Backend>: Repository<GroupEntity, B> {}
