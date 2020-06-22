use std::{env, fs};
use std::fmt::Write;

fn main() -> std::io::Result<()> {
    let target_exts = vec!["pdf", "hwp", "pptx", "ppt", "docx"];
    let target_dir = "keep";
    let suffix = ".keep";

    let current_dir = env::current_dir()?;
    let target_dir_full = current_dir.join(target_dir);
    // println!("{:?}", target_dir_full);

    if !target_dir_full.exists() {
        println!("{:?} not exists, generating...", &target_dir_full);
        fs::create_dir_all(&target_dir_full)?;
    }

    for entry in fs::read_dir(target_dir_full)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match path.extension() {
                Some(_ext) => {
                    let ext = _ext.to_str().unwrap();
                    if target_exts.contains(&ext) {
                        let filename = path.file_name().unwrap().to_str().unwrap();
                        let mut newname = String::new();

                        write!(newname, "{}{}", filename, suffix).unwrap();
                        println!("Renaming {} to {}...", filename, newname);
                        fs::rename(&path, newname)?;
                    }
                },
                None => continue,
            }
        }

    }
    Ok(())
}