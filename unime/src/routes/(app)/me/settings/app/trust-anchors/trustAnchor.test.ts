import { isExpired, listSummary, summary } from './trustAnchor';
import { NOT_AFTER, NOW, trustAnchor } from './trustAnchor.fixture';

describe('trust anchor summary', () => {
  test('names when the certificate expires', () => {
    expect(summary(trustAnchor(), 'en-US', NOW * 1000)).toBe('Valid until Sep 7, 2031');
  });

  test('does not mention whether a curated anchor differs from its default', () => {
    expect(summary(trustAnchor({ enabled: true }), 'en-US', NOW * 1000)).toBe('Valid until Sep 7, 2031');
  });

  test('does not mention a default for an anchor the user added', () => {
    const anchor = trustAnchor({ source: { type: 'added', url: 'https://ca.example/root.crt' }, enabled: true });

    expect(summary(anchor, 'en-US', NOW * 1000)).toBe('Valid until Sep 7, 2031');
  });

  test('marks a certificate outside its validity period', () => {
    const later = (NOT_AFTER + 1) * 1000;

    expect(isExpired(trustAnchor(), later)).toBe(true);
    expect(summary(trustAnchor(), 'en-US', later)).toBe('Expired or not yet valid');
  });
});

describe('trust anchor list summary', () => {
  test('counts the anchors and how many are enabled', () => {
    const on = trustAnchor({ enabled: true });
    const off = trustAnchor({ enabled: false });

    expect(listSummary([])).toBe('Empty list');
    expect(listSummary([on])).toBe('1 anchor, enabled');
    expect(listSummary([off])).toBe('1 anchor, disabled');
    expect(listSummary([on, on])).toBe('2 anchors, all enabled');
    expect(listSummary([off, off])).toBe('2 anchors, none enabled');
    expect(listSummary([on, off, off])).toBe('1 of 3 anchors enabled');
  });
});
