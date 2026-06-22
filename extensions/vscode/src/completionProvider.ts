import * as vscode from "vscode";
import * as path from "path";

import { INSTRUCTIONS } from "./data/instructions";
import { NATIVES } from "./data/natives";
import { REGISTERS } from "./data/registers";

const FLINT_LANGUAGES: vscode.DocumentSelector = [
  { language: "flint", scheme: "file" },
  { language: "flint-ui", scheme: "file" },
  { language: "flint", scheme: "untitled" },
  { language: "flint-ui", scheme: "untitled" },
];

const FLINT_SECTIONS = [".route", ".text", ".data", ".bss"];
const PAGE_SECTIONS = [...FLINT_SECTIONS, ".render"];
const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
const LABEL_TARGET_INSTRUCTIONS = new Set([
  "call",
  "jmp",
  "je",
  "jne",
  "jl",
  "jg",
  "jle",
  "jge",
]);

const RENDER_SNIPPETS = [
  ["window", 'window "${1:Title}"\n    $0\nend', "Open a styled document window."],
  ["card", 'card "${1:Title}"\n    $0\nend', "Open a card block."],
  ["block", 'block "${1:Title}"\n    $0\nend', "Open a content section."],
  ["row", "row\n    $0\nend", "Open a responsive row."],
  ["col", "col\n    $0\nend", "Open a column."],
  ["text", 'text "${1:Text}"', "Append paragraph text."],
  ["title", 'title "${1:Heading}"', "Append a heading."],
  ["field", 'field "${1:Label}", ${2:r1}', "Append a label/value field."],
  ["btn", 'btn "${1:Label}", "${2:/href}"', "Append a link styled as a button."],
  ["form", 'form "${1|POST,GET,PUT,PATCH,DELETE|}", "${2:/submit}"\n    $0\nend', "Open a form."],
  ["input", 'input "${1:Label}", "${2:name}"', "Append a text input."],
  ["password", 'password "${1:Label}", "${2:name}"', "Append a password input."],
  ["submit", 'submit "${1:Save}"', "Append a submit button."],
  ["end", "end", "Close the current UI block."],
] as const;

const NAMESPACES = new Set(NATIVES.map((native) => native.name.split(".")[0]));
const NATIVE_PREFIX = /([A-Za-z_][A-Za-z0-9_]*)\.$/;
const USE_PATH_PREFIX = /^(\s*@?use\s+")([^"]*)$/;
const LABEL_PATTERN = /^\s*(\.?[A-Za-z_][A-Za-z0-9_.]*):/gm;

type SectionName = ".route" | ".text" | ".data" | ".bss" | ".render" | undefined;

function currentSection(document: vscode.TextDocument, position: vscode.Position): SectionName {
  for (let line = position.line; line >= 0; line -= 1) {
    const match = document.lineAt(line).text.trim().match(/^section\s+(\.[A-Za-z_][A-Za-z0-9_]*)\b/);
    if (match) {
      return match[1] as SectionName;
    }
  }
  return undefined;
}

function isFlintInstructionContext(document: vscode.TextDocument, position: vscode.Position): boolean {
  if (document.languageId === "flint") {
    return currentSection(document, position) !== ".route";
  }
  if (document.languageId === "flint-ui") {
    return currentSection(document, position) === ".text";
  }
  return false;
}

function isRenderContext(document: vscode.TextDocument, position: vscode.Position): boolean {
  return document.languageId === "flint-ui" && currentSection(document, position) === ".render";
}

function nativeCompletions(
  document: vscode.TextDocument,
  position: vscode.Position
): vscode.CompletionItem[] | undefined {
  if (!isFlintInstructionContext(document, position)) {
    return undefined;
  }

  const line = document.lineAt(position.line).text.slice(0, position.character);
  const match = line.match(NATIVE_PREFIX);
  if (!match) {
    return undefined;
  }

  const namespace = match[1];
  if (!NAMESPACES.has(namespace)) {
    return undefined;
  }

  const prefix = `${namespace}.`;
  return NATIVES.filter((native) => native.name.startsWith(prefix)).map((native) => {
    const label = native.name.slice(prefix.length);
    const item = new vscode.CompletionItem(label, vscode.CompletionItemKind.Function);
    item.insertText = label;
    item.detail = native.signature;
    item.documentation = new vscode.MarkdownString(native.doc);
    return item;
  });
}

function instructionAndRegisterCompletions(): vscode.CompletionItem[] {
  const items: vscode.CompletionItem[] = INSTRUCTIONS.map((instr) => {
    const item = new vscode.CompletionItem(instr.name, vscode.CompletionItemKind.Keyword);
    item.insertText = new vscode.SnippetString(instr.snippet);
    item.detail = instr.signature;
    item.documentation = new vscode.MarkdownString(instr.doc);
    return item;
  });

  for (const register of REGISTERS) {
    const item = new vscode.CompletionItem(register, vscode.CompletionItemKind.Variable);
    item.detail = "register";
    items.push(item);
  }

  for (const namespace of [...NAMESPACES].sort()) {
    const item = new vscode.CompletionItem(namespace, vscode.CompletionItemKind.Module);
    item.insertText = `${namespace}.`;
    item.detail = "native namespace";
    items.push(item);
  }

  return items;
}

function sectionCompletions(document: vscode.TextDocument): vscode.CompletionItem[] {
  const sections = document.languageId === "flint-ui" ? PAGE_SECTIONS : FLINT_SECTIONS;
  return sections.map((section) => {
    const item = new vscode.CompletionItem(`section ${section}`, vscode.CompletionItemKind.Keyword);
    item.insertText = new vscode.SnippetString(`section ${section}\n$0`);
    item.detail = "Flint section";
    return item;
  });
}

function routeCompletions(document: vscode.TextDocument): vscode.CompletionItem[] {
  return HTTP_METHODS.map((method) => {
    const item = new vscode.CompletionItem(method, vscode.CompletionItemKind.EnumMember);
    const snippet =
      document.languageId === "flint-ui"
        ? `${method} "\${1:/path}"`
        : `${method} "\${1:/path}" -> \${2:handler}`;
    item.insertText = new vscode.SnippetString(snippet);
    item.detail = "HTTP route entry";
    return item;
  });
}

function renderCompletions(): vscode.CompletionItem[] {
  return RENDER_SNIPPETS.map(([label, snippet, doc]) => {
    const item = new vscode.CompletionItem(label, vscode.CompletionItemKind.Function);
    item.insertText = new vscode.SnippetString(snippet);
    item.detail = "Flint UI render command";
    item.documentation = new vscode.MarkdownString(doc);
    return item;
  });
}

function labelCompletions(document: vscode.TextDocument): vscode.CompletionItem[] {
  const labels = new Set<string>();
  for (const match of document.getText().matchAll(LABEL_PATTERN)) {
    labels.add(match[1]);
  }

  return [...labels].sort().map((label) => {
    const item = new vscode.CompletionItem(label, vscode.CompletionItemKind.Reference);
    item.detail = "label";
    return item;
  });
}

function wantsLabelCompletion(document: vscode.TextDocument, position: vscode.Position): boolean {
  const before = document.lineAt(position.line).text.slice(0, position.character);
  if (/(?:->)\s*\.?[A-Za-z_][A-Za-z0-9_.]*$/.test(before) || /(?:->)\s*$/.test(before)) {
    return true;
  }

  const instruction = before.trimStart().split(/\s+/)[0]?.toLowerCase();
  return LABEL_TARGET_INSTRUCTIONS.has(instruction);
}

function useKeywordCompletion(document: vscode.TextDocument): vscode.CompletionItem {
  const keyword = document.languageId === "flint" ? "use" : "@use";
  const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
  item.insertText = new vscode.SnippetString(`${keyword} "\${1:services/file.fl}"`);
  item.detail = "include shared Flint code";
  return item;
}

function usePathInfo(
  document: vscode.TextDocument,
  position: vscode.Position
): { prefix: string; range: vscode.Range } | undefined {
  const before = document.lineAt(position.line).text.slice(0, position.character);
  const match = before.match(USE_PATH_PREFIX);
  if (!match) {
    return undefined;
  }

  const prefix = match[2];
  const start = position.character - prefix.length;
  return {
    prefix,
    range: new vscode.Range(position.line, start, position.line, position.character),
  };
}

async function usePathCompletions(
  document: vscode.TextDocument,
  prefix: string,
  range: vscode.Range
): Promise<vscode.CompletionItem[]> {
  const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
  const base =
    workspaceFolder?.uri ??
    (document.uri.scheme === "file" ? vscode.Uri.file(path.dirname(document.uri.fsPath)) : undefined);
  if (!base) {
    return [];
  }
  const normalized = prefix.replace(/\\/g, "/");
  const slash = normalized.lastIndexOf("/");
  const dirPrefix = slash >= 0 ? normalized.slice(0, slash + 1) : "";
  const typedName = slash >= 0 ? normalized.slice(slash + 1) : normalized;
  const dirUri = joinPath(base, dirPrefix);

  let entries: [string, vscode.FileType][];
  try {
    entries = await vscode.workspace.fs.readDirectory(dirUri);
  } catch {
    return [];
  }

  return entries
    .filter(([name, type]) => {
      if (name.startsWith(".")) {
        return false;
      }
      if (!name.startsWith(typedName)) {
        return false;
      }
      return type === vscode.FileType.Directory || name.endsWith(".fl");
    })
    .sort(([aName, aType], [bName, bType]) => {
      if (aType !== bType) {
        return aType === vscode.FileType.Directory ? -1 : 1;
      }
      return aName.localeCompare(bName);
    })
    .map(([name, type]) => {
      const isDirectory = type === vscode.FileType.Directory;
      const item = new vscode.CompletionItem(
        `${dirPrefix}${name}${isDirectory ? "/" : ""}`,
        isDirectory ? vscode.CompletionItemKind.Folder : vscode.CompletionItemKind.File
      );
      item.insertText = `${dirPrefix}${name}${isDirectory ? "/" : ""}`;
      item.range = range;
      item.command = isDirectory
        ? { command: "editor.action.triggerSuggest", title: "Suggest" }
        : undefined;
      item.detail = isDirectory ? "folder" : "Flint source";
      return item;
    });
}

function joinPath(base: vscode.Uri, path: string): vscode.Uri {
  const parts = path.split("/").filter(Boolean);
  return parts.length === 0 ? base : vscode.Uri.joinPath(base, ...parts);
}

class FlintCompletionItemProvider implements vscode.CompletionItemProvider {
  async provideCompletionItems(
    document: vscode.TextDocument,
    position: vscode.Position
  ): Promise<vscode.CompletionItem[]> {
    const pathInfo = usePathInfo(document, position);
    if (pathInfo) {
      return usePathCompletions(document, pathInfo.prefix, pathInfo.range);
    }

    const nativeItems = nativeCompletions(document, position);
    if (nativeItems) {
      return nativeItems;
    }

    if (wantsLabelCompletion(document, position)) {
      return labelCompletions(document);
    }

    const section = currentSection(document, position);
    const items: vscode.CompletionItem[] = [];

    items.push(useKeywordCompletion(document));

    if (document.languageId === "flint" || document.languageId === "flint-ui") {
      items.push(...sectionCompletions(document));
    }

    if (section === ".route") {
      items.push(...routeCompletions(document));
    }

    if (isRenderContext(document, position)) {
      items.push(...renderCompletions(), ...labelCompletions(document));
    }

    if (isFlintInstructionContext(document, position)) {
      items.push(...instructionAndRegisterCompletions());
    }

    return items;
  }
}

export function activateCompletions(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.languages.registerCompletionItemProvider(
      FLINT_LANGUAGES,
      new FlintCompletionItemProvider(),
      ".",
      '"',
      "/",
      "@",
      " "
    )
  );
}
