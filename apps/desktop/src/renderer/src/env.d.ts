import type { NexgitApi } from '../../shared/protocol';

export {};

declare global {
  interface Window {
    nexgit: NexgitApi;
  }
}
