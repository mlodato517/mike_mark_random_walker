mod models;
mod random_walk;
mod utils;

fn main() -> std::io::Result<()> {
    random_walk::generate_random_walk()
}
