#[macro_use]
extern crate log;

mod capabilities;
mod main_loop;
pub mod protocol;

pub use main_loop::main_loop;

use anyhow::anyhow;
use async_std::sync::RwLock;
use futures::channel::mpsc::UnboundedReceiver;
use ra_vfs::{RootEntry, Vfs};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{path::PathBuf, sync::Arc};

pub type Result<T> = anyhow::Result<T>;

/// Deserializes a `T` from a json value.
pub fn from_json<T: DeserializeOwned>(what: &'static str, json: serde_json::Value) -> Result<T> {
    T::deserialize(&json)
        .map_err(|e| anyhow::anyhow!("could not deserialize {}: {}: {}", what, e, json))
}

/// Convert the `T` to a json value
pub fn to_json<T: Serialize>(value: T) -> Result<serde_json::Value> {
    serde_json::to_value(value).map_err(|e| anyhow::anyhow!("could not serialize to json: {}", e))
}

/// Main entry point for the language server
pub async fn run_server_async() -> Result<()> {
    info!("language server started");

    // Setup IO connections
    let mut connection = protocol::Connection::stdio();

    // Wait for a client to connect
    let (initialize_id, initialize_params) = connection.initialize_start().await?;

    let initialize_params =
        from_json::<lsp_types::InitializeParams>("InitializeParams", initialize_params)?;

    let server_capabilities = capabilities::server_capabilities(&initialize_params.capabilities);

    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("mun-language-server"),
            version: Some(String::from(env!("CARGO_PKG_VERSION"))),
        }),
    };

    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    connection
        .initialize_finish(initialize_id, initialize_result)
        .await?;

    if let Some(client_info) = initialize_params.client_info {
        info!(
            "client '{}' {}",
            client_info.name,
            client_info.version.unwrap_or_default()
        );
    }

    // Get the current working directory as fallback
    let cwd = std::env::current_dir()?;
    // Convert the root uri to a PathBuf
    let root = initialize_params
        .root_uri
        .and_then(|it| it.to_file_path().ok())
        .unwrap_or(cwd);
    // Convert the workspace_roots, if these are empy use the root_uri or the cwd
    let workspace_roots = initialize_params
        .workspace_folders
        .map(|workspaces| {
            workspaces
                .into_iter()
                .filter_map(|it| it.uri.to_file_path().ok())
                .collect::<Vec<_>>()
        })
        .filter(|workspaces| !workspaces.is_empty())
        .unwrap_or_else(|| vec![root]);

    main_loop(connection, workspace_roots).await?;

    Ok(())
}

/// Main entry point for the language server
pub fn run_server() -> Result<()> {
    async_std::task::block_on(run_server_async())
}
