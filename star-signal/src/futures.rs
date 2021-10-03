use futures_util::{future::FutureExt, stream::Stream};
use thiserror::Error;
use std::{future::Future, pin::Pin, task::{Context, Poll}};
use tokio::sync::{mpsc::UnboundedReceiver as MpscReceiver, oneshot::{error::RecvError, Receiver}};
use twilight_model::gateway::{
    event::Event, payload::{MessageCreate, ReactionAdd, InteractionCreate}
};

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Canceled(#[from] RecvError);

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForEventStream {
    pub(crate) rx: MpscReceiver<Event>,
}

impl Stream for WaitForEventStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForGuildEventFuture {
    pub(crate) rx: Receiver<Event>,
}

impl Future for WaitForGuildEventFuture {
    type Output = Result<Event, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

pub struct WaitForGuildEventStream {
    pub(crate) rx: MpscReceiver<Event>,
}

impl Stream for WaitForGuildEventStream {
    type Item = Event;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForMessageFuture {
    pub(crate) rx: Receiver<MessageCreate>,
}

impl Future for WaitForMessageFuture {
    type Output = Result<MessageCreate, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForMessageStream {
    pub(crate) rx: MpscReceiver<MessageCreate>,
}

impl Stream for WaitForMessageStream {
    type Item = MessageCreate;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForReactionFuture {
    pub(crate) rx: Receiver<ReactionAdd>,
}

impl Future for WaitForReactionFuture {
    type Output = Result<ReactionAdd, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForReactionStream {
    pub(crate) rx: MpscReceiver<ReactionAdd>,
}

impl Stream for WaitForReactionStream {
    type Item = ReactionAdd;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct WaitForInteractionFuture {
    pub(crate) rx: Receiver<InteractionCreate>,
}

impl Future for WaitForInteractionFuture {
    type Output = Result<InteractionCreate, Canceled>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rx.poll_unpin(cx).map_err(Canceled)
    }
}

#[derive(Debug)]
#[must_use = "streams do nothing unless you poll them"]
pub struct WaitForInteractionStream {
    pub(crate) rx: MpscReceiver<InteractionCreate>
}

impl Stream for WaitForInteractionStream {
    type Item = InteractionCreate;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}