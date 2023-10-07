use std::{
    ffi::OsStr,
    fs,
    io,
    path::Path,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // dir to sort
    #[arg(short, long)]
    dir_path: String,

    // excluded extentions
    #[arg(short, long, default_values_t = Vec::<String>::new())]
    excluded_extentions: Vec<String>,
}

fn main() {
    // get and parse args
    let args = Args::parse();
    // check if target dir exists
    let dir = Path::new(&args.dir_path);
    assert!(dir.is_dir(), "Dir path does not exists!");
    // get files in dir
    for file_path in dir.read_dir().expect("read dir failed") {
        if let Ok(file_path) = file_path {
            let fp = file_path.path();
            let mut er = fp.as_path().extension().and_then(OsStr::to_str);
            let ups = er.get_or_insert("unspecified");

            let sort_path = dir.join(Path::new(ups));
            if !sort_path.is_dir() {
                // create dir
                // create_dir(sort_path.to_str().unwrap_or("unspecified"));
                if let io::Result::Err(_) = fs::create_dir(sort_path.clone()) {
                    continue;
                }
                // move file
                let file_name = fp.file_name();
                fs::rename(
                    fp.clone(),
                    sort_path.join(file_name.and_then(OsStr::to_str).get_or_insert("err")),
                )
                .unwrap();
            }
        }
    }
}
