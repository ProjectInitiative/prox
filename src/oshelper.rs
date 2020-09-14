use std::path::{PathBuf};

pub fn combine_paths(path_elements: &[&str]) -> PathBuf {
    // Using a `PathBuf` accumulator, since a `PathBuf` is essentially a mutable
    // `Path`. In reality, a `PathBuf` is just an `OsString` I believe.
    let mut final_path = PathBuf::new();

    // Iterating directly on the input array. There's no need to index into the
    // input array when the index isn't otherwise used. Same thing with Python,
    // which also supports this type of iteration.
    for element in path_elements {
        // Extends the `final_path` variable of type `PathBuf` with individual
        // elements from the input array. Note that this may have unintended
        // behavior where elements resemble a "root" element (e.g. "/dev").
        final_path.push(element);
    }

    final_path
}
