use std::fs;
use std::fmt;

pub struct NodeInfo {
    name: String,
    node_type: String,
    size: u64,
    children: Vec<NodeInfo>,
}

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}\n", self.name, self.node_type, self.size);
        if self.children.len() > 0 {
            for child in &self.children {
                print!("\t{}", child);
            }
        }
        return Ok(())
    }
}

pub fn traverse_directory(path: &str) -> NodeInfo{
    let mut info = NodeInfo{
        name: path.to_string(),
        node_type: String::from("dir"),
        size: 0,
        children: vec![],
    };
    if let Ok(entries) = fs::read_dir(path) {
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
                            let child_info = NodeInfo{
                                name: entry_name.to_string(),
                                node_type: String::from("file"),
                                size: file_size,
                                children: vec![],
                            };
                            info.size += file_size;
                            info.children.push(child_info);     
                        };
                                           
                    }                    
                }else if entry_path.is_dir() {
                    // 如果是目录，则递归遍历
                    let res: NodeInfo =  traverse_directory(entry_path.to_str().unwrap());
                    info.size += res.size;         
                    info.children.push(res);
                               
                }
            }
        }
    }
    return info
}

