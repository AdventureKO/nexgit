import { spawn, type ChildProcessWithoutNullStreams } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';

import type { ClientMessage, ReadyMessage, ServerMessage } from '../shared/protocol';

type SpawnSpec = {
  command: string;
  args: string[];
  cwd?: string;
};

type PendingRequest = {
  resolve(value: unknown): void;
  reject(error: Error): void;
  timeout: NodeJS.Timeout;
};

export class CliAppServer {
  private child: ChildProcessWithoutNullStreams | null = null;
  private stdoutBuffer = '';
  private nextRequestId = 0;
  private pending = new Map<number, PendingRequest>();
  private readyPromise: Promise<ReadyMessage> | null = null;
  private resolveReady: ((message: ReadyMessage) => void) | null = null;
  private rejectReady: ((error: Error) => void) | null = null;

  async start(): Promise<ReadyMessage> {
    if (this.readyPromise) {
      return this.readyPromise;
    }

    const spec = this.resolveSpawnSpec();

    this.readyPromise = new Promise<ReadyMessage>((resolveReady, rejectReady) => {
      this.resolveReady = resolveReady;
      this.rejectReady = rejectReady;

      const timeout = setTimeout(() => {
        rejectReady(new Error('Timed out waiting for nexgit app-server ready message'));
      }, 120_000);

      const originalResolve = this.resolveReady;
      this.resolveReady = (message) => {
        clearTimeout(timeout);
        originalResolve(message);
      };
    });

    this.child = spawn(spec.command, spec.args, {
      cwd: spec.cwd,
      env: {
        ...process.env,
        RUST_LOG: process.env.RUST_LOG ?? 'warn'
      },
      stdio: ['pipe', 'pipe', 'pipe']
    });

    this.child.stdout.on('data', (chunk: Buffer) => this.handleStdout(chunk));
    this.child.stderr.on('data', (chunk: Buffer) => {
      console.error(`[nexgit app-server] ${chunk.toString('utf8').trimEnd()}`);
    });

    this.child.once('error', (error) => {
      this.rejectReady?.(error);
      this.rejectAll(error);
    });

    this.child.once('exit', (code, signal) => {
      const error = new Error(
        `nexgit app-server exited code=${code ?? 'null'} signal=${signal ?? 'null'}`
      );
      this.rejectReady?.(error);
      this.rejectAll(error);
      this.child = null;
      this.readyPromise = null;
    });

    return this.readyPromise;
  }

  async request<T = unknown>(method: string, params: unknown = {}): Promise<T> {
    await this.start();

    if (!this.child?.stdin.writable) {
      throw new Error('nexgit app-server stdin is not writable');
    }

    const id = ++this.nextRequestId;
    const message: ClientMessage = {
      type: 'request',
      id,
      method,
      params
    };

    const response = new Promise<T>((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error(`Timed out waiting for app-server response to ${method}`));
      }, 30_000);

      this.pending.set(id, {
        resolve: (value) => resolve(value as T),
        reject,
        timeout
      });
    });

    this.child.stdin.write(`${JSON.stringify(message)}\n`);
    return response;
  }

  stop(): void {
    for (const request of this.pending.values()) {
      clearTimeout(request.timeout);
      request.reject(new Error('nexgit app-server stopped'));
    }
    this.pending.clear();

    if (this.child && !this.child.killed) {
      this.child.kill();
    }

    this.child = null;
    this.readyPromise = null;
  }

  private resolveSpawnSpec(): SpawnSpec {
    if (process.env.NEXGIT_CLI_PATH) {
      return {
        command: process.env.NEXGIT_CLI_PATH,
        args: ['app-server', '--listen', 'stdio://']
      };
    }

    const devBinary = resolve(process.cwd(), '../../target/debug/nexgit');
    if (existsSync(devBinary)) {
      return {
        command: devBinary,
        args: ['app-server', '--listen', 'stdio://']
      };
    }

    const workspaceRoot = resolve(process.cwd(), '../..');
    const workspaceCargoToml = resolve(workspaceRoot, 'Cargo.toml');
    if (existsSync(workspaceCargoToml)) {
      return {
        command: 'cargo',
        args: ['run', '-p', 'nexgit-cli', '--', 'app-server', '--listen', 'stdio://'],
        cwd: workspaceRoot
      };
    }

    return {
      command: 'nexgit',
      args: ['app-server', '--listen', 'stdio://']
    };
  }

  private handleStdout(chunk: Buffer): void {
    this.stdoutBuffer += chunk.toString('utf8');

    let newlineIndex = this.stdoutBuffer.indexOf('\n');
    while (newlineIndex !== -1) {
      const line = this.stdoutBuffer.slice(0, newlineIndex).trim();
      this.stdoutBuffer = this.stdoutBuffer.slice(newlineIndex + 1);

      if (line.length > 0) {
        this.handleLine(line);
      }

      newlineIndex = this.stdoutBuffer.indexOf('\n');
    }
  }

  private handleLine(line: string): void {
    let message: ServerMessage;

    try {
      message = JSON.parse(line) as ServerMessage;
    } catch (error) {
      console.error('[nexgit app-server] failed to parse line:', line, error);
      return;
    }

    switch (message.type) {
      case 'ready':
        this.resolveReady?.(message);
        break;
      case 'response':
        this.handleResponse(message);
        break;
      case 'event':
        console.debug('[nexgit app-server event]', message.event, message.payload);
        break;
    }
  }

  private handleResponse(message: Extract<ServerMessage, { type: 'response' }>): void {
    const request = this.pending.get(message.id);
    if (!request) {
      return;
    }

    clearTimeout(request.timeout);
    this.pending.delete(message.id);

    if (message.ok) {
      request.resolve(message.result);
    } else {
      request.reject(new Error(`${message.error.code}: ${message.error.message}`));
    }
  }

  private rejectAll(error: Error): void {
    for (const request of this.pending.values()) {
      clearTimeout(request.timeout);
      request.reject(error);
    }
    this.pending.clear();
  }
}
