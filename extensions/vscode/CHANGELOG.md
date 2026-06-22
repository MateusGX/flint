# Change Log

All notable changes to the "flint-vscode" extension will be documented in this file.

## [Unreleased]

## [0.2.3]

- Added a `Flint: Run Project` command and editor play button that run
  `flint run` from the nearest project manifest.

## [0.2.2]

- Added default language icons for Flint source, Flint UI, and bytecode files
  without requiring users to switch file icon themes.

## [0.2.0]

- Removed the obsolete page-template language contribution, snippets, and
  grammar.
- Updated `.fl` highlighting and snippets for section-based route modules.
- Updated `.flint.ui` highlighting and snippets for `section .route` and
  `section .render`.
- Added autocomplete for `use` and `@use` project paths.
- Added autocomplete for section names, HTTP methods, route/jump labels, and
  Flint UI render commands.
- Synced `ui.*` native autocomplete with the current standard library.
- Added explicit language activation events for Flint files.

## [0.1.1]

- Added autocomplete for instruction mnemonics, registers (`r0`-`r15`), and
  `namespace.name` native calls, with inline signatures and documentation.

## [0.1.0]

- Added Flint syntax highlighting for `.fl` files.
- Added Flint UI syntax highlighting for `.flint.ui` pages.
- Added language configuration and snippets.
