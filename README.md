# Code Indexer

代码语义索引与搜索工具 - 基于 Tauri 2.0 + Rust 构建的桌面应用

## 功能特性

- **多语言代码扫描** - 支持 20+ 编程语言的文件识别
- **智能代码分块** - 滑动窗口算法，保留上下文重叠
- **语义符号提取** - 自动识别函数、类、结构体等代码符号
- **向量索引存储** - 高效的二进制向量存储格式
- **余弦相似度搜索** - 语义级别的代码搜索功能
- **实时进度反馈** - 索引过程实时展示

## 支持的语言

TypeScript, JavaScript, Python, Rust, Go, Java, C/C++, C#, Ruby, PHP, Swift, Kotlin, Scala, Vue, Svelte 等

## 技术栈

- **前端**: HTML5 + Vanilla JavaScript
- **后端**: Rust + Tauri 2.0
- **向量引擎**: 阿里云 DashScope API (text-embedding-v4)
- **存储**: JSON + 二进制向量文件

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows 10/11, macOS 10.15+, Linux

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run dev
# 或
npm run tauri dev
```

### 构建发布

```bash
npm run build
# 或
npm run tauri build
```

## 使用说明

1. **配置 API** - 点击右上角设置，输入阿里云 DashScope API Key
2. **选择文件夹** - 从左侧文件浏览器选择代码仓库
3. **创建索引** - 点击「开始索引」建立向量索引
4. **语义搜索** - 选择已索引的项目，输入查询内容进行搜索

## 项目结构

```
tauri-app/
├── src/                      # 前端资源
│   └── index.html           # 单文件应用 (HTML + CSS + JS)
├── src-tauri/
│   ├── src/                 # Rust 后端源码
│   │   ├── lib.rs          # 入口模块
│   │   ├── main.rs         # 主程序
│   │   ├── commands.rs     # Tauri 命令
│   │   ├── api.rs          # 嵌入 API 调用
│   │   ├── models.rs       # 数据结构
│   │   ├── config.rs       # 配置管理
│   │   ├── constants.rs    # 常量定义
│   │   └── indexer/        # 索引模块
│   │       ├── scanner.rs   # 文件扫描
│   │       ├── chunker.rs  # 代码分块
│   │       ├── symbols.rs   # 符号提取
│   │       └── storage.rs  # 持久化存储
│   └── tauri.conf.json     # Tauri 配置
├── package.json
└── README.md
```

## 配置说明

配置文件位置: `~/.config/code-indexer/config.json`

```json
{
  "api_key": "your-api-key",
  "model": "text-embedding-v4",
  "base_url": "https://dashscope.aliyuncs.com/api/v1/services/embeddings/text-embedding"
}
```

索引数据位置: `~/.config/code-indexer/indices/`

## License

MIT
