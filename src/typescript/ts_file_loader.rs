use std::io;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use swc_core::common::FileLoader;
use virtual_filesystem::zip_fs::ZipFS;
use virtual_filesystem::FileSystem;

pub struct TsFileLoader<R: Read + Seek> {
    zip_fs: ZipFS<R>,
    root: PathBuf,
}

impl<R: Read + Seek> TsFileLoader<R> {
    pub fn new(zip_fs: ZipFS<R>, root: PathBuf) -> Self {
        Self { zip_fs, root }
    }
}

impl<R: Read + Seek> FileLoader for TsFileLoader<R> {
    fn file_exists(&self, path: &Path) -> bool {
        self.zip_fs
            .exists(self.root.join(path).to_str().unwrap())
            .unwrap()
    }

    fn abs_path(&self, path: &Path) -> Option<PathBuf> {
        Some(path.to_path_buf())
    }

    fn read_file(&self, path: &Path) -> io::Result<String> {
        match self
            .zip_fs
            .open_file(self.root.join(path).to_str().unwrap())
        {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(content)
            }
            _ => Ok(String::new()),
        }
    }
}
