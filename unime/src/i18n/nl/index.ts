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
      SYSTEM: 'Systeem',
      LIGHT: 'Licht',
      DARK: 'Donker',
    },
    PASSWORD: {
      POLICY: {
        TITLE: 'Wachtwoord',
        UPPERCASE_LETTER: 'Hoofdletters',
        LOWERCASE_LETTER: 'Kleine letters',
        NUMBER: 'Nummers',
        CHARACTERS: 'Karakters',
      },
    },
    RESET_APP: {
      TITLE: 'Opnieuw instellen app',
      DESCRIPTION: 'Weet je zeker dat je de app wilt resetten en alle gegevens wilt verwijderen? ',
      CONFIRM: 'Ja, reset alles',
      CANCEL: 'Nee, bewaar mijn profiel',
    },
    ACCOUNT: 'Profiel',
    SUPPORT: {
      TITLE: 'Support',
      ABOUT: {
        TITLE: 'Over UniMe',
      },
      FEEDBACK: {
        TITLE: 'Stuur feedback',
      },
    },
    NAVBAR_TITLE: 'Instellingen',
    PROFILE: {
      TITLE: 'Mijn profiel',
      PROFILE_NAME: {
        TITLE: 'Profielnaam',
        NAVBAR_TITLE: 'Bewerk profielfoto',
        INPUT_PLACEHOLDER: 'Profielnaam invoeren',
        CONFIRM: 'Wijzigingen opslaan',
      },
      DISPLAY_PICTURE: {
        TITLE: 'Toon afbeelding',
        EDIT: 'Bewerk',
        CHANGE: 'Opslaan',
      },
      DELETE_PROFILE: {
        TITLE: 'Verwijder profiel',
      },
    },
    APP: {
      TITLE: 'App instellingen',
      NAVBAR_TITLE: 'App instellingen',
      LANGUAGE: {
        TITLE: 'Taal',
      },
      THEME: {
        TITLE: 'Thema',
        NAVBAR_TITLE: 'Selecteer thema',
      },
      PASSWORD: {
        TITLE: 'Wachtwoord',
      },
      ONBOARDING_JOURNEY: {
        TITLE: 'Onboarding journey',
        BUTTON_TEXT: 'Herstart',
      },
      HINTS_AND_TIPS: {
        TITLE: 'Hints en tips',
        BUTTON_TEXT: 'Opnieuw instellen',
      },
      DEVELOPER_MODE: {
        TITLE: 'Ontwikkelaarsmodus',
      },
    },
    BACKUP_RECOVERY: {
      TITLE: 'Back-up en herstel',
    },
    LOG_OUT: {
      TITLE: 'Log uit',
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Voer je wachtwoord in',
    BUTTON_TEXT: 'Ontgrendel wallet',
    FORGOT_PASSWORD: 'Wachtwoord vergeten?',
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
      TITLE: 'Geen credentials gevonden',
      SUBTITLE: 'Voeg credentials toe om je digitale "me" te starten.',
    },
    FAVORITES: 'Mijn favorieten',
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
      TITLE: 'Verbonden met',
      FIRST_CONNECTED: 'Eerst verbonden',
      LAST_CONNECTED: 'Laatst verbonden',
    },
    DATA: {
      EMPTY: 'Nog geen data.',
    },
  },
  TIMELINE: {
    EMPTY: 'Nog geen activiteit.',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Zoeken',
    NO_QUERY: {
      TITLE: 'Geef een zoekopdracht op.',
      DESCRIPTION: 'Zoek hier naar je credentials en badges.',
    },
    NO_RESULTS: {
      TITLE: 'Geen resultaten gevonden',
      DESCRIPTION: 'Probeer iets anders te zoeken.',
    },
  },
  CANCEL: 'Annuleren',
  CLOSE: 'Sluiten',
  CONTINUE: 'Doorgaan',
  SKIP: 'Overslaan',
} satisfies Translation;

export default nl;
