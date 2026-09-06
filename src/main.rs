//! hex-mcp — binary inspection and patching over MCP.

mod args;
mod policy;
mod tools;

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use mcp_toolkit::ServerOptions;

use policy::Policy;
use tools::HexTools;

#[derive(Parser, Debug)]
#[command(name = "hex-mcp", version, about = "Hex viewing and patching as an MCP server")]
struct Cli {
    #[command(flatten)]
    server: ServerOptions,

    /// Permit `hex_patch` to write. Without this the server is read-only.
    #[arg(long, env = "HEX_MCP_ALLOW_WRITE")]
    allow_write: bool,

    /// Confine every path to this directory. Repeatable. Unrestricted if unset.
    #[arg(long = "root", value_name = "DIR", env = "HEX_MCP_ROOT")]
    roots: Vec<PathBuf>,

    /// Largest file the server will open, in bytes.
    #[arg(long, default_value_t = 64 * 1024 * 1024, env = "HEX_MCP_MAX_FILE_BYTES")]
    max_file_bytes: u64,
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let policy = Policy::new(cli.allow_write, cli.roots, cli.max_file_bytes);
    let group = Arc::new(HexTools::new(policy));

    match mcp_toolkit::run("hex", env!("CARGO_PKG_VERSION"), group, cli.server).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            // stderr, never stdout: stdout carries the JSON-RPC stream.
            eprintln!("hex-mcp: {err}");
            std::process::ExitCode::FAILURE
        }
    }
}
