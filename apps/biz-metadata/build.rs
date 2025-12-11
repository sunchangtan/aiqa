use quote::ToTokens;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    let handler_path = manifest_dir.join("src/interface/http/handler.rs");
    let dto_request_dir = manifest_dir.join("src/interface/http/dto/request");
    let dto_response_dir = manifest_dir.join("src/interface/http/dto/response");

    let handlers = collect_handlers(&handler_path);
    let body_types = collect_body_types(&handler_path);
    let schemas = collect_schemas(&[dto_request_dir, dto_response_dir], &body_types);

    let generated = render_api_doc(&handlers, &schemas);
    let out_file = out_dir.join("api_doc.rs");
    fs::write(&out_file, generated).expect("write api_doc.rs");
    println!("cargo:rerun-if-changed={}", handler_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("src/interface/http/dto").display()
    );
}

fn collect_handlers(handler_path: &Path) -> Vec<String> {
    let content = fs::read_to_string(handler_path).expect("read handler.rs");
    let mut handlers = Vec::new();
    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("#[utoipa::path") {
            // advance until we see a function definition
            for fn_line in lines.by_ref() {
                let trimmed = fn_line.trim_start();
                if trimmed.starts_with("pub async fn ") {
                    if let Some(name) = trimmed
                        .strip_prefix("pub async fn ")
                        .and_then(|rest| rest.split('(').next())
                    {
                        handlers.push(name.trim().to_string());
                    }
                    break;
                }
            }
        }
    }
    handlers
}

fn collect_body_types(handler_path: &Path) -> Vec<String> {
    let content = fs::read_to_string(handler_path).expect("read handler.rs");
    let mut types = BTreeSet::new();
    let markers = ["body"];
    for attr_block in content.split("#[utoipa::path").skip(1) {
        // tokens start after attribute marker
        let token_str = attr_block;
        for marker in markers {
            let mut search = 0;
            while let Some(pos) = token_str[search..].find(marker) {
                let pos = pos + search;
                // 跳过 request_body 等前缀
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
                    // 仅保留包含泛型或绝对路径的类型，避免与 DTO 扫描重复。
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
    types.into_iter().collect()
}

fn normalize_type(raw: &str) -> String {
    raw.split_whitespace().collect::<String>()
}

fn collect_schemas(dirs: &[PathBuf], extra: &[String]) -> Vec<String> {
    let mut set = BTreeSet::new();
    set.extend(extra.iter().cloned());
    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                    continue;
                }
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
    }
    // 去重后按字母序输出，避免重复短名/长名。
    let mut items: Vec<String> = set.into_iter().collect();
    items.sort();
    items
}

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

fn to_schema_path(file_path: &Path, type_name: &str) -> String {
    // file_path ends with .../dto/<subdir>/<file>.rs
    let parts: Vec<_> = file_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    let dto_idx = parts.iter().position(|p| p == "dto").unwrap_or(0);
    let sub_path = parts[dto_idx + 1..parts.len() - 1].join("::");
    format!("crate::interface::http::dto::{}::{}", sub_path, type_name)
}

fn render_api_doc(handlers: &[String], schemas: &[String]) -> String {
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

    let template = r#"
use axum::{{routing::{{delete, get, post, put}}}};
use utoipa::OpenApi;
use crate::interface::http::{{handler, state::AppState}};
use crate::interface::http::dto::response::{{ResultResponse, PageResultResponse, BizMetadataResponse, EmptyPayload}};

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
        (name = "biz_metadata", description = "BizMetadata HTTP API")
    ),
    modifiers(&BizMetadataPathPrefix)
)]
pub struct ApiDoc;

pub fn build_generated_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/", post(handler::create_biz_metadata))
        .route("/{id}", put(handler::update_biz_metadata))
        .route("/{id}", get(handler::get_biz_metadata))
        .route("/{id}", delete(handler::delete_biz_metadata))
        .route("/", get(handler::list_biz_metadata))
        .with_state(state)
}
"#;

    template
        .replace("{paths}", &paths_joined)
        .replace("{schemas}", &schemas_joined)
}
