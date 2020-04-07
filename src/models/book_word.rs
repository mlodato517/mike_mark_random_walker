#[derive(Debug)]
pub struct BookWord {
    book_id: String,
    word_id: String,
}
impl BookWord {
    pub fn new(book_id: &str, word_id: &str) -> Self {
        BookWord {
            book_id: book_id.to_owned(),
            word_id: word_id.to_owned(),
        }
    }

    pub fn from_collection(ids: Vec<Vec<String>>) -> Vec<Self> {
        ids.iter()
            .map(|id_tuple| Self::new(&id_tuple[0], &id_tuple[1]))
            .collect()
    }
}
