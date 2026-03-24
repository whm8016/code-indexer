use crate::config::load_config;
use serde::{Deserialize, Serialize};

/// API 嵌入请求体
#[derive(Serialize)]
struct EmbedRequest {
    model: String,
    input: String,  // 直接字符串，不是对象
}

/// API 响应体 - 兼容百炼新格式
#[derive(Deserialize)]
struct EmbedResponse {
    data: Vec<EmbedData>,
}

/// 嵌入数据项
#[derive(Deserialize)]
struct EmbedData {
    embedding: Vec<f32>,
}

/// 调用嵌入 API 获取文本的向量表示
pub async fn get_embedding(text: &str) -> Result<Vec<f32>, String> {
    // #region debug logging - hypothesis H1
    use std::fs::OpenOptions;
    use std::io::Write;
    let log_path = "c:\\Users\\86181\\Desktop\\demo\\.cursor\\debug.log";
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .and_then(|mut f| {
            writeln!(f, r#"{{"id":"log_{}","timestamp":{},"location":"api.rs:36","message":"get_embedding called","data":{{"text_len":{},"text_repr":"{}","trim_is_empty":{}}}}}"#,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
                text.len(),
                text.replace("\"", "\\\"").chars().take(50).collect::<String>(),
                text.trim().is_empty()
            )
        });
    // #endregion

    if text.trim().is_empty() {
        return Err("文本内容为空".to_string());
    }

    let config = load_config()?;
    let trimmed_text = text.trim();

    let client = reqwest::Client::new();

    let request = EmbedRequest {
        model: config.model.clone(),
        input: trimmed_text.to_string(),  // 直接传字符串
    };

    // #region debug logging - hypothesis H3
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .and_then(|mut f| {
            writeln!(f, r#"{{"id":"log_{}","timestamp":{},"location":"api.rs:54","message":"API request payload","data":{{"model":"{}","texts_len":1,"first_text_len":{}}}}}"#,
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
                config.model,
                trimmed_text.len()
            )
        });
    // #endregion

    let response = client
        .post(&config.base_url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("API 请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        // #region debug logging - hypothesis H3
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .and_then(|mut f| {
                writeln!(f, r#"{{"id":"log_{}","timestamp":{},"location":"api.rs:64","message":"API error response","data":{{"status":"{}","body":"{}"}}}}"#,
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis(),
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
                    status,
                    body.replace("\"", "\\\"")
                )
            });
        // #endregion
        return Err(format!("API 错误 {}: {}", status, body));
    }

    let resp: EmbedResponse = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    resp.data
        .first()
        .map(|e| e.embedding.clone())
        .ok_or_else(|| "响应中没有 embedding".to_string())
}
