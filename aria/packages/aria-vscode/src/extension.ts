/**
 * ARIA VS Code extension — thin wrapper that starts aria-lsp
 * and connects it to the VS Code language client.
 */

import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
  const config = vscode.workspace.getConfiguration('aria');
  let serverPath: string = config.get('lspServerPath') ?? '';

  if (!serverPath) {
    serverPath = await resolveAriaLspBinary(context);
  }

  if (!serverPath) {
    vscode.window.showErrorMessage(
      'aria-lsp binary not found. Install @aria/build-bin or set aria.lspServerPath.'
    );
    return;
  }

  const serverOptions: ServerOptions = {
    run: { command: serverPath, transport: TransportKind.stdio },
    debug: { command: serverPath, transport: TransportKind.stdio },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', pattern: '**/*.manifest.yaml' },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.manifest.yaml'),
    },
  };

  client = new LanguageClient(
    'aria-lsp',
    'ARIA Manifest Language Server',
    serverOptions,
    clientOptions,
  );

  await client.start();
  context.subscriptions.push(client);
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
  }
}

/**
 * Resolve the aria-lsp binary path by requiring @aria/build-bin,
 * which knows where the platform-specific binary is installed.
 */
async function resolveAriaLspBinary(context: vscode.ExtensionContext): Promise<string> {
  // Try @aria/build-bin postinstall resolver
  try {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const buildBin = require('@aria/build-bin') as { getBinPath(): string };
    return buildBin.getBinPath();
  } catch {
    // Not installed via npm — try PATH
    const { execSync } = require('child_process') as typeof import('child_process');
    try {
      const which = process.platform === 'win32' ? 'where' : 'which';
      const result = execSync(`${which} aria-lsp`, { encoding: 'utf8' }).trim();
      if (result) return result;
    } catch {
      // Not on PATH either
    }
  }
  return '';
}
