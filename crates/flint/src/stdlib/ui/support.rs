//! Shared HTML escaping and the default stylesheet used by `ui.*` natives.

use std::sync::Arc;

use crate::stdlib::{arg, expect_str, native};
use crate::vm::{NativeFn, Value};

pub(super) fn append_literal(name: &'static str, fragment: &'static str) -> NativeFn {
    native(move |args| {
        let html = expect_str(arg(args, 0, name)?, name, 0)?;
        Ok(Some(Value::Str(Arc::from(format!("{html}{fragment}")))))
    })
}

pub(super) fn labeled_input(name: &'static str, input_type: Option<&'static str>) -> NativeFn {
    native(move |args| {
        let html = expect_str(arg(args, 0, name)?, name, 0)?;
        let label = expect_str(arg(args, 1, name)?, name, 1)?;
        let field_name = expect_str(arg(args, 2, name)?, name, 2)?;
        let label = escape_html(label);
        let field_name = escape_attr(field_name);
        let type_attr = input_type
            .map(|kind| format!(" type=\"{}\"", escape_attr(kind)))
            .unwrap_or_default();
        Ok(Some(Value::Str(Arc::from(format!(
            "{html}<div class=\"flint-input\"><label>{label}</label><input{type_attr} name=\"{field_name}\"></div>\n"
        )))))
    })
}

pub(super) fn percent(value: &str, max: &str) -> i64 {
    let value: i64 = value.parse().unwrap_or(0);
    let max: i64 = max.parse().unwrap_or(100);
    if max > 0 {
        (value * 100 / max).clamp(0, 100)
    } else {
        0
    }
}

pub(super) fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            other => escaped.push(other),
        }
    }
    escaped
}

pub(super) fn escape_attr(text: &str) -> String {
    escape_html(text)
}

pub const UI_JS: &str = r#"<script>
function flintDialog(id){
  var el=document.getElementById(id);
  el.style.display='flex';
  document.body.style.overflow='hidden';
}
function flintDialogClose(id){
  document.getElementById(id).style.display='none';
  if(!document.querySelector('.flint-dialog-overlay[style*="flex"]')){
    document.body.style.overflow='';
  }
}
function flintAccordionToggle(btn){
  var body=btn.nextElementSibling;
  var isOpen=body.style.display!=='none';
  var acc=btn.closest('.flint-accordion');
  acc.querySelectorAll('.flint-accordion-body').forEach(function(b){b.style.display='none';});
  acc.querySelectorAll('.flint-accordion-header').forEach(function(h){h.classList.remove('active');});
  if(!isOpen){body.style.display='block';btn.classList.add('active');}
}
document.addEventListener('DOMContentLoaded',function(){
  /* Move dialogs to <body> root to escape any stacking context */
  document.querySelectorAll('.flint-dialog-overlay').forEach(function(el){
    document.body.appendChild(el);
    el.addEventListener('click',function(e){if(e.target===el)flintDialogClose(el.id);});
  });
});
document.addEventListener('keydown',function(e){
  if(e.key==='Escape'){
    document.querySelectorAll('.flint-dialog-overlay').forEach(function(d){
      if(d.style.display==='flex')flintDialogClose(d.id);
    });
  }
});
</script>
"#;

pub const UI_CSS: &str = r#"body {
    margin: 0;
    padding: 8px;
    font-family: Verdana, Arial, Helvetica, sans-serif;
    font-size: 12px;
    color: #000000;
    background-color: #c0c0c0;
    background-image: url("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAQAAAAECAYAAACp8Z5+AAAAGklEQVQImWNgYGD4z8BQDwAEgAF/QualIQAAAABJRU5ErkJggg==");
}
a { color: #0000cc; }
a:visited { color: #551a8b; }
a:hover { color: #cc0000; }
h1, h2, h3, p { margin-top: 0; }
h1 {
    margin: 0 0 4px 0;
    font-family: "Times New Roman", Times, serif;
    font-size: 22px;
    font-weight: bold;
    color: #000080;
    text-shadow: 1px 1px 0 #ffffff;
}
h2 {
    margin: 0 0 6px 0;
    font-size: 13px;
    font-weight: bold;
    color: #000080;
}
.flint-window {
    width: 760px;
    margin: 10px auto;
}
.flint-surface {
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    padding: 4px;
}
.flint-header {
    padding: 6px 8px;
    margin-bottom: 6px;
    background: linear-gradient(to right, #000080, #1084d0);
    border-top: 1px solid #ffffff;
    border-left: 1px solid #ffffff;
    border-right: 1px solid #404040;
    border-bottom: 1px solid #404040;
    color: #ffffff;
}
.flint-header p.flint-eyebrow {
    margin: 0 0 2px 0;
    font-size: 10px;
    font-weight: normal;
    color: #c0c0c0;
    text-transform: uppercase;
    letter-spacing: 1px;
}
.flint-header h1 {
    color: #ffffff;
    text-shadow: 1px 1px 0 #000040;
    font-size: 18px;
}
.flint-stack {
    display: block;
    padding: 6px;
}
.flint-stack > * { margin-bottom: 8px; }
.flint-stack > *:last-child { margin-bottom: 0; }
.flint-card {
    background-color: #ece9d8;
    border-top: 1px solid #ffffff;
    border-left: 1px solid #ffffff;
    border-right: 1px solid #808080;
    border-bottom: 1px solid #808080;
    padding: 0;
}
.flint-card > .flint-card-title {
    background: linear-gradient(to right, #316ac5, #4a90d9);
    color: #ffffff;
    font-weight: bold;
    font-size: 11px;
    padding: 3px 6px;
    border-bottom: 1px solid #1a4a8a;
}
.flint-card-body {
    padding: 8px;
}
.flint-section {
    display: block;
}
.flint-section > * { margin-bottom: 6px; }
.flint-row {
    display: table;
    width: 100%;
    border-spacing: 6px 0;
}
.flint-row > * { display: table-cell; vertical-align: middle; }
.flint-column {
    display: block;
}
.flint-column > * { margin-bottom: 6px; }
.flint-text {
    margin: 0 0 4px 0;
    font-size: 11px;
    color: #333333;
    line-height: 1.5;
}
.flint-field {
    display: table;
    width: 100%;
    border-collapse: collapse;
    margin: 0;
    padding: 4px 0;
    border-bottom: 1px dotted #999999;
}
.flint-field:last-child { border-bottom: 0; }
.flint-field dt {
    display: table-cell;
    width: 35%;
    padding-right: 8px;
    font-size: 11px;
    font-weight: bold;
    color: #333333;
    vertical-align: top;
    padding-top: 1px;
}
.flint-field dd {
    display: table-cell;
    margin: 0;
    font-size: 11px;
    color: #000000;
    vertical-align: top;
}
.flint-button {
    display: inline-block;
    min-width: 80px;
    padding: 3px 12px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #000000;
    text-decoration: none;
    text-align: center;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    cursor: pointer;
}
.flint-button:hover {
    background-color: #e8e4d8;
    color: #000000;
}
.flint-button:active {
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    padding: 4px 11px 2px 13px;
}
.flint-form {
    display: block;
}
.flint-form > * { margin-bottom: 8px; }
.flint-input {
    display: block;
}
.flint-input > label {
    display: block;
    font-size: 11px;
    font-weight: bold;
    color: #333333;
    margin-bottom: 3px;
}
.flint-input input {
    width: 100%;
    padding: 2px 4px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    color: #000000;
    background-color: #ffffff;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    box-sizing: border-box;
}
.flint-input input:focus {
    outline: 1px dotted #000080;
    outline-offset: -1px;
}
.flint-button:focus {
    outline: 1px dotted #000000;
    outline-offset: -4px;
}
.flint-navbar {
    display: block;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    padding: 3px 4px 0 4px;
    margin-bottom: 6px;
}
.flint-nav-item {
    display: inline-block;
    padding: 3px 10px 4px 10px;
    margin-right: 2px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #000000;
    text-decoration: none;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 0;
    vertical-align: bottom;
    position: relative;
    bottom: -2px;
}
.flint-nav-item:hover {
    background-color: #ece9d8;
    color: #000080;
}
.flint-alert {
    padding: 6px 10px;
    margin-bottom: 6px;
    font-size: 11px;
    font-weight: bold;
    border-top: 2px solid;
    border-left: 2px solid;
    border-right: 2px solid;
    border-bottom: 2px solid;
}
.flint-alert-info {
    background-color: #dce6f5;
    color: #000080;
    border-top-color: #aac0e8;
    border-left-color: #aac0e8;
    border-right-color: #5a7ab8;
    border-bottom-color: #5a7ab8;
}
.flint-alert-success {
    background-color: #d8f0d8;
    color: #006400;
    border-top-color: #90cc90;
    border-left-color: #90cc90;
    border-right-color: #3a8a3a;
    border-bottom-color: #3a8a3a;
}
.flint-alert-warning {
    background-color: #fff8cc;
    color: #7a5800;
    border-top-color: #e8d870;
    border-left-color: #e8d870;
    border-right-color: #b09030;
    border-bottom-color: #b09030;
}
.flint-alert-error {
    background-color: #f8d8d8;
    color: #800000;
    border-top-color: #e89090;
    border-left-color: #e89090;
    border-right-color: #b83a3a;
    border-bottom-color: #b83a3a;
}
.flint-badge {
    display: inline-block;
    padding: 1px 5px;
    font-size: 10px;
    font-weight: bold;
    color: #ffffff;
    background-color: #316ac5;
    border: 1px solid #1a4a8a;
    text-transform: uppercase;
    letter-spacing: 1px;
    vertical-align: middle;
}
.flint-divider {
    border: 0;
    border-top: 1px solid #808080;
    border-bottom: 1px solid #ffffff;
    margin: 8px 0;
}
.flint-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 11px;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
}
.flint-table th {
    background: linear-gradient(to bottom, #316ac5, #1a4a8a);
    color: #ffffff;
    font-weight: bold;
    text-align: left;
    padding: 4px 8px;
    border-right: 1px solid #1a4a8a;
    white-space: nowrap;
}
.flint-table td {
    padding: 3px 8px;
    border-bottom: 1px solid #c8c4bc;
    border-right: 1px solid #c8c4bc;
    color: #000000;
    vertical-align: top;
}
.flint-table tr:nth-child(even) td { background-color: #ece9e0; }
.flint-table tr:nth-child(odd) td  { background-color: #f5f3ee; }
.flint-table tr:hover td { background-color: #dce6f5; }
.flint-input textarea {
    width: 100%;
    padding: 2px 4px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    color: #000000;
    background-color: #ffffff;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    box-sizing: border-box;
    resize: vertical;
}
.flint-input select {
    width: 100%;
    padding: 2px 4px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    color: #000000;
    background-color: #ffffff;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    box-sizing: border-box;
}
.flint-input textarea:focus,
.flint-input select:focus {
    outline: 1px dotted #000080;
    outline-offset: -1px;
}
.flint-check {
    display: block;
    margin-bottom: 4px;
    padding-left: 8px;
    font-size: 11px;
}
.flint-check label {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    cursor: pointer;
    color: #000000;
    font-weight: normal;
}
.flint-check input[type="checkbox"],
.flint-check input[type="radio"] {
    margin: 0;
    cursor: pointer;
}
.flint-breadcrumb {
    display: block;
    padding: 4px 0;
    margin-bottom: 6px;
    font-size: 11px;
}
.flint-bc-item {
    color: #0000cc;
    text-decoration: none;
}
.flint-bc-item:hover { text-decoration: underline; }
.flint-bc-item::after {
    content: " \00BB ";
    color: #808080;
    text-decoration: none;
}
.flint-bc-item:last-child::after { content: ""; }
.flint-progress {
    width: 100%;
    height: 18px;
    background-color: #ffffff;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    box-sizing: border-box;
    overflow: hidden;
}
.flint-progress-bar {
    height: 100%;
    background: linear-gradient(to bottom, #3a9e3a 0%, #1e7a1e 50%, #3a9e3a 100%);
    transition: width 0s;
}
.flint-tabs {
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    background-color: #d4d0c8;
}
.flint-tab-bar {
    background-color: #d4d0c8;
    padding: 4px 4px 0 4px;
    border-bottom: 2px solid #808080;
}
.flint-tab-btn {
    display: inline-block;
    padding: 3px 12px 4px 12px;
    margin-right: 2px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #000000;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 0;
    cursor: pointer;
    position: relative;
    bottom: -2px;
}
.flint-tab-btn.active {
    background-color: #ece9d8;
    color: #000080;
    z-index: 1;
}
.flint-tab-btn:hover:not(.active) { background-color: #e0ddd4; }
.flint-tab-panels { padding: 8px; background-color: #ece9d8; }
.flint-tab-panel  { display: none; }
.flint-code {
    margin: 0;
    padding: 8px;
    font-family: "Courier New", Courier, monospace;
    font-size: 11px;
    color: #000000;
    background-color: #ffffff;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    white-space: pre;
    overflow-x: auto;
}
.flint-status {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    font-weight: bold;
}
.flint-status-dot {
    display: inline-block;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    border: 1px solid rgba(0,0,0,0.3);
    flex-shrink: 0;
}
.flint-status-online  .flint-status-dot { background-color: #00aa00; }
.flint-status-offline .flint-status-dot { background-color: #808080; }
.flint-status-busy    .flint-status-dot { background-color: #cc0000; }
.flint-status-away    .flint-status-dot { background-color: #cc8800; }
.flint-fieldset {
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    padding: 8px 10px;
    margin: 0;
}
.flint-fieldset legend {
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #000080;
    padding: 0 4px;
}
.flint-pagination {
    display: block;
    text-align: center;
    padding: 6px 0;
    font-size: 11px;
}
.flint-page-item {
    display: inline-block;
    min-width: 22px;
    padding: 2px 6px;
    margin: 0 1px;
    text-align: center;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #000000;
    text-decoration: none;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
}
.flint-page-item:hover { background-color: #e0ddd4; color: #000000; }
.flint-page-current {
    background-color: #000080;
    color: #ffffff;
    border-top: 2px solid #404040;
    border-left: 2px solid #404040;
    border-right: 2px solid #c0c0c0;
    border-bottom: 2px solid #c0c0c0;
}
.flint-toolbar {
    display: block;
    padding: 4px 6px;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    margin-bottom: 6px;
}
.flint-toolbar > * { margin-right: 4px; }
.flint-list {
    margin: 0;
    padding-left: 18px;
    font-size: 11px;
    color: #000000;
}
.flint-list-item { margin-bottom: 3px; }
.flint-stat {
    display: inline-block;
    padding: 8px 14px;
    background-color: #ece9d8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    text-align: center;
    min-width: 90px;
    margin-right: 6px;
    margin-bottom: 6px;
    vertical-align: top;
}
.flint-stat-value {
    display: block;
    font-family: "Times New Roman", Times, serif;
    font-size: 30px;
    font-weight: bold;
    color: #000080;
    line-height: 1;
}
.flint-stat-label {
    display: block;
    font-size: 10px;
    color: #808080;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-top: 4px;
}
.flint-image {
    display: block;
    max-width: 100%;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
}
.flint-empty {
    padding: 20px;
    text-align: center;
    color: #808080;
    font-size: 11px;
    font-style: italic;
    background-color: #f5f3ee;
    border: 1px solid #c8c4bc;
}
.flint-action-bar {
    display: block;
    padding: 6px 8px;
    background-color: #d4d0c8;
    border-top: 1px solid #808080;
    margin-top: 8px;
}
.flint-action-bar > * { margin-right: 6px; }
.flint-tree {
    list-style: none;
    margin: 0;
    padding: 0 0 0 14px;
    font-size: 11px;
    font-family: Verdana, Arial, sans-serif;
}
.flint-tree-leaf { padding: 2px 0; }
.flint-tree-leaf a { color: #0000cc; text-decoration: none; }
.flint-tree-leaf a::before { content: "- "; color: #808080; }
.flint-tree-leaf a:hover { text-decoration: underline; }
.flint-tree-node { padding: 2px 0; }
.flint-tree-label {
    font-weight: bold;
    color: #000080;
    cursor: default;
}
.flint-tree-label::before { content: "[+] "; color: #808080; font-weight: normal; }
.flint-steps {
    display: table;
    width: 100%;
    list-style: none;
    margin: 0 0 8px 0;
    padding: 0;
    border-collapse: collapse;
}
.flint-step {
    display: table-cell;
    text-align: center;
    padding: 5px 8px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #808080;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-bottom: 2px solid #808080;
    border-right: 1px solid #808080;
}
.flint-step:last-child { border-right: 0; }
.flint-step-active {
    background-color: #000080;
    color: #ffffff;
    border-top-color: #404040;
    border-bottom-color: #c0c0c0;
}
.flint-footer {
    margin-top: 12px;
    padding: 6px 8px;
    border-top: 2px solid #808080;
    font-size: 10px;
    color: #808080;
    text-align: center;
}
.flint-footer a { color: #808080; }
.flint-dialog-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.55);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
}
.flint-dialog {
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    min-width: 280px;
    max-width: 480px;
    width: 100%;
    box-shadow: 4px 4px 0 rgba(0,0,0,0.6);
}
.flint-dialog-titlebar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: linear-gradient(to right, #000080, #1084d0);
    color: #ffffff;
    padding: 3px 4px 3px 8px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    user-select: none;
}
.flint-dialog-close {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 14px;
    padding: 0;
    font-family: Verdana, Arial, sans-serif;
    font-size: 10px;
    font-weight: bold;
    color: #000000;
    background-color: #d4d0c8;
    border-top: 1px solid #ffffff;
    border-left: 1px solid #ffffff;
    border-right: 1px solid #808080;
    border-bottom: 1px solid #808080;
    cursor: pointer;
    line-height: 1;
    flex-shrink: 0;
}
.flint-dialog-close:active {
    border-top: 1px solid #808080;
    border-left: 1px solid #808080;
    border-right: 1px solid #ffffff;
    border-bottom: 1px solid #ffffff;
}
.flint-dialog-body {
    padding: 12px 14px;
    background-color: #ece9d8;
    border-top: 1px solid #ffffff;
    border-bottom: 1px solid #808080;
}
.flint-dialog-footer {
    padding: 6px 10px;
    text-align: right;
    background-color: #d4d0c8;
}
.flint-dialog-footer > * + * { margin-left: 4px; }
.flint-layout {
    display: table;
    width: 100%;
    table-layout: fixed;
    border-spacing: 0;
}
.flint-sidebar {
    display: table-cell;
    width: 185px;
    vertical-align: top;
    padding-right: 8px;
}
.flint-main {
    display: table-cell;
    vertical-align: top;
}
.flint-menu {
    display: block;
    background-color: #d4d0c8;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    margin-bottom: 8px;
}
.flint-menu-title {
    display: block;
    padding: 4px 8px;
    font-size: 11px;
    font-weight: bold;
    color: #ffffff;
    background: linear-gradient(to right, #000080, #1084d0);
}
.flint-menu-item {
    display: block;
    padding: 4px 10px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    color: #000000;
    text-decoration: none;
    border-left: 3px solid transparent;
    border-bottom: 1px solid #c8c4bc;
}
.flint-menu-item:last-child { border-bottom: 0; }
.flint-menu-item:hover {
    background-color: #dce6f5;
    border-left-color: #000080;
    color: #000080;
}
.flint-menu-active {
    display: block;
    padding: 4px 10px;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #ffffff;
    text-decoration: none;
    background-color: #000080;
    border-left: 3px solid #c0c0c0;
    border-bottom: 1px solid #1a4a8a;
}
.flint-menu-active:last-child { border-bottom: 0; }
.flint-accordion {
    display: block;
    border-top: 2px solid #ffffff;
    border-left: 2px solid #ffffff;
    border-right: 2px solid #808080;
    border-bottom: 2px solid #808080;
    margin-bottom: 8px;
}
.flint-accordion-item { border-bottom: 1px solid #808080; }
.flint-accordion-item:last-child { border-bottom: 0; }
.flint-accordion-header {
    display: block;
    width: 100%;
    padding: 5px 10px;
    text-align: left;
    font-family: Verdana, Arial, sans-serif;
    font-size: 11px;
    font-weight: bold;
    color: #000000;
    background-color: #d4d0c8;
    border: 0;
    cursor: pointer;
    box-sizing: border-box;
}
.flint-accordion-header::before { content: "[+] "; color: #808080; font-weight: normal; }
.flint-accordion-header.active {
    background: linear-gradient(to right, #000080, #1084d0);
    color: #ffffff;
}
.flint-accordion-header.active::before { content: "[-] "; color: #c0c0c0; }
.flint-accordion-header:hover:not(.active) { background-color: #dce6f5; }
.flint-accordion-body {
    padding: 8px;
    background-color: #ece9d8;
    border-top: 1px solid #808080;
}
.flint-ol {
    margin: 0;
    padding-left: 20px;
    font-size: 11px;
    color: #000000;
}
.flint-ol-item { margin-bottom: 3px; }
.flint-kbd {
    display: inline-block;
    padding: 1px 5px;
    font-family: "Courier New", Courier, monospace;
    font-size: 10px;
    color: #000000;
    background-color: #d4d0c8;
    border-top: 1px solid #ffffff;
    border-left: 1px solid #ffffff;
    border-right: 1px solid #404040;
    border-bottom: 1px solid #404040;
    margin: 0 1px;
}
.flint-caption {
    caption-side: top;
    padding: 4px 6px;
    font-size: 11px;
    font-weight: bold;
    color: #333333;
    text-align: left;
    background-color: #ece9d8;
}
.flint-tfoot td {
    background-color: #d4d0c8 !important;
    font-weight: bold;
    border-top: 2px solid #808080;
}
.flint-meter {
    width: 100%;
    height: 18px;
    background-color: #ffffff;
    border-top: 2px solid #808080;
    border-left: 2px solid #808080;
    border-right: 2px solid #ffffff;
    border-bottom: 2px solid #ffffff;
    box-sizing: border-box;
    overflow: hidden;
}
.flint-meter-bar { height: 100%; }
.flint-meter-low  { background: linear-gradient(to bottom, #3a9e3a 0%, #1e7a1e 50%, #3a9e3a 100%); }
.flint-meter-medium { background: linear-gradient(to bottom, #cc8800 0%, #aa6600 50%, #cc8800 100%); }
.flint-meter-high { background: linear-gradient(to bottom, #cc3300 0%, #aa1100 50%, #cc3300 100%); }
"#;
