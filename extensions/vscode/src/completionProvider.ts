import * as vscode from "vscode";

import { NATIVES } from "./data/natives";
import { INSTRUCTIONS } from "./data/instructions";
import { REGISTERS } from "./data/registers";

const NAMESPACES = new Set(
  NATIVES.map((native) => native.name.split(".")[0])
);

const FLINT_LANGUAGES = ["flint", "flint-html", "flint-ui"];

const NATIVE_PREFIX = /([A-Za-z_][A-Za-z0-9_]*)\.$/;

/** Whether `position` falls inside an `<% ... %>` / `<%= ... %>` block. */
function isInsideCodeBlock(
  document: vscode.TextDocument,
  position: vscode.Position
): boolean {
  if (document.languageId === "flint") {
    return true;
  }
  const range = new vscode.Range(new vscode.Position(0, 0), position);
  const text = document.getText(range);
  const lastOpen = text.lastIndexOf("<%");
  const lastClose = text.lastIndexOf("%>");
  return lastOpen > lastClose;
}

function nativeCompletions(
  document: vscode.TextDocument,
  position: vscode.Position
): vscode.CompletionItem[] | undefined {
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
  return NATIVES.filter((native) => native.name.startsWith(prefix)).map(
    (native) => {
      const label = native.name.slice(prefix.length);
      const item = new vscode.CompletionItem(
        label,
        vscode.CompletionItemKind.Function
      );
      item.insertText = label;
      item.detail = native.signature;
      item.documentation = new vscode.MarkdownString(native.doc);
      return item;
    }
  );
}

function instructionAndRegisterCompletions(): vscode.CompletionItem[] {
  const items: vscode.CompletionItem[] = INSTRUCTIONS.map((instr) => {
    const item = new vscode.CompletionItem(
      instr.name,
      vscode.CompletionItemKind.Keyword
    );
    item.insertText = new vscode.SnippetString(instr.snippet);
    item.detail = instr.signature;
    item.documentation = new vscode.MarkdownString(instr.doc);
    return item;
  });

  for (const register of REGISTERS) {
    const item = new vscode.CompletionItem(
      register,
      vscode.CompletionItemKind.Variable
    );
    item.detail = "register";
    items.push(item);
  }

  return items;
}

class FlintCompletionItemProvider implements vscode.CompletionItemProvider {
  provideCompletionItems(
    document: vscode.TextDocument,
    position: vscode.Position
  ): vscode.CompletionItem[] {
    const natives = nativeCompletions(document, position);
    if (natives) {
      return natives;
    }
    if (!isInsideCodeBlock(document, position)) {
      return [];
    }
    return instructionAndRegisterCompletions();
  }
}

export function activateCompletions(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.languages.registerCompletionItemProvider(
      FLINT_LANGUAGES,
      new FlintCompletionItemProvider(),
      "."
    )
  );
}
