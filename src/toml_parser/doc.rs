use std::{fs, path::Path};
use toml_edit::Document;

#[derive(Debug)]
pub struct Doc {
    pub doc: Document,
}

impl std::fmt::Display for Doc {
    /// Formats Doc struct
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = &self.doc;
        s.fmt(f)
    }
}

impl Doc {
    /// Converts file to Doc struct
    pub fn from_file(file: &str) -> Doc {
        let c = fs::read_to_string(file).unwrap();
        let doc = c.parse::<Document>();
        assert!(doc.is_ok());
        Doc { doc: doc.unwrap() }
    }

    pub fn from_str(s: &str) -> Doc {
        let doc = s.parse::<Document>();
        assert!(doc.is_ok());
        Doc { doc: doc.unwrap() }
    }

    pub fn running<F>(&mut self, func: F) -> &mut Self
    where
        F: Fn(&mut toml_edit::Table),
    {
        {
            let root = self.doc.as_table_mut();
            func(root);
        }
        self
    }

    pub fn write_to_file(g: &Self, new_file: &Path) {
        let s = g.doc.to_string();
        let new_contents = s.as_bytes();

        if let Err(e) = fs::write(new_file, new_contents) {
            eprintln!("{}", e)
        }
    }
}
