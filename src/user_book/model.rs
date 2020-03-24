#[derive(Debug)]
pub struct UserBook {
    book_id: String,
    user_id: String
}
impl UserBook {
  pub fn new(book_id: String, user_id: String) -> Self {
    UserBook { book_id, user_id }
  }
}
