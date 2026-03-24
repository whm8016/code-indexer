use serde::{Deserialize, Serialize};

/// 文件系统条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_folder: bool,
}

/// 索引进度事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressPayload {
    pub progress: f64,
    pub current_file: String,
    pub message: String,
}

/// 代码块 - 索引的基本单位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChunk {
    pub chunk_id: String,
    pub file_path: String,
    pub relative_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub extension: String,
    pub symbols: Vec<String>,
}

/// API 返回的嵌入向量结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResult {
    pub chunk_id: String,
    pub embedding: Vec<f32>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub chunk: CodeChunk,
    pub score: f32,
}

/// API 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "YOUR_API_KEY".to_string(),
            model: "text-embedding-v4".to_string(),
            base_url: "https://dashscope.aliyuncs.com/api/v1/services/embeddings/text-embedding"
                .to_string(),
        }
    }
}

/// 索引元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexMetadata {
    pub project_name: String,
    pub project_path: String,
    pub total_files: usize,
    pub total_chunks: usize,
    pub embedding_model: String,
    pub embedding_dim: usize,
    pub created_at: String,
}
