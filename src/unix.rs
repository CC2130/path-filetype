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
    /// 返回一个枚举类型结果 FileType
    fn filetype(&self) -> io::Result<FileType>;

    /// 判断路径是否为一个链接
    fn is_symlink(&self) -> bool;

    /// 判断路径是否为字符设备文件
    fn is_char_device(&self) -> bool;

    /// 判断路径是否为块设备文件
    fn is_block_device(&self) -> bool;

    /// 判断路径是否为管道文件
    fn is_fifo(&self) -> bool;

    /// 判断路径是否为套接字文件
    fn is_socket(&self) -> bool;
}

impl PathFileType for Path {
    fn filetype(&self) -> io::Result<FileType> {
        // 确认文件存在
        if ! self.exists() {
            return Err(io::Error::from(ErrorKind::NotFound))
        }

        if self.is_block_device() {
            Ok(FileType::BlockDevice) 
        } else if self.is_char_device() {
            Ok(FileType::CharDevice)
        } else if self.is_fifo() {
            Ok(FileType::Fifo)
        } else if self.is_socket() {
            Ok(FileType::Socket)
        } else if self.is_symlink() {
            Ok(FileType::Symlink)
        } else if self.is_dir() {
            Ok(FileType::Directory)
        } else {
            Ok(FileType::Regular)
        }
    }

    fn is_symlink(&self) -> bool {
        self.symlink_metadata().unwrap().file_type().is_symlink()
    }

    fn is_char_device(&self) -> bool {
        self.metadata().unwrap().file_type().is_char_device()
    }

    fn is_block_device(&self) -> bool {
        self.metadata().unwrap().file_type().is_block_device()
    }

    fn is_fifo(&self) -> bool {
        self.metadata().unwrap().file_type().is_fifo()
    }

    fn is_socket(&self) -> bool {
        self.metadata().unwrap().file_type().is_socket()
    }
}

impl PathFileType for PathBuf {
    fn filetype(&self) -> io::Result<FileType> {
        self.as_path().filetype()
    }

    fn is_symlink(&self) -> bool {
        self.as_path().is_symlink()
    }

    fn is_char_device(&self) -> bool {
        self.as_path().is_char_device()
    }

    fn is_block_device(&self) -> bool {
        self.as_path().is_block_device()
    }

    fn is_fifo(&self) -> bool {
        self.as_path().is_fifo()
    }

    fn is_socket(&self) -> bool {
        self.as_path().is_socket()
    }
}
