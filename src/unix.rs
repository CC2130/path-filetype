//use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::os::unix::fs::FileTypeExt;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    CharDevice,
    BlockDevice,
    Fifo,
    Socket,
}

// TODO
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Regular {
    ASCII,
    OTHER,
}

pub trait PathFileType {
    fn filetype(&self) -> io::Result<FileType>;
}

impl PathFileType for Path {
    fn filetype(&self) -> io::Result<FileType> {
        // 确认文件存在
        if ! self.exists() {
            return Err(io::Error::from(ErrorKind::NotFound))
        }

        // 使用 symlink_metadata 方法才可判断其是否为链接文件
        let _filetype = self.symlink_metadata().unwrap().file_type();
        println!("{}", _filetype.is_symlink());

        if _filetype.is_block_device() {
            Ok(FileType::BlockDevice) 
        } else if _filetype.is_char_device() {
            Ok(FileType::CharDevice)
        } else if _filetype.is_fifo() {
            Ok(FileType::Fifo)
        } else if _filetype.is_socket() {
            Ok(FileType::Socket)
        } else if _filetype.is_symlink() {
            Ok(FileType::Symlink)
        } else if _filetype.is_dir() {
            Ok(FileType::Directory)
        } else {
            Ok(FileType::Regular)
        }
    }
}

impl PathFileType for PathBuf {
    fn filetype(&self) -> io::Result<FileType> {
        self.as_path().filetype()
    }
}
