use std::{ffi::OsStr, fs, path::Path, thread, time::Duration};

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
    // get files in dir
    loop {
        thread::sleep(Duration::from_secs(5));

        'main_loop: for file_path in dir.read_dir().expect("read dir failed").flatten() {
            let fp = file_path.path();
            if !fp.is_file() {
                continue 'main_loop;
            }
            let mut er = fp.as_path().extension().and_then(OsStr::to_str);
            let ups = er.get_or_insert("unspecified");

            if args.excluded_extentions.contains(&ups.to_string()) {
                continue 'main_loop;
            }

            let sort_path = dir.join(Path::new(ups));
            if !sort_path.is_dir() && fs::create_dir(sort_path.clone()).is_err() {
                continue 'main_loop;
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
