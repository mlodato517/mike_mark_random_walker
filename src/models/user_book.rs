#[derive(Debug)]
pub struct UserBook {
    book_id: String,
    user_id: String,
}
impl UserBook {
    pub fn new(user_id: &str, book_id: &str) -> Self {
        UserBook {
            book_id: book_id.to_owned(),
            user_id: user_id.to_owned(),
        }
    }

    pub fn from_collection(ids: Vec<Vec<String>>) -> Vec<Self> {
        ids.iter()
            .map(|id_tuple| Self::new(&id_tuple[0], &id_tuple[1]))
            .collect()
    }
}
