use std::{env, error::Error, fs, path::PathBuf};

use vibe_wire::compatibility_vector_files;

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_output_dir);

    fs::create_dir_all(&output_dir)?;

    for vector_file in compatibility_vector_files() {
        fs::write(
            output_dir.join(vector_file.file_name),
            vector_file.render_json(),
        )?;
    }

    Ok(())
}

fn default_output_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}
