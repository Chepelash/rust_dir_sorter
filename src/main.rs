use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use clap::Parser;

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

fn main() {
    // get and parse args
    let args = Args::parse();
    // check if target dir exists
    let dir = Path::new(&args.dir_path);
    assert!(dir.is_dir(), "Dir path does not exists!");
    loop {
        thread::sleep(Duration::from_secs(5));
        // get files in dir
        'main_loop: for entry in dir.read_dir().expect("read dir failed").flatten() {
            // get file
            let file_path = entry.path();
            if !file_path.is_file() {
                continue 'main_loop;
            }

            // get file extension
            let file_extension = file_path
                .as_path()
                .extension()
                .get_or_insert(OsStr::new("unspecified"))
                .to_str()
                .unwrap();

            if args
                .excluded_extentions
                .iter()
                .any(|ex| ex == file_extension)
            {
                continue 'main_loop;
            }

            // get dir to place a file
            let sort_dir = dir.join(Path::new(file_extension));
            if !sort_dir.is_dir() && fs::create_dir::<&Path>(sort_dir.as_ref()).is_err() {
                continue 'main_loop;
            }
            // move file
            let file_name: &OsStr = file_path.file_name().expect("file dont have name");
            fs::rename::<&Path, PathBuf>(file_path.as_ref(), sort_dir.join(file_name))
                .expect("Cannot move file");
        }
    }
}
