use std::fs;

pub fn traverse_directory(path: &str) -> u64{
    if let Ok(entries) = fs::read_dir(path) {
        let mut sum  = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_name = entry_path.display();              

                // 如果是链接，则忽略
                if entry_path.is_symlink(){
                    continue;
                }else if entry_path.is_file() {
                    // 如果是普通文件，则计算其大小
                    if let Ok(metadata) = fs::metadata(entry_name.to_string()){
                        let file_size = metadata.len();
                        if file_size > 0 {
                            println!("{} with size {}", entry_name, file_size);
                            sum = sum+file_size;
                        };
                        
                    }                    
                }else if entry_path.is_dir() {
                    // 如果是目录，则递归遍历
                    let res: u64 =  traverse_directory(entry_path.to_str().unwrap());
                    sum += res; 
                    // 
                }
            }
        }
        if sum != 0{
            println!("{} is a directory, with summary size: {}", path, sum);
        }
        return sum
    }
    return 0
}

