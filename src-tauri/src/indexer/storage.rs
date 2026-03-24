use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::config::get_index_path;
use crate::models::{IndexMetadata, CodeChunk};

/// 保存索引数据到磁盘
pub fn save_index(
    project_name: &str,
    metadata: &IndexMetadata,
    chunks: &[CodeChunk],
    embeddings: &[Vec<f32>],
) -> Result<(), String> {
    let index_path = get_index_path(project_name);

    fs::create_dir_all(&index_path).map_err(|e| format!("创建索引目录失败: {}", e))?;

    // 1. 保存元数据 (JSON)
    let metadata_path = index_path.join("metadata.json");
    let metadata_json = serde_json::to_string_pretty(metadata)
        .map_err(|e| format!("序列化元数据失败: {}", e))?;
    fs::write(&metadata_path, metadata_json).map_err(|e| format!("写入元数据失败: {}", e))?;

    // 2. 保存代码块 (JSON)
    let chunks_path = index_path.join("chunks.json");
    let chunks_json =
        serde_json::to_string_pretty(chunks).map_err(|e| format!("序列化 chunks 失败: {}", e))?;
    fs::write(&chunks_path, chunks_json).map_err(|e| format!("写入 chunks 失败: {}", e))?;

    // 3. 保存嵌入向量 (二进制格式，更高效)
    save_embeddings(&index_path.join("embeddings.bin"), embeddings)?;

    Ok(())
}

/// 将嵌入向量保存为二进制格式
fn save_embeddings(path: &Path, embeddings: &[Vec<f32>]) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| format!("创建 embeddings 文件失败: {}", e))?;

    for embedding in embeddings {
        // 写入向量维度 (4 bytes, little-endian)
        let dim = embedding.len() as u32;
        file.write_all(&dim.to_le_bytes())
            .map_err(|e| format!("写入维度失败: {}", e))?;

        // 写入向量数据 (4 bytes per f32, little-endian)
        for v in embedding {
            file.write_all(&v.to_le_bytes())
                .map_err(|e| format!("写入向量失败: {}", e))?;
        }
    }

    Ok(())
}

/// 从磁盘加载索引数据
pub fn load_index(
    project_name: &str,
) -> Result<(IndexMetadata, Vec<CodeChunk>, Vec<Vec<f32>>), String> {
    let index_path = get_index_path(project_name);

    // 1. 加载元数据
    let metadata_path = index_path.join("metadata.json");
    let metadata_json =
        fs::read_to_string(&metadata_path).map_err(|e| format!("读取元数据失败: {}", e))?;
    let metadata: IndexMetadata =
        serde_json::from_str(&metadata_json).map_err(|e| format!("解析元数据失败: {}", e))?;

    // 2. 加载代码块
    let chunks_path = index_path.join("chunks.json");
    let chunks_json =
        fs::read_to_string(&chunks_path).map_err(|e| format!("读取 chunks 失败: {}", e))?;
    let chunks: Vec<CodeChunk> =
        serde_json::from_str(&chunks_json).map_err(|e| format!("解析 chunks 失败: {}", e))?;

    // 3. 加载嵌入向量
    let embeddings_path = index_path.join("embeddings.bin");
    let embeddings = load_embeddings(&embeddings_path)?;

    Ok((metadata, chunks, embeddings))
}

/// 从二进制文件加载嵌入向量
fn load_embeddings(path: &Path) -> Result<Vec<Vec<f32>>, String> {
    let data = fs::read(path).map_err(|e| format!("读取 embeddings 失败: {}", e))?;

    let mut embeddings = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        // 读取维度
        let dim = u32::from_le_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        offset += 4;

        // 读取向量数据
        let mut embedding = Vec::with_capacity(dim);
        for i in 0..dim {
            let val = f32::from_le_bytes([
                data[offset + i * 4],
                data[offset + i * 4 + 1],
                data[offset + i * 4 + 2],
                data[offset + i * 4 + 3],
            ]);
            embedding.push(val);
        }
        offset += dim * 4;
        embeddings.push(embedding);
    }

    Ok(embeddings)
}
