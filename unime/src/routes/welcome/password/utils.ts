import { get } from 'svelte/store';

import LL from '$src/i18n/i18n-svelte';
import { warn } from '@tauri-apps/plugin-log';

export const passwordPolicy = [
  {
    name: get(LL).SETTINGS.PASSWORD.POLICY.UPPERCASE_LETTER(),
    regex: /[A-Z]/,
    count: 1,
  },
  {
    name: get(LL).SETTINGS.PASSWORD.POLICY.LOWERCASE_LETTER(),
    regex: /[a-z]/,
    count: 1,
  },
  {
    name: get(LL).SETTINGS.PASSWORD.POLICY.NUMBER(),
    regex: /[0-9]/,
    count: 1,
  },
  {
    name: get(LL).SETTINGS.PASSWORD.POLICY.CHARACTERS(),
    regex: /.{8,}/,
    count: 8,
  },
];

export const checkPasswordPolicy = (password: string) => {
  const violations: string[] = [];
  passwordPolicy.forEach((rule) => {
    if (!password.match(rule.regex)) {
      violations.push(rule.name);
    }
  });
  if (violations.length !== 0) {
    warn(`Password policy violations: ${JSON.stringify(violations)}`);
  }
  return violations;
};
