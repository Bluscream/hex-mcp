# hex-mcp

> ## ⚠️ Superseded by [common-mcp](https://github.com/Bluscream/common-mcp)
>
> These tools now live in **[common-mcp](https://github.com/Bluscream/common-mcp)**,
> which serves them alongside the rest of the family from one process. The tool
> names and arguments are unchanged, so switching is only a change of command:
>
> ```diff
> - "command": "/path/to/hex-mcp"
> + "command": "/path/to/common-mcp"
> ```
>
> Beyond what this server did, common-mcp enforces the file size cap through a single shared policy, so `hex_view` and `hex_patch` are confined exactly as the filesystem tools are.
>
> This repository is archived and will not receive further changes. The release
> below remains downloadable.

Hex viewing and byte-patching of binary files, as an MCP server.

```bash
hex-mcp --root /path/to/files                 # read-only, stdio
hex-mcp --root /work --allow-write            # patching enabled
hex-mcp --transport http --auth-token "$TOK"  # over HTTP
hex-mcp --single-tool                         # one tool instead of two
hex-mcp --list-tools                          # inspect and exit
```

## Tools

| Tool | Does |
| --- | --- |
| `hex_view` | Dumps a byte range as offsets, hex columns and an ASCII gutter. Reads only the requested window, so it is safe on very large files. |
| `hex_patch` | Replaces every occurrence of a hex pattern, or writes bytes at an offset. Previews by default; `apply: true` writes and leaves a `.omni-bak` backup. |

Both accept an optional `timeout` (seconds), clamped to `--max-timeout`.

## Safety

This server can rewrite arbitrary bytes on disk, so the dangerous parts are off
by default:

- **Read-only** unless `--allow-write`.
- **Confined** to `--root` directories (repeatable). Symlinks and `..` cannot
  escape them. Unrestricted only if you pass no roots at all.
- **Size-capped** by `--max-file-bytes` (default 64 MiB). `hex_patch` reads the
  whole file to rewrite it, so without a cap a large file exhausts memory.
- **Preview-first**: `hex_patch` reports what would change unless `apply: true`.
- **Size-preserving**: a patch that would change the file's length is refused
  unless `allow_resize: true`, because resizing usually corrupts a binary.
- **HTTP requires a bearer token** unless `--allow-unauthenticated` is passed.

## Options

Everything from [`mcp-toolkit`](https://github.com/Bluscream/mcp-toolkit)
(`--transport`, `--single-tool`, `--timeout`, `--bind`, `--auth-token`,
`--list-tools`), plus:

| Flag | Env | Default |
| --- | --- | --- |
| `--allow-write` | `HEX_MCP_ALLOW_WRITE` | off |
| `--root DIR` | `HEX_MCP_ROOT` | unrestricted |
| `--max-file-bytes N` | `HEX_MCP_MAX_FILE_BYTES` | 67108864 |

`--transport sse` is accepted as an alias for `http`; the standalone SSE
transport was deprecated in the MCP spec in favour of Streamable HTTP.

## IDE configuration

```json
{
  "mcpServers": {
    "hex": {
      "command": "/path/to/hex-mcp",
      "args": ["--root", "/path/to/files"]
    }
  }
}
```

## Development

```bash
./scripts/build.sh --release
```

Runs `cargo fmt --check`, `cargo clippy -D warnings` (max 100 lines per
function) and the tests (max 1000 lines per file).

## License

[Unlicense](LICENSE) (public domain).
