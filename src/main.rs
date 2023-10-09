use std::{ffi::OsStr, path::Path, time::Duration};

use clap::Parser;
use tokio::{fs, time::sleep};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // dir to sort
    #[arg(short, long)]
    dir_path: String,

    // excluded extentions
    #[arg(short, long, default_values_t = Vec::<String>::new(), value_delimiter=' ')]
    excluded_extentions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get and parse args
    let args = Args::parse();
    // check if target dir exists
    let dir = Path::new(&args.dir_path);
    assert!(dir.is_dir(), "Dir path does not exists!");
    loop {
        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            if !file_path.is_file() {
                continue;
            }
            let extension = file_path
                .extension()
                .get_or_insert(OsStr::new("unspecified"))
                .to_str()
                .unwrap();
            if args.excluded_extentions.iter().any(|ex| ex == extension) {
                continue;
            }
            
            let sort_dir = dir.join(extension);
            if !sort_dir.is_dir() && fs::create_dir(&sort_dir).await.is_err() {
                continue;
            }
            fs::rename(
                &file_path,
                sort_dir.join(file_path.file_name().expect("file should have a name")),
            )
            .await?;
        }
        sleep(Duration::from_secs(5)).await;
    }
}
