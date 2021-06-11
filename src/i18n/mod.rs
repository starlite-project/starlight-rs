pub mod definition;
pub mod en_us;

use std::collections::HashMap;
use std::iter::FromIterator;
lazy_static! {
    pub static ref OUTPUT: HashMap<String, definition::Language> = HashMap::from_iter(
        vec![
            ("en_us".to_owned(), self::en_us::OUTPUT),
        ]
        .into_iter()
    );
}
