import { i18nObject } from '$i18n/i18n-util';
import { loadAllLocales } from '$i18n/i18n-util.sync';
import { describe, expect, it } from 'vitest';

import { describeError, frontendError } from './errors';

loadAllLocales();
const LL = i18nObject('en');

describe('describeError', () => {
  it('names what failed and why', () => {
    expect(
      describeError(
        { context: 'receive_credential', cause: 'unreachable', counterparty: 'Mira', detail: 'error sending request' },
        LL,
      ),
    ).toEqual({
      title: "Couldn't add the credential",
      detail: "UniMe couldn't reach Mira. Check your internet connection and try again.",
    });
  });

  it('falls back to a generic name for the other party', () => {
    expect(
      describeError(
        { context: 'receive_credential', cause: 'offer_expired', counterparty: null, detail: 'HTTP 404' },
        LL,
      ).detail,
    ).toBe('This offer has expired or has already been used. Ask the issuer for a new one.');
  });

  it('describes errors raised in the frontend', () => {
    expect(describeError(frontendError('biometrics', new Error('not available')), LL)).toEqual({
      title: "Couldn't change biometric unlock",
      detail: 'Something unexpected happened. Try again, and restart UniMe if it keeps happening.',
    });
  });
});
