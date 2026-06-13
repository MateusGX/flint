use std::path::PathBuf;

pub fn minimal(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (PathBuf::from("flint.toml"), manifest(name)),
        (PathBuf::from("routes/hello.fl"), hello(name)),
        (PathBuf::from("pages/index.flint.ui"), home_page(name)),
    ]
}

pub fn tasks(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (PathBuf::from("flint.toml"), manifest(name)),
        (PathBuf::from("routes/tasks.fl"), tasks_controller()),
        (PathBuf::from("services/tasks.fl"), tasks_service()),
        (PathBuf::from("repositories/tasks.fl"), tasks_repository()),
        (PathBuf::from("routes/hello.fl"), hello(name)),
        (PathBuf::from("pages/index.flint.ui"), home_page(name)),
    ]
}

// ---------------------------------------------------------------------------

fn manifest(name: &str) -> String {
    let name = toml_string(name);
    format!(
        r#"[project]
name    = {name}
version = "0.1.0"

[server]
host        = "127.0.0.1"
port        = 3000
routes      = "routes"
pages       = "pages"
services    = "services"
repositories = "repositories"
"#
    )
}

fn hello(name: &str) -> String {
    let name = flint_string_literal(name);
    format!(
        r#"say_hello:
    mov r0, "Hello from {name}!"
    ncall http.text, r0
    ret

route GET "/hello" -> say_hello
"#
    )
}

fn home_page(name: &str) -> String {
    let name = flint_string_literal(name);
    format!(
        r#"@page "/"
<%
mov r15, "{name}"
ncallr r14, ui.window, r14, r15
mov r15, "This page was rendered from pages/index.flint.ui without writing HTML."
ncallr r14, ui.text, r14, r15
mov r15, "Next steps"
ncallr r14, ui.card, r14, r15
mov r15, "Use UI controls for server-rendered pages, or .flint.html when you want full HTML."
ncallr r14, ui.text, r14, r15
mov r15, "Open the API route"
mov r1, "/hello"
ncallr r14, ui.button, r14, r15, r1
ncallr r14, ui.card_end, r14
ncallr r14, ui.window_end, r14
%>
"#
    )
}

fn toml_string(value: &str) -> String {
    format!("\"{}\"", escape_common_string(value))
}

fn flint_string_literal(value: &str) -> String {
    escape_common_string(value)
}

fn escape_common_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            other => escaped.push(other),
        }
    }
    escaped
}

fn tasks_controller() -> String {
    r#"use "services/tasks.fl"

tasks_controller_list:
    call tasks_service_list
    ncall http.json, r0
    ret

tasks_controller_get:
    mov r0, "id"
    ncallr r0, http.param, r0
    call tasks_service_get
    cmp r1, 1
    je tasks_get_found
    mov r1, 404
    ncall http.set_status, r1
    ncallr r1, json.object
    mov r2, "error"
    mov r3, "not found"
    ncallr r1, json.set, r1, r2, r3
    ncall http.json, r1
    jmp tasks_get_done
tasks_get_found:
    ncall http.json, r0
tasks_get_done:
    ret

tasks_controller_create:
    ncallr r0, http.json_body
    call tasks_service_create
    cmp r1, 1
    je tasks_create_ok
    mov r2, 400
    ncall http.set_status, r2
    ncall http.json, r0
    jmp tasks_create_done
tasks_create_ok:
    mov r2, 201
    ncall http.set_status, r2
    ncall http.json, r0
tasks_create_done:
    ret

route GET  "/tasks"     -> tasks_controller_list
route GET  "/tasks/:id" -> tasks_controller_get
route POST "/tasks"     -> tasks_controller_create
"#
    .to_string()
}

fn tasks_service() -> String {
    r#"use "repositories/tasks.fl"

tasks_service_list:
    call tasks_repository_all
    ret

tasks_service_get:
    call tasks_repository_find_by_id
    ncallr r1, json.stringify, r0
    mov r2, "null"
    ncallr r1, string.equals, r1, r2
    mov r2, 1
    sub r1, r2, r1
    ret

tasks_service_create:
    mov r1, "title"
    ncallr r2, json.get, r0, r1
    ncallr r2, json.stringify, r2
    mov r3, "null"
    ncallr r2, string.equals, r2, r3
    cmp r2, 1
    jne tasks_service_create_ok
    ncallr r0, json.object
    mov r1, "error"
    mov r2, "'title' is required"
    ncallr r0, json.set, r0, r1, r2
    mov r1, 0
    ret
tasks_service_create_ok:
    call tasks_repository_create
    mov r1, 1
    ret
"#
    .to_string()
}

fn tasks_repository() -> String {
    r#"tasks_repository_all:
    mov r0, "{\"1\":{\"id\":\"1\",\"title\":\"Buy milk\",\"done\":false},\"2\":{\"id\":\"2\",\"title\":\"Walk the dog\",\"done\":true}}"
    ncallr r0, json.parse, r0
    ret

tasks_repository_find_by_id:
    mov r1, "{\"1\":{\"id\":\"1\",\"title\":\"Buy milk\",\"done\":false},\"2\":{\"id\":\"2\",\"title\":\"Walk the dog\",\"done\":true}}"
    ncallr r1, json.parse, r1
    ncallr r0, json.get, r1, r0
    ret

tasks_repository_create:
    mov r1, "id"
    mov r2, "3"
    ncallr r0, json.set, r0, r1, r2
    ret
"#.to_string()
}
