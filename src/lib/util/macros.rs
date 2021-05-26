#[macro_export]
macro_rules! language_use {
    ($language:expr, $key:ident) => {
        $crate::i18n::OUTPUT.get(&$language.to_owned()).unwrap().$key()
    };
    ($language:expr, $key:ident, $($args:expr)*) => {
        $crate::i18n::OUTPUT.get(&$language.to_owned()).unwrap().$key($(&$args)*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_get_ping() {
        assert_eq!(language_use!("en_us", ping), "Ping...")
    }

    #[test]
    fn language_get_pong() {
        assert_eq!(
            language_use!("en_us", pong, "10"),
            "Pong! I took 10 milliseconds!"
        )
    }
}
