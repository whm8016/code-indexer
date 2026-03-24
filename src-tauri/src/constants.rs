// ============ 常量定义 ============

/// 需要忽略的目录
pub const IGNORE_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "build",
    ".next",
    "target",
    "__pycache__",
    ".venv",
    "venv",
    "vendor",
    ".idea",
    ".vscode",
    "bin",
    "obj",
    ".cache",
    ".temp",
    "tmp",
    "temp",
];

/// 支持的代码文件扩展名
pub const CODE_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "js", "jsx", "mjs", "cts", "py", "rs", "go", "java", "c", "cpp", "h", "hpp", "cs",
    "rb", "php", "swift", "kt", "scala", "vue", "svelte",
];

/// 其他可选文件（配置文件、文档等）
pub const OTHER_EXTENSIONS: &[&str] = &[
    "md", "json", "yaml", "yml", "toml", "xml", "html", "css", "scss",
    "sql", "sh", "bash", "zsh", "ps1", "bat", "dockerfile",
];

/// 代码块配置
pub const CHUNK_MAX_LINES: usize = 100;
pub const CHUNK_OVERLAP: usize = 10;

/// 批量处理大小
pub const BATCH_SIZE: usize = 10;
