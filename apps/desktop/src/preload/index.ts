import { contextBridge, ipcRenderer } from 'electron';

import type { AppInfo, NexgitApi, RepoStatus, StackSummary } from '../shared/protocol';

const api: NexgitApi = {
  request<T = unknown>(method: string, params?: unknown): Promise<T> {
    return ipcRenderer.invoke('nexgit:request', method, params) as Promise<T>;
  },

  getVersion(): Promise<AppInfo> {
    return ipcRenderer.invoke('nexgit:request', 'system.version', {}) as Promise<AppInfo>;
  },

  getRepoStatus(): Promise<RepoStatus> {
    return ipcRenderer.invoke('nexgit:request', 'repo.status', {}) as Promise<RepoStatus>;
  },

  listStacks(): Promise<StackSummary[]> {
    return ipcRenderer.invoke('nexgit:request', 'stack.list', {}) as Promise<StackSummary[]>;
  }
};

contextBridge.exposeInMainWorld('nexgit', api);
