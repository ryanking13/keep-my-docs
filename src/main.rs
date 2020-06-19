use std::{env, fs};

fn main() -> std::io::Result<()> {
    let target_exts = vec!["pdf", "hwp", "pptx", "ppt", "docx"];
    let target_dir = "keep";
    let suffix = ".keep";

    let current_dir = env::current_dir()?;
    println!("{:?}", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match path.extension() {
                
            }
        }

    }
    Ok(())
}
