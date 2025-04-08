import { extendLanguage } from '$i18n/i18n-extend-language';

import en from '../en';
import type { Translation } from '../i18n-types';

const en_GB = extendLanguage(en, {
ONBOARDING: {
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Customisation',
    },
    PLEDGE: {
      TITLE_1: 'No dodgy',
      TITLE_2: 'dealings',
      ITEM_1: {
        DESCRIPTION:
          'Your data belongs to you and only you decide who you share it with. Full stop. In fact, your data never even touches any of our systems - unless you opt-in to one of the cloud storage options.',
      },
      ITEM_2: {
        DESCRIPTION:
          "We do not track your actions behind the scenes. Full stop. Not for testing or any other reasons. That's our pledge. We also do not collect any anonymous device information or usage statistics. That decision makes developing the app a bit harder for us, but we believe it is the right decision.",
      },
    }, 
  },
  SCAN:{
    CONNECTION_REQUEST:{
      DESCRIPTION: 'Only accept new connections that you recognise and trust'
    }
  },
  ME: {
    FAVORITES: 'My favourites',
  },
ERROR: {
  TITLE: 'Oh dear!',
}
}) satisfies Translation;

export default en_GB;
