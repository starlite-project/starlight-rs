use async_trait::async_trait;
use twilight_model::channel::Message;

#[async_trait]
pub trait Framework {
    async fn dispatch(&self, _: crate::State, _: Message)
}