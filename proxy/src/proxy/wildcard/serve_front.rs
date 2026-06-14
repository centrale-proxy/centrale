use actix_files::NamedFile;
use actix_web::HttpRequest;
use config::CentraleConfig;
use std::path::{Component, Path, PathBuf};

/// Resolve a URL path against the static root, rejecting traversal.
fn resolve_safe_path(root: &str, url_path: &str) -> Option<PathBuf> {
    let rel = url_path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel }; // mirrors index_file

    let mut safe = PathBuf::from(root);
    for component in Path::new(rel).components() {
        match component {
            Component::Normal(c) => safe.push(c),
            // reject "..", absolute roots, Windows prefixes, etc.
            _ => return None,
        }
    }
    Some(safe)
}

pub async fn serve_front_end(req: HttpRequest) -> actix_web::HttpResponse {
    let path = match resolve_safe_path(&CentraleConfig::get("FRONT_END_FOLDER"), req.path()) {
        Some(p) => p,
        None => return actix_web::HttpResponse::Forbidden().finish(),
    };
    match NamedFile::open_async(&path).await {
        Ok(file) => file.into_response(&req),
        Err(_) => actix_web::HttpResponse::NotFound().finish(),
    }
}
