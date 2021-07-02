/*!
# Path FileType
If you don't know chinese, the next may give you a chance to learn chinses.

在有一个确定的路径时，如何获得此路径是哪种文件类型，这就是这个库完成的工作。  
  
它提供了一个特性`PathFileType`，使你可以直接在使用 Path 类型时，通过 p.filetype()
来获得此路径的文件类型（万物皆文件）。  
```rust
#[cfg(unix)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    CharDevice,
    BlockDevice,
    Fifo,
    Socket,
}
```
  
其只支持 Uinx 类系统，因为目前我只在 Linux 上有使用的需要，后续将继续完善，添加
常用系统，如 MacOS、Windows 的支持。  

## 示例
下面是一些简单的例子，使用很方便。  
  
### 获取文件类型
这是一个简单的示例，例在此项目的根目录里，有一个 data 目录，其里面有一个普通
文件，在测试时还会创建一个链接文件（用完即删）。
  
```rust
extern crate path_filetype;

use std::path::Path;

use path_filetype::*;

let _not_exist = Path::new("not_exist_path");
let _dir = Path::new("data");
let _regular_file = Path::new("data/regular_file");

// 若文件不存在，则返回为 ErrorKind::NotFound
assert_eq!("entity not found", format!("{}", _not_exist.filetype().unwrap_err()));

// 判断是否为普通文件
if let Ok(regular_file) = _regular_file.filetype() {
    assert_eq!(regular_file, FileType::Regular);
}

// 判断是否为目录
assert_eq!(_dir.filetype().unwrap(), FileType::Directory);

// 创建一个链接文件，并检测其文件类型
#[cfg(unix)]
{
    use std::os::unix::fs;
    use std::fs::remove_file;

    let _symlink_file = Path::new("data/symlink_file");
    let _ = fs::symlink("regular_file", _symlink_file);

    assert_eq!(_symlink_file.filetype().unwrap(), FileType::Symlink);

    let _ = remove_file(_symlink_file);
}
```
  
此外，亦可直接对期望类型进行判断，提供 `is_symlink()`、`is_char_device()`、
`is_block_device()`、`is_fifo()`、`is_socket()`方法，返回值为`bool`：  
```rust
extern crate path_filetype;

use std::path::Path;

use path_filetype::*;

#[cfg(unix)]
{
    use std::os::unix::fs;
    use std::fs::remove_file;

    let _symlink_file = Path::new("data/symlink_file");
    let _ = fs::symlink("regular_file", _symlink_file);

    assert_eq!(_symlink_file.is_symlink(), true);

    // Path 中原提供的`is_file()`和`is_dir()`方法不能判断链接
    assert_eq!(_symlink_file.is_file(), true);

    let _ = remove_file(_symlink_file);
}
```
## 注意
下面几项，在使用时需要注意一下：  
 * `Path`中原有的`is_file()`和`is_dir()`方法，不能用来判断路径是否为链接文件。
*/
#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub use unix::*;

// TODO: Windows
