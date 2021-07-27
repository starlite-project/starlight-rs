mod emoji;
mod guild;
mod member;
mod message;
mod presence;
mod voice_state;

pub use self::{
    emoji::CachedEmoji, guild::CachedGuild, member::CachedMember, message::CachedMessage,
    presence::CachedPresence, voice_state::CachedVoiceState,
};

#[cfg(test)]
mod tests {
    #[test]
    fn reexports() {
        #[allow(unused_imports)]
        use super::{CachedEmoji, CachedGuild, CachedMember, CachedPresence, CachedVoiceState};
    }
}
