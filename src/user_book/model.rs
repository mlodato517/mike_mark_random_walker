#[derive(Debug)]
pub struct UserBook {
    book_id: String,
    user_id: String,
}
impl UserBook {
    pub fn new(book_id: &str, user_id: &str) -> Self {
        UserBook {
            book_id: book_id.to_owned(),
            user_id: user_id.to_owned(),
        }
    }
}
