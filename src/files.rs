use std::{fs, os::unix::fs::PermissionsExt};

use crate::{
    constants::{obtener_nombre_lenguaje, Lenguaje},
    word::Word,
};

#[derive(Debug)]
pub struct Archivo {
    pub path: String,
    pub file: Option<fs::File>,
    pub buffer: Vec<Vec<Word>>,
    pub width: usize,
    pub height: usize,
    pub lang: Lenguaje,
}

impl Archivo {
    pub fn new(p: impl Into<String>) -> Archivo {
        let path = p.into();
        let file = fs::File::open(&path).unwrap();
        let content = fs::read_to_string(&path).unwrap();

        /*
                let mut buffer: Vec<Vec<char>> =
                    content.lines().map(|line| line.chars().collect()).collect();
                if buffer.is_empty() {
                    buffer.push(Vec::<char>::new());
                }
        */

        let binding = path.clone();
        let f: Vec<&str> = binding.split('/').collect();
        let s = f.last().unwrap().split('.').last().unwrap();
        let lang = obtener_nombre_lenguaje(s);

        let buffer: Vec<Vec<Word>> = content
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|txt| Word::word_from_lang(txt, &lang))
                    .collect()
            })
            .collect();

        let width = buffer.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = buffer.len();

        Archivo {
            path,
            lang,
            file: Some(file),
            buffer,
            width,
            height,
        }
    }
}

pub fn format_permissions(permissions: fs::Permissions, is_directory: bool) -> String {
    let mode = permissions.mode();

    let file_type_char = if is_directory { 'd' } else { '-' };

    let owner_read = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_execute = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_execute = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_execute = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char,
        owner_read,
        owner_write,
        owner_execute,
        group_read,
        group_write,
        group_execute,
        other_read,
        other_write,
        other_execute
    )
}

impl Default for Archivo {
    fn default() -> Self {
        let buffer: Vec<Vec<Word>> = vec![Vec::new()];
        Self {
            path: "".to_string(),
            file: None,
            lang: Lenguaje::Undefined,
            buffer,
            width: 0,
            height: 0,
        }
    }
}

pub fn is_file(path: impl AsRef<str>) -> bool {
    let path = std::path::Path::new(path.as_ref());
    path.is_file()
}
