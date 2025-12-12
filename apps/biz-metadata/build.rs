use quote::ToTokens;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    // handler：扫描所有 #[utoipa::path] 生成 paths & 路由。
    // dto：扫描 #[derive(ToSchema)] 生成 components.schemas。
    let handler_dir = manifest_dir.join("src/interface/http/handler");
    let dto_request_dir = manifest_dir.join("src/interface/http/dto/request");
    let dto_response_dir = manifest_dir.join("src/interface/http/dto/response");

    let handler_files = gather_rs_files(&handler_dir);
    let route_groups = collect_routes(&handler_files);
    let handler_names: Vec<String> = route_groups
        .iter()
        .flat_map(|(_, routes)| routes.iter().map(|r| r.name.clone()))
        .collect();
    let body_types = collect_body_types(&handler_files);
    let schemas = collect_schemas(&[dto_request_dir, dto_response_dir], &body_types);

    // 渲染 api_doc.rs，包含 OpenAPI 定义与生成的路由。
    let generated = render_api_doc(&route_groups, &handler_names, &schemas);
    let out_file = out_dir.join("api_doc.rs");
    fs::write(&out_file, generated).expect("write api_doc.rs");
    // 监听 handler/dto 变更。
    println!("cargo:rerun-if-changed={}", handler_dir.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("src/interface/http/dto").display()
    );
}

/// 递归收集给定目录下的所有 .rs 文件（包含子目录），用于 handler/DTO 扫描。
fn gather_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(gather_rs_files(&path));
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }
    files
}

/// 路由信息，包含方法/路径/函数名。
#[derive(Debug, Clone)]
struct RouteInfo {
    method: String,
    path: String,
    name: String,
}

/// 收集被 #[utoipa::path] 标记的 handler，生成路由元信息，并按文件分组。
fn collect_routes(handler_files: &[PathBuf]) -> Vec<(String, Vec<RouteInfo>)> {
    let mut grouped: std::collections::BTreeMap<String, Vec<RouteInfo>> =
        std::collections::BTreeMap::new();
    for path in handler_files {
        let group = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("routes")
            .to_string();
        let content = fs::read_to_string(path).expect("read handler file");
        let mut lines = content.lines().peekable();
        while let Some(line) = lines.next() {
            if line.trim_start().starts_with("#[utoipa::path") {
                let mut attr_lines = vec![line.to_string()];
                for l in lines.by_ref() {
                    attr_lines.push(l.to_string());
                    if l.contains(")]") {
                        break;
                    }
                }
                let attr_block = attr_lines.join("\n");
                let method = parse_method(&attr_block);
                let path_value = parse_path(&attr_block);
                let context_path = parse_context_path(&attr_block);

                for fn_line in lines.by_ref() {
                    let trimmed = fn_line.trim_start();
                    if trimmed.starts_with("pub async fn ") {
                        if let Some(name) = trimmed
                            .strip_prefix("pub async fn ")
                            .and_then(|rest| rest.split('(').next())
                            && let (Some(m), Some(p)) = (method.clone(), path_value.clone())
                        {
                            let full_path = combine_path(context_path.as_deref(), &p);
                            grouped.entry(group.clone()).or_default().push(RouteInfo {
                                method: m,
                                path: full_path,
                                name: name.trim().to_string(),
                            });
                        }
                        break;
                    }
                }
            }
        }
    }
    grouped.into_iter().collect()
}

/// 解析 #[utoipa::path] 中的 body= 类型（含泛型/绝对路径），补齐自动扫描不到的 schema。
fn collect_body_types(handler_files: &[PathBuf]) -> Vec<String> {
    let mut types = BTreeSet::new();
    let markers = ["body"];
    for path in handler_files {
        let content = fs::read_to_string(path).expect("read handler file");
        for attr_block in content.split("#[utoipa::path").skip(1) {
            let token_str = attr_block;
            for marker in markers {
                let mut search = 0;
                while let Some(pos) = token_str[search..].find(marker) {
                    let pos = pos + search;
                    if token_str[..pos].ends_with("request_") {
                        search = pos + marker.len();
                        continue;
                    }
                    let rest = &token_str[pos + marker.len()..];
                    let Some(eq_pos_rel) = rest.find('=') else {
                        break;
                    };
                    let rest_after_eq = &rest[eq_pos_rel + 1..];
                    let rest_after_eq = rest_after_eq.trim_start();
                    let mut end = rest_after_eq.len();
                    for (i, ch) in rest_after_eq.char_indices() {
                        if ch == ',' || ch == ')' {
                            end = i;
                            break;
                        }
                    }
                    let ty = rest_after_eq[..end].trim();
                    if !ty.is_empty() {
                        let normalized = normalize_type(ty);
                        if !normalized.contains("::") && !normalized.contains('<') {
                            search = pos + marker.len() + eq_pos_rel + end;
                            continue;
                        }
                        types.insert(normalized);
                    }
                    search = pos + marker.len() + eq_pos_rel + end;
                }
            }
        }
    }
    types.into_iter().collect()
}

fn normalize_type(raw: &str) -> String {
    raw.split_whitespace().collect::<String>()
}

/// 收集 DTO 中带 ToSchema 的结构体/枚举，并合并额外的类型（如泛型响应包装）。
fn collect_schemas(dirs: &[PathBuf], extra: &[String]) -> Vec<String> {
    let mut set = BTreeSet::new();
    set.extend(extra.iter().cloned());
    for dir in dirs {
        for path in gather_rs_files(dir) {
            let content = fs::read_to_string(&path).expect("read dto file");
            if let Ok(file) = syn::parse_file(&content) {
                for item in file.items {
                    match item {
                        syn::Item::Struct(s) => {
                            if has_to_schema(&s.attrs) {
                                let type_name = s.ident.to_string();
                                if should_skip_schema(&type_name) {
                                    continue;
                                }
                                set.insert(to_schema_path(&path, &type_name));
                            }
                        }
                        syn::Item::Enum(e) => {
                            if has_to_schema(&e.attrs) {
                                let type_name = e.ident.to_string();
                                if should_skip_schema(&type_name) {
                                    continue;
                                }
                                set.insert(to_schema_path(&path, &type_name));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    let mut items: Vec<String> = set.into_iter().collect();
    items.sort();
    items
}

/// 判断 attr 是否包含 `#[derive(ToSchema)]`。
fn has_to_schema(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }
        let tokens = attr.to_token_stream().to_string();
        tokens.contains("ToSchema")
    })
}

fn should_skip_schema(type_name: &str) -> bool {
    matches!(type_name, "ResultResponse" | "PageResultResponse")
}

/// 根据文件路径推导 crate 内的 schema 路径，例如 request/biz_metadata/foo.rs -> crate::interface::http::dto::request::biz_metadata::Foo
fn to_schema_path(file_path: &Path, type_name: &str) -> String {
    let parts: Vec<_> = file_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    let dto_idx = parts.iter().position(|p| p == "dto").unwrap_or(0);
    let sub_path = parts[dto_idx + 1..parts.len() - 1].join("::");
    format!("crate::interface::http::dto::{}::{}", sub_path, type_name)
}

/// 渲染 OpenAPI 定义与按文件分组的自动路由。
fn render_api_doc(
    route_groups: &[(String, Vec<RouteInfo>)],
    handlers: &[String],
    schemas: &[String],
) -> String {
    let paths_joined = handlers
        .iter()
        .map(|h| format!("        crate::interface::http::handler::{h}"))
        .collect::<Vec<_>>()
        .join(",\n");
    let schemas_joined = schemas
        .iter()
        .map(|s| format!("            {s}"))
        .collect::<Vec<_>>()
        .join(",\n");

    let route_defs = route_groups
        .iter()
        .map(|(group, routes)| {
            let routes_str = routes
                .iter()
                .map(|r| {
                    format!(
                        "        .route(\"{}\", {}(crate::interface::http::handler::{}))",
                        r.path, r.method, r.name
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "pub fn generated_routes_{group}(state: crate::interface::http::state::AppState) -> axum::Router<()> {{\n    use axum::routing::{{delete, get, post, put}};\n    let router: axum::Router<crate::interface::http::state::AppState> = axum::Router::new()\n{routes_str};\n    router.with_state(state)\n}}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let template = r#"
use utoipa::OpenApi;
use crate::interface::http::dto::response::{
    BizMetadataAliasResponse, BizMetadataResponse, EmptyPayload, PageResultResponse, ResultResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
{paths}
    ),
    components(
        schemas(
{schemas}
        )
    ),
    tags(
        (name = "biz_metadata", description = "BizMetadata HTTP API"),
        (name = "biz_metadata_alias", description = "BizMetadata Alias HTTP API")
    ),
    modifiers(&TrimTrailingSlash)
)]
pub struct ApiDoc;

pub struct TrimTrailingSlash;

impl utoipa::Modify for TrimTrailingSlash {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut new_paths = utoipa::openapi::path::Paths::new();
        for (path, item) in std::mem::take(&mut openapi.paths.paths) {
            let normalized = if path != "/" && path.ends_with('/') {
                path.trim_end_matches('/').to_string()
            } else {
                path
            };
            new_paths.paths.insert(normalized, item);
        }
        openapi.paths = new_paths;
    }
}

{routes}
"#;

    template
        .replace("{paths}", &paths_joined)
        .replace("{schemas}", &schemas_joined)
        .replace("{routes}", &route_defs)
}

/// 从属性块解析 HTTP 方法。
fn parse_method(attr_block: &str) -> Option<String> {
    for method in ["get", "post", "put", "delete"] {
        if attr_block.contains(&format!("{method},")) || attr_block.contains(&format!("{method}\n"))
        {
            return Some(method.to_string());
        }
    }
    None
}

/// 从属性块解析 path = "..."，支持常量或字面量。
fn parse_path(attr_block: &str) -> Option<String> {
    parse_attr_line(attr_block, "path")
}

/// 从属性块解析 context_path = "..."，支持常量或字面量。
fn parse_context_path(attr_block: &str) -> Option<String> {
    parse_attr_line(attr_block, "context_path")
}

/// 解析类似 `field = "..."` 或 `field = CONST_NAME` 的行。
fn parse_attr_line(attr_block: &str, field: &str) -> Option<String> {
    for line in attr_block.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with(field) {
            continue;
        }
        let Some((_, rhs)) = trimmed.split_once('=') else {
            continue;
        };
        let value = rhs.trim().trim_end_matches(',');
        if let Some(stripped) = value.strip_prefix('"') {
            let end = stripped.find('"')?;
            return Some(stripped[..end].to_string());
        } else if let Some(resolved) = resolve_context_const(value.trim()) {
            return Some(resolved);
        }
    }
    None
}

fn resolve_context_const(name: &str) -> Option<String> {
    match name {
        "BIZ_METADATA_CONTEXT" => Some("/biz_metadata".to_string()),
        "BIZ_METADATA_ALIAS_CONTEXT" => Some("/biz_metadata_alias".to_string()),
        _ => None,
    }
}

/// 组合 context_path 与子路径，确保不会出现重复斜杠。
fn combine_path(context_path: Option<&str>, path: &str) -> String {
    let base = context_path.unwrap_or("").trim_end_matches('/');
    if path == "/" {
        return if base.is_empty() {
            "/".to_string()
        } else {
            base.to_string()
        };
    }
    let sub = path.trim_start_matches('/');
    if base.is_empty() {
        format!("/{}", sub)
    } else if sub.is_empty() {
        base.to_string()
    } else {
        format!("{base}/{sub}")
    }
}
