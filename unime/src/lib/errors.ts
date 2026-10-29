import type { TranslationFunctions } from '$i18n/i18n-types';

import type { ErrorCause } from '@bindings/error/ErrorCause';
import type { ErrorContext } from '@bindings/error/ErrorContext';
import type { UserError } from '@bindings/error/UserError';

/**
 * Builds an error that is raised in the frontend, so that it is shown like an error from the backend.
 */
export function frontendError(context: ErrorContext, detail: unknown, cause: ErrorCause = 'unknown'): UserError {
  return {
    context,
    cause,
    counterparty: null,
    detail: detail instanceof Error ? detail.message : String(detail),
  };
}

function counterpartyName(error: UserError, LL: TranslationFunctions): string {
  if (error.counterparty) return error.counterparty;
  switch (error.context) {
    case 'receive_credential':
    case 'refresh_credential_status':
      return LL.ERROR.COUNTERPARTY.ISSUER();
    case 'share_credential':
    case 'share_public_link':
      return LL.ERROR.COUNTERPARTY.VERIFIER();
    case 'verify_email':
      return LL.ERROR.COUNTERPARTY.EMAIL_SERVICE();
    default:
      return LL.ERROR.COUNTERPARTY.DEFAULT();
  }
}

/**
 * The title (what failed) and the explanation (why, and what to do next) shown to the user.
 */
export function describeError(error: UserError, LL: TranslationFunctions): { title: string; detail: string } {
  const context = error.context.toUpperCase() as Uppercase<ErrorContext>;
  const cause = error.cause.toUpperCase() as Uppercase<ErrorCause>;
  return {
    title: LL.ERROR.CONTEXT[context](),
    detail: LL.ERROR.CAUSE[cause]({ name: counterpartyName(error, LL) }),
  };
}
