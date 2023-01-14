// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ErrorInner } from './ErrorInner';

// implement toString
export class ClientError {
  inner: ErrorInner | undefined;
  detail: string | undefined;

  constructor(source: Partial<ClientError>) {
    Object.assign(this, source);
  }

  toString(): string {
    // convert camelCase to space separated words
    let inner = '';
    if (this.inner) {
      inner = this.inner.replace(/([A-Z])/g, ' $1').trim();
    }
    let detail = '';
    if (this.detail) {
      detail = this.detail;
    }
    if (!inner && !detail) return 'Unknown error';
    return `${inner}${detail ? `: ${detail}` : ''}`;
  }
}