import { ipcMain } from 'electron';

import type { CliAppServer } from './cli-server';

const ALLOWED_METHODS = new Set(['system.version', 'repo.status', 'stack.list', 'stacks.list']);

export function registerIpc(appServer: CliAppServer): void {
  ipcMain.handle('nexgit:request', async (_event, method: string, params?: unknown) => {
    if (!ALLOWED_METHODS.has(method)) {
      throw new Error(`Renderer is not allowed to call app-server method: ${method}`);
    }

    return appServer.request(method, params ?? {});
  });
}
