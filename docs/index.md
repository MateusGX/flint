---
layout: home

hero:
  name: Flint
  text: Assembly-like APIs and web systems
  tagline: Flint explores how a small register-based language can build HTTP APIs, JSON services, and server-rendered UI pages.
  actions:
    - theme: brand
      text: Start Learning
      link: /guide/introduction
    - theme: brand
      text: Build Your First API
      link: /guide/first-api
    - theme: alt
      text: Language Reference
      link: /reference/language

features:
  - title: Assembly-like HTTP
    details: Declare routes in .fl files, move values through registers, call http.* natives, and return text, HTML, or JSON.
  - title: Styled UI pages
    details: Build server-rendered pages from pages/**/*.flint.ui with section .route and section .render.
  - title: Native runtime bridge
    details: Use string.*, json.*, math.*, time.*, env.*, crypto.*, and http.* natives without adding new bytecode instructions.
  - title: Visible VM model
    details: Registers, stack, calls, jumps, memory, bytecode, string pools, and native calls stay explicit and inspectable.
  - title: Project tooling
    details: Use the Flint CLI to scaffold, serve, and build projects; use the VS Code extension for .fl and .flint.ui files.
---
