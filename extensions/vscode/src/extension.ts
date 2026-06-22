import * as vscode from "vscode";

import { activateCompletions } from "./completionProvider";
import { activateRunProject } from "./runProject";

export function activate(context: vscode.ExtensionContext): void {
  activateCompletions(context);
  activateRunProject(context);
}

export function deactivate(): void {}
