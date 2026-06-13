import * as vscode from "vscode";

import { activateCompletions } from "./completionProvider";

export function activate(context: vscode.ExtensionContext): void {
  activateCompletions(context);
}

export function deactivate(): void {}
