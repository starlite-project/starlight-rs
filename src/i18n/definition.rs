#![allow(dead_code)]
#[rustfmt::skip]
pub struct Language {
    pub pong: &'static str,
    pub ping: &'static str,
}
#[rustfmt::skip]
impl Language {
    pub fn pong(&self, latency: &str) -> String {
        self.pong.to_owned()
            .replace("{latency}", latency)
    }
    pub fn ping(&self) -> String {
        self.ping.to_owned()
    }
}
