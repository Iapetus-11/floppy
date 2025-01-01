use std::{fs::read_dir, io, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    File,
    Folder,
}

impl FileType {
    #[allow(clippy::inherent_to_string, clippy::wrong_self_convention)]
    pub fn to_string(&self) -> String {
        match self {
            FileType::Folder => "folder",
            FileType::File => "file",
        }
        .to_string()
    }
}

pub fn walk_directory(path: PathBuf) -> Result<Vec<(PathBuf, FileType)>, io::Error> {
    let mut out = vec![];

    let dir_results = read_dir(&path);

    if let Err(error) = dir_results {
        println!("An error occurred while indexing directory {path:?}: {error:?}");
        return Ok(vec![]);
    }

    for file in read_dir(path)? {
        let file = file?;
        let file_type = file.file_type()?;

        if file_type.is_file() {
            out.push((file.path(), FileType::File));
        } else if file_type.is_dir() {
            out.push((file.path(), FileType::Folder));
            out.extend_from_slice(walk_directory(file.path())?.as_slice());
        }
    }

    Ok(out)
}
