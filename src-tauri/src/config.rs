use std::fs;
use std::path::PathBuf;

use crate::models::Config;

/// 获取配置目录路径
pub fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("code-indexer")
}

/// 获取配置文件的完整路径
pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.json")
}

/// 获取索引存储目录
pub fn get_index_dir() -> PathBuf {
    get_config_dir().join("indices")
}

/// 根据项目名称获取索引目录
pub fn get_index_path(project_name: &str) -> PathBuf {
    get_index_dir().join(project_name)
}

/// 加载配置文件（不存在则创建并返回默认配置）
pub fn load_config() -> Result<Config, String> {
    let config_path = get_config_path();

    if config_path.exists() {
        let content =
            fs::read_to_string(&config_path).map_err(|e| format!("读取配置文件失败: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))
    } else {
        let default_config = Config::default();

        fs::create_dir_all(get_config_dir())
            .map_err(|e| format!("创建配置目录失败: {}", e))?;

        let json =
            serde_json::to_string_pretty(&default_config).map_err(|e| format!("序列化配置失败: {}", e))?;
        fs::write(&config_path, &json)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;

        Ok(default_config)
    }
}

/// 保存配置文件
pub fn save_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_path();
    fs::create_dir_all(get_config_dir())
        .map_err(|e| format!("创建配置目录失败: {}", e))?;

    let json =
        serde_json::to_string_pretty(config).map_err(|e| format!("序列化配置失败: {}", e))?;
    fs::write(&config_path, &json).map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}
