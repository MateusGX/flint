use std::path::PathBuf;

use crate::util::{escape, toml_string};

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
        (PathBuf::from("components/navbar.fl"), tasks_navbar(name)),
        (PathBuf::from("pages/index.flint.ui"), home_page(name)),
    ]
}

pub fn site(name: &str) -> Vec<(PathBuf, String)> {
    vec![
        (PathBuf::from("flint.toml"), manifest(name)),
        (PathBuf::from("components/navbar.fl"), site_navbar()),
        (PathBuf::from("pages/index.flint.ui"), static_home_page(name)),
        (PathBuf::from("pages/about.flint.ui"), static_about_page(name)),
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
host         = "127.0.0.1"
port         = 3000
routes       = "routes"
pages        = "pages"
services     = "services"
repositories = "repositories"
components   = "components"
"#
    )
}

fn hello(name: &str) -> String {
    let name = escape(name);
    format!(
        r#"section .route
    GET "/hello" -> say_hello

section .text
say_hello:
    mov r0, "Hello from {name}!"
    ncall http.text, r0
    ret
"#
    )
}

fn home_page(name: &str) -> String {
    let name = escape(name);
    format!(
        r#"@use "components/navbar.fl"

section .route
    GET "/"

section .render
    window "{name}"
        call app_navbar
        card "Next steps"
            text "This page was rendered from pages/index.flint.ui without writing HTML."
            btn "Open the API route", "/hello"
            btn "View tasks", "/tasks"
        end
    end
"#
    )
}

fn tasks_navbar(name: &str) -> String {
    let name = escape(name);
    format!(
        r#"section .text
app_navbar:
    ncallr r14, ui.navbar, r14
    mov r15, "{name}"
    mov r13, "/"
    ncallr r14, ui.nav_item, r14, r15, r13
    mov r15, "Tasks"
    mov r13, "/tasks"
    ncallr r14, ui.nav_item, r14, r15, r13
    mov r15, "API"
    mov r13, "/hello"
    ncallr r14, ui.nav_item, r14, r15, r13
    ncallr r14, ui.navbar_end, r14
    ret
"#
    )
}

fn site_navbar() -> String {
    r#"section .text
site_navbar:
    ncallr r14, ui.navbar, r14
    mov r15, "Home"
    mov r13, "/"
    ncallr r14, ui.nav_item, r14, r15, r13
    mov r15, "About"
    mov r13, "/about"
    ncallr r14, ui.nav_item, r14, r15, r13
    ncallr r14, ui.navbar_end, r14
    ret
"#
    .to_string()
}

fn static_home_page(name: &str) -> String {
    let name = escape(name);
    format!(
        r#"@use "components/navbar.fl"

section .route
    GET "/"

section .render
    window "{name}"
        call site_navbar
        card "Welcome"
            text "This is a static Flint UI site."
            btn "About", "/about"
        end
    end
"#
    )
}

fn static_about_page(name: &str) -> String {
    let name = escape(name);
    format!(
        r#"@use "components/navbar.fl"

section .route
    GET "/about"

section .render
    window "About {name}"
        call site_navbar
        card "About"
            text "Run flint build --static to export this page to dist/about/index.html."
        end
    end
"#
    )
}

fn tasks_controller() -> String {
    r#"use "services/tasks.fl"

section .route
    GET  "/tasks"     -> tasks_controller_list
    GET  "/tasks/:id" -> tasks_controller_get
    POST "/tasks"     -> tasks_controller_create

section .text
tasks_controller_list:
    call tasks_service_list
    ncall http.json, r0
    ret

tasks_controller_get:
    mov r0, "id"
    ncallr r0, http.param, r0
    call tasks_service_get
    cmp r1, 1
    je tasks_controller_get_found
    mov r1, 404
    ncall http.set_status, r1
    ncallr r1, json.object
    mov r2, "error"
    mov r3, "not found"
    ncallr r1, json.set, r1, r2, r3
    ncall http.json, r1
    jmp tasks_controller_get_done
tasks_controller_get_found:
    ncall http.json, r0
tasks_controller_get_done:
    ret

tasks_controller_create:
    ncallr r0, http.json_body
    call tasks_service_create
    cmp r1, 1
    je tasks_controller_create_ok
    mov r2, 400
    ncall http.set_status, r2
    ncall http.json, r0
    jmp tasks_controller_create_done
tasks_controller_create_ok:
    mov r2, 201
    ncall http.set_status, r2
    ncall http.json, r0
tasks_controller_create_done:
    ret
"#
    .to_string()
}

fn tasks_service() -> String {
    r#"use "repositories/tasks.fl"

section .text
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
    r#"section .text
tasks_repository_all:
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
"#
    .to_string()
}
