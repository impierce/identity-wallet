import type { Translation } from '../i18n-types';

const nl = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Welkom bij',
      WHAT_IS_UNIME_1: 'UniMe verbindt je digitale wereld, veilig en beschermd.',
      WHAT_IS_UNIME_2: 'Maak een gloednieuw profiel aan om te beginnen.',
      CREATE_NEW_PROFILE: 'Maak nieuw profiel aan',
    },
    PLEDGE: {
      NAVBAR_TITLE: '',
      TITLE_1: '',
      TITLE_2: '',
      SUBTITLE: '',
      ITEM_1: {
        TITLE: '',
        DESCRIPTION: '',
      },
      ITEM_2: {
        TITLE: '',
        DESCRIPTION: '',
      },
      ITEM_3: {
        TITLE: '',
        DESCRIPTION: '',
      },
    },
    TERMS: {
      NAVBAR_TITLE: '',
      TITLE_1: '',
      TITLE_2: '',
      SUBTITLE: '',
      OWNERSHIP: {
        TITLE: '',
        DESCRIPTION: '',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: '',
      NAME: {
        TITLE_1: '',
        TITLE_2: '',
        SUBTITLE: '',
        INPUT_PLACEHOLDER: '',
      },
      THEME: {
        TITLE_1: '',
        TITLE_2: '',
        SUBTITLE: '',
      },
      PICTURE: {
        TITLE_1: '',
        TITLE_2: '',
        SUBTITLE: '',
      },
      SKIP: {
        TITLE: '',
        TEXT: '',
        CONFIRM: '',
        ABORT: '',
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: '',
      TITLE_1: '',
      TITLE_2: '',
      SUBTITLE: '',
      INPUT_PLACEHOLDER: '',
      CONFIRM: {
        NAVBAR_TITLE: '',
        TITLE_1: '',
        TITLE_2: '',
        SUBTITLE: '',
        INPUT_PLACEHOLDER: '',
        MATCH: '',
        NO_MATCH: '',
      },
      COMPLETED: {
        NAVBAR_TITLE: '',
        TITLE_1: '',
        TITLE_2: '',
        MESSAGE_1: '',
        MESSAGE_2: '',
      },
    },
  },
  SETTINGS: {
    THEME: {
      SYSTEM: '',
      LIGHT: '',
      DARK: '',
    },
    PASSWORD: {
      POLICY: {
        TITLE: '',
        UPPERCASE_LETTER: '',
        LOWERCASE_LETTER: '',
        NUMBER: '',
        CHARACTERS: '',
      },
    },
    RESET_APP: {
      TITLE: '',
      DESCRIPTION: '',
      CONFIRM: '',
      CANCEL: '',
    },
    ACCOUNT: '',
    SUPPORT: '',
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: '',
    BUTTON_TEXT: '',
    FORGOT_PASSWORD: '',
  },
  ME: {
    GREETINGS: {
      GREETING_0: 'Hey',
      GREETING_1: 'Hallo',
      GREETING_2: 'Welkom terug',
      GREETING_3: 'Hoi',
      GREETING_4: 'Dag',
    },
    CREDENTIAL_TABS: {
      ALL: 'Alle',
      DATA: 'Data',
      BADGES: 'Badges',
    },
    EMPTY_CREDENTIALS: {
      TITLE: '',
      SUBTITLE: '',
    },
    FAVORITES: '',
  },
  ACTIVITY: {
    NAVBAR_TITLE: 'Verbonden',
    TABS: {
      CONNECTIONS: 'Verbindingen',
      TIMELINE: 'Cursus',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Overzicht',
      DATA: 'Data',
      ACTIVITY: 'Cursus',
    },
    SUMMARY: {
      TITLE: '',
      FIRST_CONNECTED: '',
      LAST_CONNECTED: '',
    },
    DATA: {
      EMPTY: '',
    },
  },
  TIMELINE: {
    EMPTY: '',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: '',
    NO_QUERY: {
      TITLE: '',
      DESCRIPTION: '',
    },
    NO_RESULTS: {
      TITLE: '',
      DESCRIPTION: '',
    },
  },
  CANCEL: 'Annuleren',
  CONTINUE: 'Doorgaan',
  SKIP: 'Overslaan',
} satisfies Translation;

export default nl;
