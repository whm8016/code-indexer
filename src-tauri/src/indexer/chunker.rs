use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::constants::{CHUNK_MAX_LINES, CHUNK_OVERLAP};
use crate::models::CodeChunk;
use crate::indexer::symbols::extract_symbols;

/// 将文件按行切分成重叠的代码块
pub fn chunk_file(file_path: &str, root_path: &Path) -> Result<Vec<CodeChunk>, String> {
    let path = Path::new(file_path);
    let file = File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    let total_lines = lines.len();

    if total_lines == 0 {
        return Ok(Vec::new());
    }

    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    // 计算相对路径
    let relative_path = if let Ok(stripped) = path.strip_prefix(root_path) {
        stripped.to_string_lossy().to_string()
    } else {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string())
    };

    let mut chunks = Vec::new();
    let mut start = 0;

    while start < total_lines {
        let end = (start + CHUNK_MAX_LINES).min(total_lines);
        let content = lines[start..end].join("\n");

        // 提取当前块的符号
        let symbols = extract_symbols(&content, &ext);

        let chunk_id = format!(
            "{}:{}:{}",
            relative_path.replace(['/', '\\'], "_"),
            start,
            end
        );

        chunks.push(CodeChunk {
            chunk_id,
            file_path: file_path.to_string(),
            relative_path: relative_path.clone(),
            start_line: start + 1, // 1-indexed
            end_line: end,
            content,
            extension: ext.clone(),
            symbols,
        });

        // 滑动窗口：移动到下一个块起始位置（带重叠）
        if end == total_lines {
            break;
        }
        start = end - CHUNK_OVERLAP;
    }

    Ok(chunks)
}
