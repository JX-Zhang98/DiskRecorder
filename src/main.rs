


mod scan;

use scan::traverse_directory;

fn main() {
    let root_path = "."; // 要遍历的目录路径
    
    // 递归遍历目录
    traverse_directory(root_path);
}

