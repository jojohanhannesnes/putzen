// use std::{
//     fs::{self, File, OpenOptions},
//     path::{Path, PathBuf},
// };

// fn generate_config(config_path: &PathBuf) -> File {
//     let is_config_exists = Path::new(&config_path).exists();
//     if !is_config_exists {
//         File::create(config_path).expect("Error creating .config file")
//     } else {
//         OpenOptions::new()
//             .append(true)
//             .read(true)
//             .open(config_path)
//             .expect("Error opening config file")
//     }
// }
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let entries = if let Some(dir) = dirs::download_dir() {
//         fs::read_dir(dir)?
//     } else {
//         panic!("Download folder not found");
//     };
//     let mut config_location = dirs::home_dir().unwrap();
//     config_location.push("./putzen");
//     let config = generate_config(&config_location);
//     for entry in entries {
//         let path = entry?.path();
//         let meta = fs::metadata(&path)?;
//         let last_modified = meta.clone().modified()?.elapsed()?.as_secs();
//         let formats = format_elapsed(last_modified);
//         println!("path: {path:?} meta: {meta:?} last_modified: {last_modified:?}\n\n\n {formats}");
//     }
//     Ok(())
// }

// fn format_elapsed(secs: u64) -> String {
//     let days = secs / (24 * 3600);
//     let remaining = secs % (24 * 3600);
//     format!("Seconds: {secs} -> Elapsed: {days} days , {remaining} secs remaining")
// }
use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

fn generate_config(config_path: &PathBuf) -> Result<fs::File, std::io::Error> {
    let is_config_exists = Path::new(config_path).exists();
    if !is_config_exists {
        Ok(File::create(config_path)?)
    } else {
        Ok(OpenOptions::new()
            .append(true)
            .read(true)
            .open(config_path)?)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let download_dir = dirs::download_dir()
        .ok_or_else(|| {
            panic!("Download folder not found");
        })
        .unwrap();

    let mut config_location = dirs::home_dir().unwrap();
    config_location.push("./putzen");
    let config = generate_config(&config_location)?;

    for entry in fs::read_dir(download_dir)? {
        let path = entry?.path();
        let meta = fs::metadata(&path)?;
        let last_modified = meta.modified()?.elapsed()?.as_secs();

        // Check if file is older than 30 days
        let formats = format_elapsed(last_modified);
        println!(
            "path: {:?} meta: {:?} last_modified: {:?}\n\n\n {} elapsed: {}",
            path, meta, last_modified, formats.0, formats.1
        );
        if formats.1 > 30 {
            // fs::remove_file(&path)?;
            println!("Deleted file: {:?}", path);
        } else {
            println!(
                "path: {:?} meta: {:?} last_modified: {:?}\n\n\n {}",
                path, meta, last_modified, formats.0
            );
        }
    }

    Ok(())
}

fn format_elapsed(secs: u64) -> (String, u64) {
    let days = secs / (24 * 3600);
    let remaining = secs % (24 * 3600);
    (
        format!("Seconds: {secs} -> Elapsed: {days} days , {remaining} secs remaining"),
        days,
    )
}
