use gitswitch_lib::{AppEngine, FolderMapping, Profile, APP_VERSION};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: Option<String>,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

fn tool_definitions() -> Value {
    json!([
        {
            "name": "get_status",
            "description": "Get GitSwitch daemon status, active Git/SSH profile, last switch latency, and config file paths.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "list_profiles",
            "description": "List all configured GitHub/Git identity profiles in GitSwitch.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "switch_profile",
            "description": "Switch the active global Git & SSH identity profile, atomically updating ~/.gitconfig, ~/.gitconfig-<id>, and ~/.ssh/config.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": {
                        "type": "string",
                        "description": "Profile ID to activate (e.g. 'work', 'personal', 'client', 'enterprise')."
                    }
                },
                "required": ["profile_id"],
                "additionalProperties": false
            }
        },
        {
            "name": "save_profile",
            "description": "Create or update a GitSwitch identity profile and sync its includeIf folder mappings.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Unique slug identifier (e.g. 'work')." },
                    "name": { "type": "string", "description": "Display name for the profile." },
                    "username": { "type": "string", "description": "Git user.name / GitHub handle." },
                    "email": { "type": "string", "description": "Git committing email (user.email)." },
                    "ssh_key": { "type": "string", "description": "SSH private key filename inside ~/.ssh (e.g. 'id_ed25519_work')." },
                    "signing_key": { "type": "string", "description": "Optional GPG/SSH signing key ID." },
                    "scope_paths": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Directory glob patterns for automatic includeIf.gitdir switching (e.g. ['~/Developer/Work/*'])."
                    },
                    "color": { "type": "string", "description": "Optional hex color accent (defaults to '#3fb950')." },
                    "tag": { "type": "string", "description": "Optional short workspace tag." },
                    "shortcut": { "type": "string", "description": "Optional keyboard shortcut label." }
                },
                "required": ["id", "name", "username", "email", "ssh_key"],
                "additionalProperties": false
            }
        },
        {
            "name": "delete_profile",
            "description": "Delete a GitSwitch profile by ID (fails if it is the sole remaining profile).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": { "type": "string", "description": "Profile ID to remove." }
                },
                "required": ["profile_id"],
                "additionalProperties": false
            }
        },
        {
            "name": "list_folder_mappings",
            "description": "List all configured includeIf.gitdir folder-to-profile mappings.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "save_folder_mapping",
            "description": "Add or update an includeIf.gitdir folder mapping for automatic profile detection.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Optional mapping ID; auto-generated if omitted." },
                    "path_pattern": { "type": "string", "description": "Directory pattern (e.g. '~/Developer/Work/*')." },
                    "profile_id": { "type": "string", "description": "Target profile ID to bind." }
                },
                "required": ["path_pattern", "profile_id"],
                "additionalProperties": false
            }
        },
        {
            "name": "delete_folder_mapping",
            "description": "Unlink and remove a folder mapping by ID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Folder mapping ID to remove." }
                },
                "required": ["id"],
                "additionalProperties": false
            }
        },
        {
            "name": "validate_hook",
            "description": "Test a directory path against configured includeIf.gitdir rules to resolve which profile applies.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Directory path to test (e.g. '~/Developer/Work/repo')." }
                },
                "required": ["path"],
                "additionalProperties": false
            }
        },
        {
            "name": "list_ssh_keys",
            "description": "Scan ~/.ssh/*.pub and list registered SSH cryptographic keys, algorithms, and SHA-256 fingerprints.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "generate_ssh_key",
            "description": "Generate a new Ed25519 SSH keypair in ~/.ssh/ via ssh-keygen and register its SHA-256 fingerprint.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "key_name": { "type": "string", "description": "Key filename inside ~/.ssh (alphanumeric, '_' or '-')." },
                    "email": { "type": "string", "description": "Comment email embedded in the public key." }
                },
                "required": ["key_name", "email"],
                "additionalProperties": false
            }
        },
        {
            "name": "test_ssh_connection",
            "description": "Run an OpenSSH diagnostic handshake probe against git@github.com for a given SSH key.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "key_name": { "type": "string", "description": "SSH key filename inside ~/.ssh (e.g. 'id_ed25519_work')." }
                },
                "required": ["key_name"],
                "additionalProperties": false
            }
        },
        {
            "name": "get_config_files",
            "description": "Inspect the current ~/.gitconfig and ~/.ssh/config file contents on disk.",
            "inputSchema": {
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }
        },
        {
            "name": "apply_to_terminal",
            "description": "Generate PowerShell and POSIX/Bash export commands to override Git/SSH identity in the current shell.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "profile_id": { "type": "string", "description": "Profile ID to generate terminal commands for." }
                },
                "required": ["profile_id"],
                "additionalProperties": false
            }
        }
    ])
}

fn call_tool(engine: &mut AppEngine, name: &str, args: &Value) -> Result<Value, String> {
    match name {
        "get_status" => Ok(serde_json::to_value(engine.get_status()).map_err(|e| e.to_string())?),
        "list_profiles" => {
            Ok(serde_json::to_value(&engine.config.profiles).map_err(|e| e.to_string())?)
        }
        "switch_profile" => {
            let pid = args
                .get("profile_id")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: profile_id")?;
            let res = engine.switch_active_profile(pid)?;
            Ok(serde_json::to_value(res).map_err(|e| e.to_string())?)
        }
        "save_profile" => {
            let id = args
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: id")?
                .to_string();
            let p_name = args
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: name")?
                .to_string();
            let username = args
                .get("username")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: username")?
                .to_string();
            let email = args
                .get("email")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: email")?
                .to_string();
            let ssh_key = args
                .get("ssh_key")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: ssh_key")?
                .to_string();

            let scope_paths = args
                .get("scope_paths")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let profile = Profile {
                id: id.clone(),
                name: p_name,
                username,
                email,
                ssh_key: ssh_key.clone(),
                ssh_key_path: Some(format!("~/.ssh/{}", ssh_key)),
                signing_key: args
                    .get("signing_key")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                scope_paths,
                color: args
                    .get("color")
                    .and_then(|v| v.as_str())
                    .unwrap_or("#3fb950")
                    .to_string(),
                tag: args
                    .get("tag")
                    .and_then(|v| v.as_str())
                    .unwrap_or("identity")
                    .to_string(),
                shortcut: args
                    .get("shortcut")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                custom_ssh_command: None,
            };
            engine.save_profile(profile)?;
            Ok(json!({ "saved": true, "profile_id": id }))
        }
        "delete_profile" => {
            let pid = args
                .get("profile_id")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: profile_id")?;
            engine.delete_profile(pid)?;
            Ok(json!({ "deleted": true, "profile_id": pid }))
        }
        "list_folder_mappings" => {
            Ok(serde_json::to_value(&engine.config.folder_mappings).map_err(|e| e.to_string())?)
        }
        "save_folder_mapping" => {
            let path_pattern = args
                .get("path_pattern")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: path_pattern")?
                .to_string();
            let profile_id = args
                .get("profile_id")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: profile_id")?
                .to_string();
            let target = engine
                .config
                .profiles
                .iter()
                .find(|p| p.id == profile_id)
                .cloned()
                .ok_or_else(|| format!("Profile not found: {}", profile_id))?;

            let id = args
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    format!(
                        "{}-{}",
                        profile_id,
                        path_pattern.replace(['/', '\\', '*'], "_")
                    )
                });

            let mapping = FolderMapping {
                id: id.clone(),
                path_pattern,
                profile_id: target.id.clone(),
                profile_name: target.name,
                is_active: engine.config.settings.active_profile_id == target.id,
                ssh_key: target.ssh_key,
            };
            engine.save_folder_mapping(mapping)?;
            Ok(json!({ "saved": true, "id": id }))
        }
        "delete_folder_mapping" => {
            let id = args
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: id")?;
            engine.delete_folder_mapping(id)?;
            Ok(json!({ "deleted": true, "id": id }))
        }
        "validate_hook" => {
            let path = args
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: path")?;
            Ok(serde_json::to_value(engine.validate_hook(path)).map_err(|e| e.to_string())?)
        }
        "list_ssh_keys" => {
            engine.scan_ssh_keys();
            Ok(serde_json::to_value(&engine.config.ssh_keys).map_err(|e| e.to_string())?)
        }
        "generate_ssh_key" => {
            let key_name = args
                .get("key_name")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: key_name")?;
            let email = args
                .get("email")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: email")?;
            let info = engine.generate_ssh_key(key_name, email)?;
            Ok(serde_json::to_value(info).map_err(|e| e.to_string())?)
        }
        "test_ssh_connection" => {
            let key_name = args
                .get("key_name")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: key_name")?;
            Ok(serde_json::to_value(engine.test_ssh_connection(key_name))
                .map_err(|e| e.to_string())?)
        }
        "get_config_files" => {
            Ok(serde_json::to_value(engine.get_config_files()).map_err(|e| e.to_string())?)
        }
        "apply_to_terminal" => {
            let pid = args
                .get("profile_id")
                .and_then(|v| v.as_str())
                .ok_or("Missing required argument: profile_id")?;
            let script = engine.apply_to_terminal(pid)?;
            Ok(json!({ "profile_id": pid, "script": script }))
        }
        _ => Err(format!("Unknown tool: {}", name)),
    }
}

fn handle_request_with_engine(req: JsonRpcRequest, engine: &mut AppEngine) -> Option<JsonRpcResponse> {
    let id = match req.id {
        Some(id) => id,
        None => return None, // Notification: do not respond
    };

    let response = match req.method.as_str() {
        "initialize" => {
            let requested_version = req
                .params
                .get("protocolVersion")
                .and_then(|v| v.as_str())
                .unwrap_or("2024-11-05");
            let protocol_version = match requested_version {
                "2025-03-26" | "2024-11-05" => requested_version,
                _ => "2024-11-05",
            };
            JsonRpcResponse {
                jsonrpc: "2.0",
                id,
                result: Some(json!({
                    "protocolVersion": protocol_version,
                    "capabilities": {
                        "tools": { "listChanged": false },
                        "resources": { "subscribe": false, "listChanged": false }
                    },
                    "serverInfo": {
                        "name": "gitswitch-mcp",
                        "version": APP_VERSION
                    }
                })),
                error: None,
            }
        }
        "ping" => JsonRpcResponse {
            jsonrpc: "2.0",
            id,
            result: Some(json!({})),
            error: None,
        },
        "tools/list" => JsonRpcResponse {
            jsonrpc: "2.0",
            id,
            result: Some(json!({ "tools": tool_definitions() })),
            error: None,
        },
        "tools/call" => {
            let tool_name = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let args = req
                .params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));

            match call_tool(engine, tool_name, &args) {
                Ok(val) => {
                    let text = serde_json::to_string_pretty(&val).unwrap_or_else(|_| val.to_string());
                    JsonRpcResponse {
                        jsonrpc: "2.0",
                        id,
                        result: Some(json!({
                            "content": [{ "type": "text", "text": text }],
                            "isError": false
                        })),
                        error: None,
                    }
                }
                Err(err) => JsonRpcResponse {
                    jsonrpc: "2.0",
                    id,
                    result: Some(json!({
                        "content": [{ "type": "text", "text": err }],
                        "isError": true
                    })),
                    error: None,
                },
            }
        }
        "resources/list" => JsonRpcResponse {
            jsonrpc: "2.0",
            id,
            result: Some(json!({
                "resources": [
                    {
                        "uri": "gitswitch://config/gitconfig",
                        "name": "Global Git Config (~/.gitconfig)",
                        "description": "Current host ~/.gitconfig managed by GitSwitch",
                        "mimeType": "text/plain"
                    },
                    {
                        "uri": "gitswitch://config/sshconfig",
                        "name": "SSH Client Config (~/.ssh/config)",
                        "description": "Current host ~/.ssh/config managed by GitSwitch",
                        "mimeType": "text/plain"
                    },
                    {
                        "uri": "gitswitch://state/profiles",
                        "name": "GitSwitch Profiles & Active State",
                        "description": "Configured GitSwitch profiles and folder mappings",
                        "mimeType": "application/json"
                    }
                ]
            })),
            error: None,
        },
        "resources/read" => {
            let uri = req
                .params
                .get("uri")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let files = engine.get_config_files();
            match uri {
                "gitswitch://config/gitconfig" => JsonRpcResponse {
                    jsonrpc: "2.0",
                    id,
                    result: Some(json!({
                        "contents": [{
                            "uri": uri,
                            "mimeType": "text/plain",
                            "text": files.gitconfig_content
                        }]
                    })),
                    error: None,
                },
                "gitswitch://config/sshconfig" => JsonRpcResponse {
                    jsonrpc: "2.0",
                    id,
                    result: Some(json!({
                        "contents": [{
                            "uri": uri,
                            "mimeType": "text/plain",
                            "text": files.sshconfig_content
                        }]
                    })),
                    error: None,
                },
                "gitswitch://state/profiles" => JsonRpcResponse {
                    jsonrpc: "2.0",
                    id,
                    result: Some(json!({
                        "contents": [{
                            "uri": uri,
                            "mimeType": "application/json",
                            "text": serde_json::to_string_pretty(&engine.config).unwrap_or_default()
                        }]
                    })),
                    error: None,
                },
                _ => JsonRpcResponse {
                    jsonrpc: "2.0",
                    id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: format!("Unknown resource URI: {}", uri),
                    }),
                },
            }
        }
        other => JsonRpcResponse {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", other),
            }),
        },
    };

    Some(response)
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let req: JsonRpcRequest = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(err) => {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: Value::Null,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", err),
                    }),
                };
                let _ = serde_json::to_writer(&mut out, &resp);
                let _ = out.write_all(b"\n");
                let _ = out.flush();
                continue;
            }
        };

        // Reload fresh config from disk each request to stay in sync with the GUI app
        let mut engine = AppEngine::new();
        if let Some(resp) = handle_request_with_engine(req, &mut engine) {
            if serde_json::to_writer(&mut out, &resp).is_ok() {
                let _ = out.write_all(b"\n");
                let _ = out.flush();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_engine(name: &str) -> (std::path::PathBuf, AppEngine) {
        let tmp = std::env::temp_dir().join(name);
        let _ = fs::remove_dir_all(&tmp);
        let mut engine = AppEngine::new_in_dirs(tmp.join("home"), tmp.join("cfg"));
        engine
            .save_profile(Profile {
                id: "work".to_string(),
                name: "Work".to_string(),
                username: "test-work".to_string(),
                email: "work@example.com".to_string(),
                ssh_key: "id_ed25519_work".to_string(),
                ssh_key_path: Some("~/.ssh/id_ed25519_work".to_string()),
                signing_key: None,
                scope_paths: vec!["~/Projects/Work/*".to_string()],
                color: "#3fb950".to_string(),
                tag: "corp-id".to_string(),
                shortcut: Some("⌘1".to_string()),
                custom_ssh_command: None,
            })
            .unwrap();
        engine
            .save_profile(Profile {
                id: "personal".to_string(),
                name: "Personal".to_string(),
                username: "test-personal".to_string(),
                email: "personal@example.com".to_string(),
                ssh_key: "id_ed25519_personal".to_string(),
                ssh_key_path: Some("~/.ssh/id_ed25519_personal".to_string()),
                signing_key: None,
                scope_paths: vec!["~/Projects/Personal/*".to_string()],
                color: "#6366f1".to_string(),
                tag: "personal".to_string(),
                shortcut: Some("⌘2".to_string()),
                custom_ssh_command: None,
            })
            .unwrap();
        (tmp, engine)
    }

    #[test]
    fn mcp_initialize_and_list_tools() {
        let (tmp, mut engine) = temp_engine("gitswitch_mcp_test_init");
        let init_req = JsonRpcRequest {
            jsonrpc: Some("2.0".to_string()),
            id: Some(json!(1)),
            method: "initialize".to_string(),
            params: json!({ "protocolVersion": "2024-11-05" }),
        };
        let init_resp = handle_request_with_engine(init_req, &mut engine).unwrap();
        let res = init_resp.result.unwrap();
        assert_eq!(res["serverInfo"]["name"], "gitswitch-mcp");
        assert_eq!(res["serverInfo"]["version"], "1.0.0");

        let list_req = JsonRpcRequest {
            jsonrpc: Some("2.0".to_string()),
            id: Some(json!(2)),
            method: "tools/list".to_string(),
            params: json!({}),
        };
        let list_resp = handle_request_with_engine(list_req, &mut engine).unwrap();
        let tools = list_resp.result.unwrap()["tools"].as_array().unwrap().clone();
        assert!(tools.iter().any(|t| t["name"] == "switch_profile"));
        assert!(tools.iter().any(|t| t["name"] == "validate_hook"));
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn mcp_tools_call_switch_and_validate() {
        let (tmp, mut engine) = temp_engine("gitswitch_mcp_test_call");
        let switch_req = JsonRpcRequest {
            jsonrpc: Some("2.0".to_string()),
            id: Some(json!(3)),
            method: "tools/call".to_string(),
            params: json!({
                "name": "switch_profile",
                "arguments": { "profile_id": "personal" }
            }),
        };
        let switch_resp = handle_request_with_engine(switch_req, &mut engine).unwrap();
        let result = switch_resp.result.unwrap();
        assert_eq!(result["isError"], false);
        assert!(result["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("\"profile_id\": \"personal\""));

        let hook_req = JsonRpcRequest {
            jsonrpc: Some("2.0".to_string()),
            id: Some(json!(4)),
            method: "tools/call".to_string(),
            params: json!({
                "name": "validate_hook",
                "arguments": { "path": "~/Projects/Work/api-gateway" }
            }),
        };
        let hook_resp = handle_request_with_engine(hook_req, &mut engine).unwrap();
        let hook_res = hook_resp.result.unwrap();
        assert_eq!(hook_res["isError"], false);
        assert!(hook_res["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("\"matched\": true"));
        let _ = fs::remove_dir_all(&tmp);
    }
}

