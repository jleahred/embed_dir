use std::path::{Path, PathBuf};


const MAX_FILES: usize = 500;

struct GetFilesStatus {
    depth: usize,
    files: Vec<PathBuf>,
}


fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        show_usage();
        return;
    }

    let orig_folder = Path::new(&args[1]);
    let dest_path_file = Path::new(&args[2]);
    let dest_path = dest_path_file.parent().unwrap();

    let path_source2data = get_rel_path(orig_folder, dest_path);
    let found = get_files(orig_folder, orig_folder.iter().count());

    write_output(&dest_path_file, path_source2data, found.files);
}


impl GetFilesStatus {
    fn new() -> GetFilesStatus {
        GetFilesStatus {
            depth: 0,
            files: vec![],
        }
    }
}


fn get_files(dir: &Path, removefront: usize) -> GetFilesStatus {
    let mut status = GetFilesStatus::new();
    iget_files(dir, removefront, &mut status);
    status
}


fn iget_files(dir: &Path, removefront: usize, status: &mut GetFilesStatus) {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir).map_err(|e| e.to_string()).expect("error reading dir") {
            let entry = entry.map_err(|e| e.to_string()).expect("error getting file");
            let file = entry.path();

            if file.starts_with(".") {
                continue;
            }

            if file.is_dir() {
                status.depth += 1;
                iget_files(&file, removefront, status);
                status.depth -= 1;
            } else {
                status.files.push(file.iter().skip(removefront).collect::<PathBuf>());
                if status.files.len() > MAX_FILES {
                    panic!("Too many files...")
                }
            }
        }
    }
}




fn get_rel_path(from: &Path, to: &Path) -> std::path::PathBuf {
    let get_canon_path = |path: &Path| {
        path.canonicalize()
            .expect(&format!("Error getting full path for {}", path.display()))
    };

    let corig = get_canon_path(from);
    let cdest = get_canon_path(to);

    let ncommom = corig.iter()
        .zip(cdest.iter())
        .take_while(|&(o, d)| o == d)
        .count();

    let backward = Path::new("..")
        .iter()
        .cycle()
        .take(cdest.iter().count() - ncommom)
        .collect::<PathBuf>();

    let orig_branch = corig.iter()
        .skip(ncommom)
        .collect::<PathBuf>();

    backward.join(orig_branch)
}


fn write_output(dest_file: &std::path::Path, relpath: std::path::PathBuf, files: Vec<PathBuf>) {
    use std::io::prelude::*;
    let mut buffer = std::fs::File::create(dest_file).unwrap();

    let strfiles = files.iter()
        .map(|f| {
            format!(r#"          "{}""#,
                    f.to_str()
                        .expect("Error converting file to string"))
        })
        .collect::<Vec<String>>()
        .join(",\n");

    let content = format!(r#"//  autogenerated  embed_dir  on {}


macro_rules! gen_get_includes_bytes{{
    ($rel_path:expr, $($file:expr),+) => {{
        pub fn get(file_name: &str) -> Option<&'static [u8]> {{
            match file_name {{
                $(
                    $file => Some(include_bytes!(concat!($rel_path, $file))),
                )+
                _ => None,
            }}
        }}
    }}
}}


gen_get_includes_bytes!("{}{}",
{}
);
"#,
                          "...",
                          relpath.display(),
                          std::path::MAIN_SEPARATOR,
                          strfiles);

    let _ = write!(buffer, "{}", content);
}



fn show_usage() {
    println!("
    Simple program to generate code to embed files recursivily from a chosen
              folder.

    It will generate a source file with code to embed the files on
              folder.

    More info:  https://github.com/jleahred/embed_dir

    Usage:

              embed_dir  <origin-folder>  <destiny-file>

    where:
        origin-folder
              string is the path to the folder containing the files to embed

              destiny-file    string is rust output filename (you have to write .rs extension)



              example:

        embed_dir src/public src/embed.rs

");

}
