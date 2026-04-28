import { useEffect, useState } from 'react';

import type { AppInfo, RepoStatus, StackSummary } from '../../shared/protocol';

type LoadState = {
  version: AppInfo | null;
  repoStatus: RepoStatus | null;
  stacks: StackSummary[];
  error: string | null;
};

export default function App(): JSX.Element {
  const [state, setState] = useState<LoadState>({
    version: null,
    repoStatus: null,
    stacks: [],
    error: null
  });

  useEffect(() => {
    let cancelled = false;

    async function load(): Promise<void> {
      try {
        const [version, repoStatus, stacks] = await Promise.all([
          window.nexgit.getVersion(),
          window.nexgit.getRepoStatus(),
          window.nexgit.listStacks()
        ]);

        if (!cancelled) {
          setState({ version, repoStatus, stacks, error: null });
        }
      } catch (error) {
        if (!cancelled) {
          setState((current) => ({
            ...current,
            error: error instanceof Error ? error.message : String(error)
          }));
        }
      }
    }

    void load();

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main className="app-shell">
      <section className="hero-card">
        <div className="eyebrow">Nexgit desktop scaffold</div>
        <h1>Stacked Git workflows with one Rust engine.</h1>
        <p>
          The desktop app launches the Rust app-server and communicates through a narrow preload
          API. This keeps CLI, TUI, headless commands, and GUI behavior aligned.
        </p>
      </section>

      {state.error ? <section className="error-card">{state.error}</section> : null}

      <section className="grid">
        <article className="panel">
          <h2>App server</h2>
          <dl>
            <dt>Name</dt>
            <dd>{state.version?.name ?? 'loading…'}</dd>
            <dt>Version</dt>
            <dd>{state.version?.version ?? 'loading…'}</dd>
            <dt>Protocol</dt>
            <dd>{state.version?.protocolVersion ?? 'loading…'}</dd>
          </dl>
        </article>

        <article className="panel">
          <h2>Repository</h2>
          <dl>
            <dt>Root</dt>
            <dd>{state.repoStatus?.root ?? 'not selected yet'}</dd>
            <dt>Branch</dt>
            <dd>{state.repoStatus?.currentBranch ?? 'none'}</dd>
            <dt>Status</dt>
            <dd>
              {state.repoStatus ? (state.repoStatus.isClean ? 'clean' : 'dirty') : 'loading…'}
            </dd>
            <dt>Pending changes</dt>
            <dd>{state.repoStatus?.pendingChanges ?? 'loading…'}</dd>
          </dl>
        </article>

        <article className="panel panel-wide">
          <h2>Stacks</h2>
          {state.stacks.length === 0 ? (
            <p className="muted">
              No stacks yet. This is where branch stacks and PR state will render.
            </p>
          ) : (
            <ul>
              {state.stacks.map((stack) => (
                <li key={stack.name}>{stack.name}</li>
              ))}
            </ul>
          )}
        </article>
      </section>
    </main>
  );
}
