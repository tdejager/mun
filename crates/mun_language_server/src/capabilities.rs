use lsp_types::{
    ClientCapabilities, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};

/// Returns the capabilities of this LSP server implementation given the capabilities of the client.
pub fn server_capabilities(_client_caps: &ClientCapabilities) -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full)),
        ..Default::default()
    }
}
