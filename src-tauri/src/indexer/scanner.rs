use std::fs;
use std::path::Path;

use crate::constants::{CODE_EXTENSIONS, IGNORE_DIRS, OTHER_EXTENSIONS};

/// 判断文件是否为需要索引的代码文件
pub fn is_code_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return CODE_EXTENSIONS.contains(&ext_str.as_str())
            || OTHER_EXTENSIONS.contains(&ext_str.as_str());
    }
    false
}

/// 递归收集目录中的所有代码文件
pub fn collect_code_files(dir: &Path) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    collect_files_recursive(dir, &mut files)?;
    Ok(files)
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }

    let entries = fs::read_dir(dir).map_err(|e| format!("无法读取目录: {}", e))?;

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // 跳过隐藏文件和忽略的目录
        if name.starts_with('.') || IGNORE_DIRS.contains(&name.as_str()) {
            continue;
        }

        if path.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if is_code_file(&path) {
            files.push(path.to_string_lossy().to_string());
        }
    }

    Ok(())
}
