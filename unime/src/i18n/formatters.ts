import type { FormattersInitializer } from 'typesafe-i18n';
import type { Locales, Formatters } from './i18n-types';

export const initFormatters: FormattersInitializer<Locales, Formatters> = (locale: Locales) => {
  const formatters: Formatters = {
    // https://github.com/ivanhofer/typesafe-i18n/tree/main/packages/formatters
    capitalize: (value: string) => value.charAt(0).toUpperCase() + value.slice(1)
  };

  return formatters;
};
