#![allow(dead_code)]
#[rustfmt::skip]
pub struct Language {
    pub ping: &'static str,
    pub pong: &'static str,
}
#[rustfmt::skip]
impl Language {
    pub fn ping(&self) -> String {
        self.ping.to_owned()
    }
    pub fn pong(&self, latency: &str) -> String {
        self.pong.to_owned()
            .replace("{latency}", latency)
    }
}
