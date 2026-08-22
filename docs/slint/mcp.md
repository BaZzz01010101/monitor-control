# Slint MCP development

The tray application uses Slint 1.17.1. Slint's embedded MCP server is an
opt-in development feature: normal builds do not start a server and do not
include the MCP feature.

## Run the tray application with MCP

From PowerShell, set the debug metadata variable before building and choose a
free local port before starting the application:

```powershell
$env:SLINT_EMIT_DEBUG_INFO = "1"
$env:SLINT_MCP_PORT = "9315"
cargo run -p dell-controller-tray --features slint/mcp
```

`SLINT_EMIT_DEBUG_INFO` is consumed while the `.slint` files are compiled. It
preserves element IDs and source metadata needed for UI-tree inspection. The
`mcp` feature is intentionally passed on the command line; it should not be
added to `Cargo.toml`.

The server listens only on the loopback interface at the `/mcp` endpoint. The
application uses its normal Windows renderer in this command, so screenshots
represent the actual window and Windows scaling configuration.

For a windowless software-rendered run, use the MCP-oriented headless backend:

```powershell
$env:SLINT_EMIT_DEBUG_INFO = "1"
$env:SLINT_MCP_PORT = "9315"
$env:SLINT_BACKEND = "headless-software"
cargo run -p dell-controller-tray --features slint/mcp
```

Headless screenshots are useful for automation, but they do not validate
Windows DPI, GPU rendering, native tray behavior, or monitor integration.

## Connect and capture a screenshot

The MCP transport is Streamable HTTP. The following request lists the running
windows:

```powershell
$headers = @{
    "Content-Type" = "application/json"
    "Accept" = "application/json, text/event-stream"
}
$body = '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_windows","arguments":{}}}'
Invoke-RestMethod -Method Post -Uri "http://127.0.0.1:9315/mcp" -Headers $headers -Body $body
```

Use the returned `windowHandle` with `take_screenshot`. Other useful tools
include `get_element_tree`, `get_element_properties`, `find_elements_by_id`,
`click_element`, `drag_element`, `set_element_value`, and
`dispatch_key_event`.

The MCP server exposes the Slint window. The native `tray-icon` menu is not a
Slint item tree and is therefore outside MCP inspection.

## VS Code

The `Tray: Dell Controller (Slint MCP)` launch configuration builds with
debug metadata and the `slint/mcp` feature, then starts the tray binary with
port `9315`. Start that configuration, open the application window, and point
an MCP client at `http://127.0.0.1:9315/mcp`.
