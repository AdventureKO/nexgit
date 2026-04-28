export type RequestId = number;

export type ClientMessage = {
  type: 'request';
  id: RequestId;
  method: string;
  params?: unknown;
};

export type ReadyMessage = {
  type: 'ready';
  serverName: string;
  version: string;
  protocolVersion: number;
  transport: string;
  endpoint?: string;
};

export type ResponseMessage =
  | {
      type: 'response';
      id: RequestId;
      ok: true;
      result?: unknown;
    }
  | {
      type: 'response';
      id: RequestId;
      ok: false;
      error: ProtocolError;
    };

export type EventMessage = {
  type: 'event';
  event: string;
  payload?: unknown;
};

export type ServerMessage = ReadyMessage | ResponseMessage | EventMessage;

export type ProtocolError = {
  code: string;
  message: string;
};

export type AppInfo = {
  name: string;
  version: string;
  protocolVersion: number;
};

export type RepoStatus = {
  root: string | null;
  currentBranch: string | null;
  isClean: boolean;
  pendingChanges: number;
};

export type StackSummary = {
  name: string;
  branches: string[];
  baseBranch: string | null;
  pullRequests: PullRequestSummary[];
};

export type PullRequestSummary = {
  number: number;
  title: string;
  url: string;
  branch: string;
  status: 'draft' | 'open' | 'merged' | 'closed';
};

export type NexgitApi = {
  request<T = unknown>(method: string, params?: unknown): Promise<T>;
  getVersion(): Promise<AppInfo>;
  getRepoStatus(): Promise<RepoStatus>;
  listStacks(): Promise<StackSummary[]>;
};
