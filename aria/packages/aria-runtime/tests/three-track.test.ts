import { describe, it, expect } from 'vitest';
import {
  isThreeTrackSuccess,
  isThreeTrackPartialSuccess,
  isThreeTrackFailure,
  type ThreeTrack,
} from '../src/three-track.js';

describe('ThreeTrack<T, P, E>', () => {
  it('narrows SUCCESS track correctly', () => {
    const r: ThreeTrack<number, Partial<number>, string> = { _tag: 'SUCCESS', value: 42 };
    expect(isThreeTrackSuccess(r)).toBe(true);
    expect(isThreeTrackPartialSuccess(r)).toBe(false);
    expect(isThreeTrackFailure(r)).toBe(false);
  });

  it('narrows PARTIAL_SUCCESS track correctly', () => {
    const r: ThreeTrack<number, number, string> = {
      _tag: 'PARTIAL_SUCCESS',
      value: 10,
      succeeded_count: 2,
      failed_count: 1,
    };
    expect(isThreeTrackPartialSuccess(r)).toBe(true);
    if (isThreeTrackPartialSuccess(r)) {
      expect(r.succeeded_count).toBe(2);
      expect(r.failed_count).toBe(1);
    }
  });

  it('narrows FAILURE track correctly', () => {
    const r: ThreeTrack<number, number, string> = { _tag: 'FAILURE', error: 'all failed' };
    expect(isThreeTrackFailure(r)).toBe(true);
    if (isThreeTrackFailure(r)) {
      expect(r.error).toBe('all failed');
    }
  });
});
