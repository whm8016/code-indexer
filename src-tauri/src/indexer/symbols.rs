use crate::constants::CODE_EXTENSIONS;

pub mod patterns {
    pub mod rust {
        pub const FUNCTION: &[&str] = &["fn "];
        pub const STRUCT: &[&str] = &["struct "];
        pub const IMPL: &[&str] = &["impl "];
        pub const ENUM: &[&str] = &["enum "];
        pub const TRAIT: &[&str] = &["trait "];
        pub const PUB: &[&str] = &["pub "];
    }

    pub mod python {
        pub const FUNCTION: &[&str] = &["def ", "async def "];
        pub const CLASS: &[&str] = &["class "];
    }

    pub mod go {
        pub const FUNCTION: &[&str] = &["func "];
        pub const TYPE: &[&str] = &["type "];
        pub const STRUCT: &[&str] = &["struct "];
        pub const INTERFACE: &[&str] = &["interface "];
    }

    pub mod java_like {
        pub const CLASS: &[&str] = &[" class "];
        pub const PUBLIC: &[&str] = &["public "];
        pub const PRIVATE: &[&str] = &["private "];
    }

    pub mod typescript {
        pub const FUNCTION: &[&str] = &["function "];
        pub const CONST: &[&str] = &["const "];
        pub const CLASS: &[&str] = &["class "];
        pub const IMPORT: &[&str] = &["import "];
        pub const EXPORT: &[&str] = &["export "];
    }

    pub mod c_family {
        pub const VOID: &[&str] = &["void "];
        pub const INT: &[&str] = &["int "];
        pub const CLASS: &[&str] = &["class "];
        pub const STRUCT: &[&str] = &["struct "];
    }
}

/// 判断一行是否是符号定义
pub fn is_symbol_line(line: &str, ext: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.len() > 100 {
        return false;
    }

    match ext {
        "rs" => {
            patterns::rust::FUNCTION.iter().any(|p| trimmed.starts_with(p))
                || patterns::rust::STRUCT.iter().any(|p| trimmed.starts_with(p))
                || patterns::rust::IMPL.iter().any(|p| trimmed.starts_with(p))
                || patterns::rust::ENUM.iter().any(|p| trimmed.starts_with(p))
                || patterns::rust::TRAIT.iter().any(|p| trimmed.starts_with(p))
                || patterns::rust::PUB.iter().any(|p| trimmed.starts_with(p))
        }
        "py" => {
            patterns::python::FUNCTION.iter().any(|p| trimmed.starts_with(p))
                || patterns::python::CLASS.iter().any(|p| trimmed.starts_with(p))
        }
        "go" => {
            patterns::go::FUNCTION.iter().any(|p| trimmed.starts_with(p))
                || patterns::go::TYPE.iter().any(|p| trimmed.starts_with(p))
                || patterns::go::STRUCT.iter().any(|p| trimmed.starts_with(p))
                || patterns::go::INTERFACE.iter().any(|p| trimmed.starts_with(p))
        }
        "java" | "kt" | "cs" => {
            patterns::java_like::CLASS.iter().any(|p| trimmed.contains(p))
                || patterns::java_like::PUBLIC.iter().any(|p| trimmed.starts_with(p))
                || patterns::java_like::PRIVATE.iter().any(|p| trimmed.starts_with(p))
        }
        "ts" | "tsx" | "js" | "jsx" | "vue" | "svelte" => {
            patterns::typescript::FUNCTION.iter().any(|p| trimmed.contains(p))
                || patterns::typescript::CONST.iter().any(|p| trimmed.contains(p))
                || patterns::typescript::CLASS.iter().any(|p| trimmed.contains(p))
                || patterns::typescript::IMPORT.iter().any(|p| trimmed.starts_with(p))
                || patterns::typescript::EXPORT.iter().any(|p| trimmed.starts_with(p))
        }
        "c" | "cpp" | "h" | "hpp" => {
            patterns::c_family::VOID.iter().any(|p| trimmed.starts_with(p))
                || patterns::c_family::INT.iter().any(|p| trimmed.starts_with(p))
                || patterns::c_family::CLASS.iter().any(|p| trimmed.starts_with(p))
                || patterns::c_family::STRUCT.iter().any(|p| trimmed.starts_with(p))
        }
        _ => false,
    }
}

/// 从代码内容中提取符号定义
pub fn extract_symbols(content: &str, ext: &str) -> Vec<String> {
    // 如果是未知的扩展名，使用通用的检测
    let use_generic = !CODE_EXTENSIONS.contains(&ext);

    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.len() > 100 {
                return None;
            }

            let is_symbol = if use_generic {
                // 通用检测：包含常见关键字
                trimmed.starts_with("function ")
                    || trimmed.starts_with("fn ")
                    || trimmed.starts_with("def ")
                    || trimmed.starts_with("class ")
                    || trimmed.starts_with("struct ")
                    || trimmed.starts_with("import ")
                    || trimmed.starts_with("export ")
                    || trimmed.starts_with("pub ")
                    || trimmed.starts_with("func ")
            } else {
                is_symbol_line(trimmed, ext)
            };

            if is_symbol {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}
