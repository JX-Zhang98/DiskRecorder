


mod scan;

use scan::traverse_directory;

fn main() {
    let root_path = "."; // 要遍历的目录路径
    
    // 递归遍历目录
    let scan_res = traverse_directory(root_path);
    // 打印扫描结果
    println!("Scan Result:{}", scan_res);
}

