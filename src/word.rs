use crate::constants::{reserved_words, Lenguaje};

#[derive(Debug, Clone)]
pub enum KeyType {
    Keyword,
    Type,
    Function,
    Constant,
    Operator,
    Normal,
}

#[derive(Debug, Clone)]
pub struct Word {
    pub txt: String,
    pub keyword: KeyType,
}

impl Word {
    pub fn new(txt: impl Into<String>, keyword: KeyType) -> Self {
        Self {
            txt: txt.into(),
            keyword,
        }
    }

    pub fn word_from_lang(t: impl Into<String>, lang: &Lenguaje) -> Self {
        let reserved = reserved_words(&lang);
        let txt = t.into();
        let keyword: KeyType;
        if reserved.contains(&txt) {
            keyword = KeyType::Keyword;
        } else {
            keyword = KeyType::Normal;
        }

        Self { txt, keyword }
    }
}
