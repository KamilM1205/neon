use std::{env::current_dir, fs::read_dir, path::PathBuf};

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum FileType {
    Up,
    Folder,
    File,
}

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub ftype: FileType,
}

#[derive(Clone)]
pub struct FileAdapter {
    pub files: Vec<File>,
    pub curr_dir: PathBuf,
}

impl FileAdapter {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            curr_dir: current_dir().unwrap(),
        }
    }

    pub fn gen_list(&mut self) {
        self.files = Vec::new();
        let file = File {
            name: "..".to_owned(),
            path: PathBuf::from(self.curr_dir.clone()),
            ftype: FileType::Up,
        };
        self.files.push(file);
        let files = match read_dir(self.curr_dir.as_path()) {
            Ok(d) => d,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };

        for f in files {
            let f = f.unwrap();
            let file = File {
                name: f.file_name().into_string().unwrap(),
                path: f.path(),
                ftype: if f.file_type().unwrap().is_dir() {
                    FileType::Folder
                } else {
                    FileType::File
                },
            };
            self.files.push(file);
        }
        self.sort();
    }

    fn sort(&mut self) {
        let mut files: Vec<File> = Vec::new();
        let mut dirs: Vec<File> = Vec::new();

        for file in self.files.iter() {
            match file.ftype {
                FileType::Up => dirs.push(file.clone()),
                FileType::Folder => dirs.push(file.clone()),
                FileType::File => files.push(file.clone()),
            }
        }
        files.sort_by(|a, b| a.name.cmp(&b.name));

        dirs.sort_by(|a, b| a.name.cmp(&b.name));
        dirs.append(&mut files);
        self.files = dirs;
    }
}
