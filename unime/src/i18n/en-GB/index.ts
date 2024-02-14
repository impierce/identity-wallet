import { extendDictionary } from 'typesafe-i18n/utils';

import en from '../en';
import type { Translation } from '../i18n-types';

const en_GB = extendDictionary(en, {
  ...(en as Translation),
  ONBOARDING: {
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Customisation',
    },
  },
}) satisfies Translation;

export default en_GB;
