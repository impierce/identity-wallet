import extend from 'just-extend';

import type { BaseTranslation } from './i18n-types';

// Issue: "Using `extendDictionary` does not work well with nested namespaces"
// https://github.com/ivanhofer/typesafe-i18n/issues/741

type DeepPartial<T> = T extends BaseTranslation
  ? {
      [P in keyof T]?: DeepPartial<T[P]>;
    }
  : T;
type ToGenericString<T> = T extends string
  ? string
  : T extends BaseTranslation
    ? {
        [P in keyof T]?: ToGenericString<T[P]>;
      }
    : T;

export const initExtendLanguage =
  <TranslationType extends BaseTranslation>() =>
  <Base extends BaseTranslation | TranslationType, Translation extends Base>(
    base: Base,
    part: DeepPartial<ToGenericString<Translation>>,
  ): Translation =>
    extend(true, {}, base, part) as Translation;

export const extendLanguage = initExtendLanguage();
