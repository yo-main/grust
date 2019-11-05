use std::path;

pub struct Match {
    pub row: u32,
    pub count: u32,
    pub word: String,
    pub data: String,
    pub file: path::PathBuf,
}

impl Clone for Match {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            count: self.count,
            word: self.word.clone(),
            data: self.data.clone(),
            file: path::PathBuf::from(&self.file),
        }
    }
}

impl std::fmt::Debug for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n
            Match object:\n\
             file    -> {:?}\n\
             word    -> {:?}\n\
             count   -> {:?}\n\
             row     -> {:?}\n\
            ",
            self.file, self.word, self.count, self.row
        )
    }
}
