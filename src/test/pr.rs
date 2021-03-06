//  autogenerated  embed_dir  on ...


macro_rules! gen_get_includes_bytes{
    ($rel_path:expr, $($file:expr),+) => {
        pub fn get(file_name: &str) -> Option<&'static [u8]> {
            match file_name {
                $(
                    $file => Some(include_bytes!(concat!($rel_path, $file))),
                )+
                _ => None,
            }
        }
    }
}


gen_get_includes_bytes!("../../test_folder/",
                        "README.adoc",
                        "Cargo.toml",
                        "pr/README.adoc");
