use std::path::Path;

pub struct Media {
    path: Box<Path>,
    name: String,
    size: usize,
}
