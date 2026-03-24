//! Code Indexer - 代码语义索引与搜索工具
//!
//! 这是一个模块化的代码索引系统，支持：
//! - 多语言代码文件扫描
//! - 智能代码分块
//! - 向量化存储
//! - 语义搜索

pub mod api;
pub mod commands;
pub mod config;
pub mod constants;
pub mod models;

pub mod indexer;

// Re-export Tauri commands
pub use commands::{
    get_drives, get_or_create_config, index_folder, list_indices, read_directory,
    search_code, update_config,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_drives,
            read_directory,
            index_folder,
            search_code,
            list_indices,
            get_or_create_config,
            update_config,
        ])
        .run(tauri::generate_context!())
        .expect("运行应用程序时出错");
}
