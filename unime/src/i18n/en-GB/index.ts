import { extendDictionary } from 'typesafe-i18n/utils';

import en from '../en';
import type { Translation } from '../i18n-types';

const en_GB = extendDictionary(en, {
  ...(en as Translation),
  ONBOARDING: {
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Customisation',
    },
    PLEDGE: {
      ITEM_1: {
        DESCRIPTION:
          'Your data belongs to you and only you decide who you share it with. Full stop. In fact, your data never even touches any of our systems - unless you opt-in to one of the cloud storage options.',
      },
      ITEM_2: {
        DESCRIPTION:
          "We do not track your actions behind the scenes. Full stop. Not for testing or any other reasons. That's our pledge. We also do not collect any anonymous device information or usage statistics. That decision makes developing the app a bit harder for us, but we believe it is the right decision.",
      },
    },
    SKIP: {
        TITLE: 'Skip customisation',
      },
    SETTINGS: {
        FAVORITES: 'My favourites',
      },
  },
}) satisfies Translation;

export default en_GB;
