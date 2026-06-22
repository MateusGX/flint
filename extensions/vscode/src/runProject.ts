import * as path from "path";
import * as vscode from "vscode";

const FLINT_MANIFEST = "flint.toml";
const RUN_COMMAND = "flint.runProject";

const runTerminals = new Map<string, vscode.Terminal>();

export function activateRunProject(context: vscode.ExtensionContext): void {
  context.subscriptions.push(
    vscode.commands.registerCommand(RUN_COMMAND, runProject),
    vscode.window.onDidCloseTerminal((terminal) => {
      for (const [root, runningTerminal] of runTerminals) {
        if (runningTerminal === terminal) {
          runTerminals.delete(root);
          return;
        }
      }
    })
  );
}

async function runProject(resource?: vscode.Uri): Promise<void> {
  const projectRoot = await findProjectRoot(resource ?? activeDocumentUri());
  if (!projectRoot) {
    vscode.window.showErrorMessage("Flint: could not find a flint.toml project manifest.");
    return;
  }

  const cliPath = vscode.workspace.getConfiguration("flint").get<string>("cliPath", "flint").trim();
  const command = `${quoteCommand(cliPath || "flint")} run`;
  const { terminal, isNew } = terminalFor(projectRoot);

  terminal.show();
  if (!isNew) {
    terminal.sendText("\u0003", false);
  }
  terminal.sendText(command);
}

function activeDocumentUri(): vscode.Uri | undefined {
  const document = vscode.window.activeTextEditor?.document;
  return document?.uri.scheme === "file" ? document.uri : undefined;
}

async function findProjectRoot(resource?: vscode.Uri): Promise<vscode.Uri | undefined> {
  const fromResource = resource?.scheme === "file" ? await findManifestAbove(resource) : undefined;
  if (fromResource) {
    return fromResource;
  }

  const workspaceFolders = vscode.workspace.workspaceFolders ?? [];
  const manifests = await vscode.workspace.findFiles("**/flint.toml", "{**/.git/**,**/node_modules/**}", 20);

  if (manifests.length === 1) {
    return rootFromManifest(manifests[0]);
  }

  if (manifests.length > 1) {
    return pickProjectRoot(manifests);
  }

  if (workspaceFolders.length === 1 && (await hasManifest(workspaceFolders[0].uri))) {
    return workspaceFolders[0].uri;
  }

  return undefined;
}

async function findManifestAbove(resource: vscode.Uri): Promise<vscode.Uri | undefined> {
  const workspaceFolder = vscode.workspace.getWorkspaceFolder(resource);
  const stopAt = workspaceFolder?.uri.fsPath;
  let current = path.dirname(resource.fsPath);

  while (true) {
    const currentUri = vscode.Uri.file(current);
    if (await hasManifest(currentUri)) {
      return currentUri;
    }

    if (stopAt && current === stopAt) {
      break;
    }

    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }

  return undefined;
}

async function hasManifest(folder: vscode.Uri): Promise<boolean> {
  try {
    await vscode.workspace.fs.stat(vscode.Uri.joinPath(folder, FLINT_MANIFEST));
    return true;
  } catch {
    return false;
  }
}

async function pickProjectRoot(manifests: vscode.Uri[]): Promise<vscode.Uri | undefined> {
  const picks = manifests
    .map(rootFromManifest)
    .sort((a, b) => a.fsPath.localeCompare(b.fsPath))
    .map((root) => ({
      label: path.basename(root.fsPath),
      description: vscode.workspace.asRelativePath(root),
      root,
    }));

  const selected = await vscode.window.showQuickPick(picks, {
    placeHolder: "Select a Flint project to run",
  });
  return selected?.root;
}

function rootFromManifest(manifest: vscode.Uri): vscode.Uri {
  return vscode.Uri.file(path.dirname(manifest.fsPath));
}

function terminalFor(projectRoot: vscode.Uri): { terminal: vscode.Terminal; isNew: boolean } {
  const key = projectRoot.fsPath;
  const existing = runTerminals.get(key);
  if (existing) {
    return { terminal: existing, isNew: false };
  }

  const terminal = vscode.window.createTerminal({
    name: `Flint: ${path.basename(projectRoot.fsPath)}`,
    cwd: projectRoot.fsPath,
  });
  runTerminals.set(key, terminal);
  return { terminal, isNew: true };
}

function quoteCommand(command: string): string {
  if (/^[A-Za-z0-9_./:@%+=,-]+$/.test(command)) {
    return command;
  }
  return `"${command.replace(/"/g, '\\"')}"`;
}
