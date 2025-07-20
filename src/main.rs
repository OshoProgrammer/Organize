use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main () -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    println!("Organizing files in: {:?}" , dir );

    for entry in fs::read_dir(&dir)? {
        let entry =entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_upper = format!("{} files", ext.to_uppercase());
                let target_dir = dir.join(&ext_upper);

                fs::create_dir_all(&target_dir)?;

                let file_name = path.file_name().unwrap();
                let target_path = target_dir.join(file_name);

                fs::rename(&path, &target_path)?;
                
                let file_name =path.file_name().unwrap();
                let target_path = target_dir.join(file_name);
                
                   fs::rename(&path, &target_path)?;             
                println!("Moved {:?} -> {:?}", path.file_name().unwrap(), target_path);
            }
        }
        
    }
    println!("Done organizing files.");
    Ok(())
}