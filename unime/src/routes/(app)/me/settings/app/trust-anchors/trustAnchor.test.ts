import { isExpired, summary } from './trustAnchor';
import { NOT_AFTER, NOW, trustAnchor } from './trustAnchor.fixture';

describe('trust anchor summary', () => {
  test('names when the certificate expires', () => {
    expect(summary(trustAnchor(), 'en-US', NOW * 1000)).toBe('Valid until Sep 7, 2031');
  });

  test('says when a curated anchor differs from its default', () => {
    expect(summary(trustAnchor({ enabled: true }), 'en-US', NOW * 1000)).toBe(
      'Valid until Sep 7, 2031 · off by default',
    );
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
