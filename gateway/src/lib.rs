//!
//!
//!
//!
//!

use std::sync::Arc;

use database::driver::mysql::storage::MysqlStorage;
use git::lfs::LfsConfig;
use https::HttpOptions;
use webhook::WebhookOptions;
pub mod https;
pub mod ssh;
pub mod webhook;
mod model;
mod api_service;


impl From<HttpOptions> for LfsConfig {
    fn from(value: HttpOptions) -> Self {
        Self {
            host: value.host,
            port: value.port,
            lfs_content_path: value.lfs_content_path,
            storage: Arc::new(MysqlStorage::default()),
        }
    }
}

impl From<WebhookOptions> for LfsConfig {
    fn from(value: WebhookOptions) -> Self {
        Self {
            host: value.host,
            port: value.port,
            lfs_content_path: value.lfs_content_path,
            storage: Arc::new(MysqlStorage::default()),
        }
    }
}

#[cfg(test)]
mod tests {}
