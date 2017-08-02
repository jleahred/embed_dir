use std::env;


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("
    Simple program to generate code to embed files recursivily from a chosen folder.

    It will generate a source file with code to embed the files on folder.

    More info:  https://github.com/jleahred/embed_dir

    Usage:
        embed_dir  <origin-folder>  <destiny-file>

    where:
        origin-folder   string is the path to the folder containing the files to embed
        destiny-file    string is output filename (without .rs extension)


    example:

        embed_dir src/public src/embed.rs

        ");
    }
}


// fn exec_find(dir: &Path, ws_out: &::ws::Sender, status: &mut FindStatus) -> Result<(), String> {
//     use std::ffi::OsStr;
//     if dir.is_dir() {
//         thread::sleep(time::Duration::from_millis(5));
//         for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
//             let entry = entry.map_err(|e| e.to_string())?;
//             let path = entry.path();
//             let file_name_path = OsStr::to_str(path.as_os_str()).unwrap_or("???");
//             // let file_name = path.file_name()
//             //     .and_then(OsStr::to_str)
//             //     .unwrap_or("???");
//             if file_name_path.starts_with(".") && file_name_path.starts_with("./") == false {
//                 continue;
//             }
//             if path.is_dir() {
//                 println!("{:?}", file_name_path);
//                 exec_find(&path, ws_out, status)?;
//             } else {
//                 let ext = path.extension()
//                     .and_then(OsStr::to_str)
//                     .unwrap_or("");
//                 let data = match ext {
//                     "html" => {
//                         Some(proto::MsgOut::Found(Found {
//                             key0: "DOC".to_owned(),
//                             key1: ext.to_owned(),
//                             item: Item {
//                                 text: file_name_path.to_owned(),
//                                 command: proto::MsgIn::Html { file: file_name_path.to_owned() },
//                             },
//                         }))
//                     }
//                     _ => None,
//                 };
//                 send_item(data, ws_out, status)?;
//             }
//         }
//     }
//     Ok(())
// }
