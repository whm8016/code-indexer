use std::fs;
use std::path::Path;

use tauri::{AppHandle, Emitter};

use crate::api::get_embedding;
use crate::config::{get_index_dir, get_index_path, load_config, save_config};
use crate::constants::BATCH_SIZE;
use crate::models::{Config, FileEntry, IndexMetadata, ProgressPayload, SearchResult};
use crate::indexer::{chunk_file, collect_code_files, load_index, save_index};

/// 获取系统驱动器列表
#[tauri::command]
pub fn get_drives() -> Result<Vec<FileEntry>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut drives = Vec::new();
        for letter in b'A'..=b'Z' {
            let drive = format!("{}:\\", letter as char);
            let path = Path::new(&drive);
            if path.exists() {
                drives.push(FileEntry {
                    name: format!("{}:", letter as char),
                    path: drive,
                    is_folder: true,
                });
            }
        }
        Ok(drives)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec![FileEntry {
            name: "/".to_string(),
            path: "/".to_string(),
            is_folder: true,
        }])
    }
}

/// 读取目录内容
#[tauri::command]
pub fn read_directory(path: String) -> Result<Vec<FileEntry>, String> {
    use crate::constants::IGNORE_DIRS;

    let dir = Path::new(&path);

    if !dir.exists() {
        return Err("目录不存在".to_string());
    }

    if !dir.is_dir() {
        return Err("路径不是目录".to_string());
    }

    let entries = fs::read_dir(dir).map_err(|e| format!("无法读取目录: {}", e))?;

    let mut result: Vec<FileEntry> = Vec::new();
    let mut all_entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();

    all_entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for entry in all_entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name == "." || name == ".." {
            continue;
        }

        let is_folder = path.is_dir();

        if is_folder && IGNORE_DIRS.contains(&name.as_str()) {
            continue;
        }

        result.push(FileEntry {
            name,
            path: path.to_string_lossy().to_string(),
            is_folder,
        });
    }

    Ok(result)
}

/// 创建时间戳
fn get_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let days = secs / 86400;
    let years = days / 365 + 1970;
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        years, months, day, hours, minutes, seconds
    )
}

/// 索引文件夹
#[tauri::command]
pub async fn index_folder(app: AppHandle, folder_path: String) -> Result<String, String> {
    let root_path = Path::new(&folder_path);

    if !root_path.exists() || !root_path.is_dir() {
        return Err("无效的文件夹路径".to_string());
    }

    let config = load_config()?;

    if config.api_key == "YOUR_API_KEY" {
        return Err("请先在配置文件中设置 API Key".to_string());
    }

    let project_name = root_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Step 1: 扫描文件
    emit_progress(&app, 5.0, "扫描目录...", "正在扫描目录...");

    let files = collect_code_files(root_path)?;
    let total_files = files.len();

    if total_files == 0 {
        return Err("未找到任何代码文件".to_string());
    }

    // Step 2: 切分代码块
    emit_progress(&app, 10.0, "切分代码块...", "正在切分代码块...");

    let mut all_chunks = Vec::new();
    let mut file_count = 0;

    for file_path in &files {
        file_count += 1;

        if file_count % 50 == 0 || file_count == total_files {
            let progress = 10.0 + ((file_count as f64 / total_files as f64) * 20.0);
            emit_progress(
                &app,
                progress,
                file_path,
                &format!("切分代码 ({}/{})", file_count, total_files),
            );
        }

        match chunk_file(file_path, root_path) {
            Ok(chunks) => all_chunks.extend(chunks),
            Err(_) => continue,
        }
    }

    let total_chunks = all_chunks.len();

    if total_chunks == 0 {
        return Err("无法生成代码块".to_string());
    }

    // Step 3: 生成嵌入向量
    emit_progress(
        &app,
        35.0,
        "生成嵌入向量...",
        &format!("准备生成 {} 个嵌入向量...", total_chunks),
    );

    let mut embeddings = Vec::new();

    for i in (0..all_chunks.len()).step_by(BATCH_SIZE) {
        let end = (i + BATCH_SIZE).min(all_chunks.len());
        let batch = &all_chunks[i..end];

        let progress = 35.0 + ((i as f64 / total_chunks as f64) * 55.0);

        if i % 50 == 0 || end == all_chunks.len() {
            emit_progress(
                &app,
                progress,
                &format!("第 {} 批", i / BATCH_SIZE + 1),
                &format!("生成嵌入向量 ({}/{})", end, total_chunks),
            );
        }

        // 批量获取嵌入
        for chunk in batch {
            match get_embedding(&chunk.content).await {
                Ok(embedding) => {
                    embeddings.push(embedding);
                }
                Err(e) => {
                    return Err(format!(
                        "Embedding API 调用失败 (文件: {}):\n\n{}\n\n请检查 API Key 是否正确，或网络连接是否正常。",
                        chunk.relative_path, e
                    ));
                }
            }
        }
    }

    // 验证嵌入维度
    let embedding_dim = embeddings.first().map(|e| e.len()).unwrap_or(1536);

    // Step 4: 保存索引
    emit_progress(&app, 95.0, "保存索引...", "正在保存索引...");

    let metadata = IndexMetadata {
        project_name: project_name.clone(),
        project_path: folder_path.clone(),
        total_files,
        total_chunks,
        embedding_model: config.model.clone(),
        embedding_dim,
        created_at: get_timestamp(),
    };

    let index_path = get_index_path(&project_name);
    save_index(&project_name, &metadata, &all_chunks, &embeddings)?;

    // Step 5: 完成
    emit_progress(
        &app,
        100.0,
        &index_path.to_string_lossy(),
        &format!("索引完成! {} 个文件, {} 个代码块", total_files, total_chunks),
    );

    Ok(format!(
        "索引已保存到: {}\n\n统计:\n- 文件数: {}\n- 代码块: {}\n- 向量维度: {}",
        index_path.display(),
        total_files,
        total_chunks,
        embedding_dim
    ))
}

/// 发送进度事件
fn emit_progress(app: &AppHandle, progress: f64, current_file: &str, message: &str) {
    let _ = app.emit(
        "embed-progress",
        ProgressPayload {
            progress,
            current_file: current_file.to_string(),
            message: message.to_string(),
        },
    );
}

/// 列出所有索引
#[tauri::command]
pub fn list_indices() -> Result<Vec<IndexMetadata>, String> {
    let index_dir = get_index_dir();

    if !index_dir.exists() {
        return Ok(Vec::new());
    }

    let mut indices = Vec::new();

    for entry in fs::read_dir(&index_dir)
        .map_err(|e| format!("读取索引目录失败: {}", e))?
    {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let metadata_path = path.join("metadata.json");
                if metadata_path.exists() {
                    if let Ok(json) = fs::read_to_string(&metadata_path) {
                        if let Ok(meta) = serde_json::from_str::<IndexMetadata>(&json) {
                            indices.push(meta);
                        }
                    }
                }
            }
        }
    }

    indices.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(indices)
}

/// 获取或创建配置
#[tauri::command]
pub fn get_or_create_config() -> Result<Config, String> {
    let config_path = crate::config::get_config_path();

    if config_path.exists() {
        let content =
            fs::read_to_string(&config_path).map_err(|e| format!("读取配置文件失败: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))
    } else {
        let default_config = Config::default();

        fs::create_dir_all(crate::config::get_config_dir())
            .map_err(|e| format!("创建配置目录失败: {}", e))?;

        let json = serde_json::to_string_pretty(&default_config)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        fs::write(&config_path, &json)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;

        Ok(default_config)
    }
}

/// 更新配置
#[tauri::command]
pub fn update_config(config: Config) -> Result<(), String> {
    save_config(&config)
}

/// 计算两个向量的余弦相似度
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

/// 语义搜索代码
#[tauri::command]
pub async fn search_code(
    query: String,
    project_name: String,
    top_k: Option<usize>,
) -> Result<Vec<SearchResult>, String> {
    if query.trim().is_empty() {
        return Err("查询不能为空".to_string());
    }

    let index_path = get_index_dir().join(&project_name);
    if !index_path.exists() {
        return Err(format!("项目 '{}' 的索引不存在", project_name));
    }

    let (_, chunks, embeddings) = load_index(&project_name)?;

    // 获取查询向量
    let query_embedding = get_embedding(&query).await?;

    // 计算相似度并排序
    let mut results: Vec<(usize, f32)> = embeddings
        .iter()
        .enumerate()
        .map(|(i, emb)| (i, cosine_similarity(&query_embedding, emb)))
        .collect();

    results.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let k = top_k.unwrap_or(10).min(results.len());

    Ok(results
        .iter()
        .take(k)
        .map(|(idx, score)| SearchResult {
            chunk: chunks[*idx].clone(),
            score: *score,
        })
        .collect())
}
