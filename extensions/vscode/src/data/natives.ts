// Transcribed from docs/reference/native-functions.md — keep in sync when
// natives are added, removed, or documented differently.

export interface NativeDoc {
  /** Fully qualified native name, e.g. "ui.window". */
  name: string;
  /** Example call shape, e.g. "ncallr dst, ui.window, html, title". */
  signature: string;
  /** Short description of the return value or effect. */
  doc: string;
}

function uiNative(name: string, args: string[], doc?: string): NativeDoc {
  const fullName = `ui.${name}`;
  return {
    name: fullName,
    signature: `ncallr dst, ${fullName}${args.length > 0 ? `, ${args.join(", ")}` : ""}`,
    doc: doc ?? `Returns html with ${fullName} appended.`,
  };
}

const UI_NATIVES: NativeDoc[] = [
  uiNative("window", ["html", "title"], "Returns html with the document shell, stylesheet, title, and page frame opened."),
  uiNative("window_end", ["html"], "Returns html with the window frame closed."),
  uiNative("navbar", ["html"]),
  uiNative("navbar_end", ["html"]),
  uiNative("nav_item", ["html", "label", "href"]),
  uiNative("breadcrumb", ["html"]),
  uiNative("breadcrumb_item", ["html", "label", "href"]),
  uiNative("breadcrumb_end", ["html"]),
  uiNative("card", ["html", "title"]),
  uiNative("card_end", ["html"]),
  uiNative("section", ["html", "title", "subtitle?"]),
  uiNative("section_end", ["html"]),
  uiNative("tabs", ["html"]),
  uiNative("tab", ["html", "label", "id"]),
  uiNative("tabs_body", ["html"]),
  uiNative("tab_panel", ["html", "id"]),
  uiNative("tab_panel_end", ["html"]),
  uiNative("tabs_end", ["html"]),
  uiNative("row", ["html"]),
  uiNative("row_end", ["html"]),
  uiNative("column", ["html"]),
  uiNative("column_end", ["html"]),
  uiNative("title", ["html", "text"]),
  uiNative("text", ["html", "text"]),
  uiNative("field", ["html", "label", "value"]),
  uiNative("badge", ["html", "label"]),
  uiNative("alert", ["html", "kind", "message"]),
  uiNative("status", ["html", "label", "kind"]),
  uiNative("divider", ["html"]),
  uiNative("progress", ["html", "value", "max"]),
  uiNative("code", ["html", "text"]),
  uiNative("link", ["html", "label", "href"]),
  uiNative("table", ["html"]),
  uiNative("table_end", ["html"]),
  uiNative("tr", ["html"]),
  uiNative("tr_end", ["html"]),
  uiNative("th", ["html", "label"]),
  uiNative("td", ["html", "value"]),
  uiNative("button", ["html", "label", "href"]),
  uiNative("form", ["html", "method", "action"]),
  uiNative("form_end", ["html"]),
  uiNative("input", ["html", "label", "name"]),
  uiNative("textarea", ["html", "label", "name"]),
  uiNative("select", ["html", "label", "name"]),
  uiNative("select_end", ["html"]),
  uiNative("option", ["html", "label", "value"]),
  uiNative("checkbox", ["html", "label", "name", "value"]),
  uiNative("radio", ["html", "label", "name", "value"]),
  uiNative("hidden", ["html", "name", "value"]),
  uiNative("submit", ["html", "label"]),
  uiNative("fieldset", ["html", "legend"]),
  uiNative("fieldset_end", ["html"]),
  uiNative("pagination", ["html"]),
  uiNative("page_item", ["html", "label", "href"]),
  uiNative("page_current", ["html", "label"]),
  uiNative("pagination_end", ["html"]),
  uiNative("toolbar", ["html"]),
  uiNative("toolbar_end", ["html"]),
  uiNative("list", ["html"]),
  uiNative("list_item", ["html", "text"]),
  uiNative("list_end", ["html"]),
  uiNative("stat", ["html", "label", "value"]),
  uiNative("image", ["html", "src", "alt"]),
  uiNative("password", ["html", "label", "name"]),
  uiNative("file", ["html", "label", "name"]),
  uiNative("number", ["html", "label", "name"]),
  uiNative("action_bar", ["html"]),
  uiNative("action_bar_end", ["html"]),
  uiNative("empty", ["html", "message"]),
  uiNative("tree", ["html"]),
  uiNative("tree_item", ["html", "label", "href"]),
  uiNative("tree_group", ["html", "label"]),
  uiNative("tree_group_end", ["html"]),
  uiNative("tree_end", ["html"]),
  uiNative("steps", ["html"]),
  uiNative("step", ["html", "label", "active"]),
  uiNative("steps_end", ["html"]),
  uiNative("footer", ["html", "text?"]),
  uiNative("footer_end", ["html"]),
  uiNative("dialog", ["html", "id", "title"]),
  uiNative("dialog_end", ["html"]),
  uiNative("dialog_trigger", ["html", "label", "id"]),
  uiNative("dialog_alert", ["html", "id", "title", "message"]),
  uiNative("dialog_confirm", ["html", "id", "title", "message", "action"]),
  uiNative("dialog_prompt", ["html", "id", "title", "label", "name", "action"]),
  uiNative("layout", ["html"]),
  uiNative("layout_end", ["html"]),
  uiNative("sidebar", ["html"]),
  uiNative("sidebar_end", ["html"]),
  uiNative("main", ["html"]),
  uiNative("main_end", ["html"]),
  uiNative("menu", ["html", "title"]),
  uiNative("menu_item", ["html", "label", "href"]),
  uiNative("menu_active", ["html", "label", "href"]),
  uiNative("menu_end", ["html"]),
  uiNative("accordion", ["html"]),
  uiNative("accordion_item", ["html", "title"]),
  uiNative("accordion_item_end", ["html"]),
  uiNative("accordion_end", ["html"]),
  uiNative("ol", ["html"]),
  uiNative("ol_item", ["html", "text"]),
  uiNative("ol_end", ["html"]),
  uiNative("kbd", ["html", "text"]),
  uiNative("caption", ["html", "text"]),
  uiNative("tfoot", ["html"]),
  uiNative("tfoot_end", ["html"]),
  uiNative("meter", ["html", "value", "max"]),
];

export const NATIVES: NativeDoc[] = [
  // debug.*
  {
    name: "debug.print",
    signature: "ncall debug.print, r0, r1, ...",
    doc: "Prints arguments to stdout, space-separated. Returns no value.",
  },

  // string.*
  {
    name: "string.equals",
    signature: "ncallr dst, string.equals, a, b",
    doc: "1 when strings are equal, else 0.",
  },
  {
    name: "string.contains",
    signature: "ncallr dst, string.contains, s, sub",
    doc: "1 when s contains sub, else 0.",
  },
  {
    name: "string.starts_with",
    signature: "ncallr dst, string.starts_with, s, prefix",
    doc: "1 when s starts with prefix, else 0.",
  },
  {
    name: "string.ends_with",
    signature: "ncallr dst, string.ends_with, s, suffix",
    doc: "1 when s ends with suffix, else 0.",
  },
  {
    name: "string.escape_html",
    signature: "ncallr dst, string.escape_html, s",
    doc: "HTML-escaped text for safe insertion into pages.",
  },
  {
    name: "string.concat",
    signature: "ncallr dst, string.concat, a, b",
    doc: "a followed by b.",
  },
  {
    name: "string.trim",
    signature: "ncallr dst, string.trim, s",
    doc: "s without leading and trailing whitespace.",
  },
  {
    name: "string.to_upper",
    signature: "ncallr dst, string.to_upper, s",
    doc: "Uppercase text.",
  },
  {
    name: "string.to_lower",
    signature: "ncallr dst, string.to_lower, s",
    doc: "Lowercase text.",
  },
  {
    name: "string.replace",
    signature: "ncallr dst, string.replace, s, from, to",
    doc: "Text with all matches replaced.",
  },
  {
    name: "string.slice",
    signature: "ncallr dst, string.slice, s, start, end",
    doc: "Character slice between clamped indices (Unicode scalar values).",
  },
  {
    name: "string.len",
    signature: "ncallr dst, string.len, s",
    doc: "Character count as int.",
  },
  {
    name: "string.split",
    signature: "ncallr dst, string.split, s, sep",
    doc: "JSON array of string parts.",
  },
  {
    name: "string.to_int",
    signature: "ncallr dst, string.to_int, s",
    doc: "Parsed integer after trimming whitespace. Fails if not a valid i64.",
  },
  {
    name: "string.from_int",
    signature: "ncallr dst, string.from_int, n",
    doc: "Integer as string.",
  },
  {
    name: "string.from",
    signature: "ncallr dst, string.from, value",
    doc: "Any VM value as string (display form).",
  },

  // json.*
  {
    name: "json.object",
    signature: "ncallr dst, json.object",
    doc: "Empty object {}.",
  },
  {
    name: "json.array",
    signature: "ncallr dst, json.array",
    doc: "Empty array [].",
  },
  {
    name: "json.null",
    signature: "ncallr dst, json.null",
    doc: "JSON null.",
  },
  {
    name: "json.bool",
    signature: "ncallr dst, json.bool, n",
    doc: "JSON true when n != 0, else false.",
  },
  {
    name: "json.parse",
    signature: "ncallr dst, json.parse, s",
    doc: "Parsed JSON from a string. Fails on invalid JSON.",
  },
  {
    name: "json.get",
    signature: "ncallr dst, json.get, j, key",
    doc: "Object field, non-negative array item, or JSON null. Missing values return null.",
  },
  {
    name: "json.has",
    signature: "ncallr dst, json.has, j, key",
    doc: "1 if object field exists, else 0.",
  },
  {
    name: "json.len",
    signature: "ncallr dst, json.len, j",
    doc: "Length of array, object, or JSON string.",
  },
  {
    name: "json.type",
    signature: "ncallr dst, json.type, j",
    doc: 'JSON type name: null, bool, number, string, array, or object.',
  },
  {
    name: "json.keys",
    signature: "ncallr dst, json.keys, j",
    doc: "JSON array of object keys. Fails unless j is an object.",
  },
  {
    name: "json.set",
    signature: "ncallr dst, json.set, j, key, value",
    doc: "Copy of j with key set. Array keys must be non-negative and implicit expansion is capped.",
  },
  {
    name: "json.push",
    signature: "ncallr dst, json.push, j, value",
    doc: "Copy of an array with value appended. Copy-on-write.",
  },
  {
    name: "json.delete",
    signature: "ncallr dst, json.delete, j, key",
    doc: "Copy of object j without key. Copy-on-write.",
  },
  {
    name: "json.merge",
    signature: "ncallr dst, json.merge, base, patch",
    doc: "Copy of base with fields from object patch. Copy-on-write.",
  },
  {
    name: "json.stringify",
    signature: "ncallr dst, json.stringify, j",
    doc: "Compact JSON string.",
  },
  {
    name: "json.to_int",
    signature: "ncallr dst, json.to_int, j",
    doc: "JSON integer as int. Fails unless j is an integer.",
  },
  {
    name: "json.to_str",
    signature: "ncallr dst, json.to_str, j",
    doc: "JSON string as str. Fails unless j is a string.",
  },
  {
    name: "json.from_int",
    signature: "ncallr dst, json.from_int, n",
    doc: "Integer converted to JSON number.",
  },
  {
    name: "json.from_str",
    signature: "ncallr dst, json.from_str, s",
    doc: "String converted to JSON string.",
  },

  // math.*
  {
    name: "math.abs",
    signature: "ncallr dst, math.abs, n",
    doc: "Absolute value of an int or float. Fails if an int result is outside range.",
  },
  {
    name: "math.min",
    signature: "ncallr dst, math.min, a, b",
    doc: "Smaller numeric value.",
  },
  {
    name: "math.max",
    signature: "ncallr dst, math.max, a, b",
    doc: "Larger numeric value.",
  },
  {
    name: "math.floor",
    signature: "ncallr dst, math.floor, n",
    doc: "Rounded down as int. Fails if the result is not finite or outside int range.",
  },
  {
    name: "math.ceil",
    signature: "ncallr dst, math.ceil, n",
    doc: "Rounded up as int. Fails if the result is not finite or outside int range.",
  },
  {
    name: "math.sqrt",
    signature: "ncallr dst, math.sqrt, n",
    doc: "Square root as float.",
  },
  {
    name: "math.pow",
    signature: "ncallr dst, math.pow, base, exp",
    doc: "base ^ exp as float.",
  },
  {
    name: "math.random",
    signature: "ncallr dst, math.random",
    doc: "Random float in [0.0, 1.0).",
  },
  {
    name: "math.rand_int",
    signature: "ncallr dst, math.rand_int, min, max",
    doc: "Random int in [min, max]. Fails when min > max.",
  },

  // time.*
  {
    name: "time.now",
    signature: "ncallr dst, time.now",
    doc: "Unix timestamp in milliseconds as int.",
  },

  // env.*
  {
    name: "env.get",
    signature: "ncallr dst, env.get, name",
    doc: 'Environment variable value, or "" if missing.',
  },

  // crypto.*
  {
    name: "crypto.uuid",
    signature: "ncallr dst, crypto.uuid",
    doc: "Random UUID v4 string.",
  },

  // ui.*
  ...UI_NATIVES,

  // http.* — request
  {
    name: "http.method",
    signature: "ncallr dst, http.method",
    doc: "HTTP method as str.",
  },
  {
    name: "http.path",
    signature: "ncallr dst, http.path",
    doc: "Request path as str.",
  },
  {
    name: "http.body",
    signature: "ncallr dst, http.body",
    doc: "Raw request body as str.",
  },
  {
    name: "http.param",
    signature: "ncallr dst, http.param, name",
    doc: 'Path parameter as str, or "".',
  },
  {
    name: "http.query",
    signature: "ncallr dst, http.query, name",
    doc: 'Query parameter as str, or "".',
  },
  {
    name: "http.header",
    signature: "ncallr dst, http.header, name",
    doc: 'Header value as lossy UTF-8 str, or "".',
  },
  {
    name: "http.cookie",
    signature: "ncallr dst, http.cookie, name",
    doc: 'Cookie value as str, or "".',
  },
  {
    name: "http.json_body",
    signature: "ncallr dst, http.json_body",
    doc: "Parsed request body as json. Fails if the body is not valid JSON.",
  },
  {
    name: "http.form",
    signature: "ncallr dst, http.form, field",
    doc: 'URL-encoded form field as str, or "".',
  },

  // http.* — response
  {
    name: "http.set_status",
    signature: "ncall http.set_status, code",
    doc: "Set HTTP status code. Fails if the integer is not a valid HTTP status code.",
  },
  {
    name: "http.set_header",
    signature: "ncall http.set_header, name, value",
    doc: "Append a response header.",
  },
  {
    name: "http.set_cookie",
    signature: "ncall http.set_cookie, name, value",
    doc: "Append a simple Set-Cookie header.",
  },
  {
    name: "http.text",
    signature: "ncall http.text, s",
    doc: "Set plain text response body.",
  },
  {
    name: "http.html",
    signature: "ncall http.html, s",
    doc: "Set HTML response body.",
  },
  {
    name: "http.json",
    signature: "ncall http.json, j",
    doc: "Set JSON response body.",
  },
  {
    name: "http.redirect",
    signature: "ncall http.redirect, url",
    doc: "Set status 302 and append location.",
  },
  {
    name: "http.abort",
    signature: "ncall http.abort",
    doc: "Stop the handler and send the current response.",
  },
];
