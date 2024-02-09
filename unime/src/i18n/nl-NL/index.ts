import type { Translation } from '../i18n-types';

const nl = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Welkom bij',
      WHAT_IS_UNIME_1: 'UniMe verbindt je digitale wereld, veilig en beschermd.',
      WHAT_IS_UNIME_2: 'Maak een gloednieuw profiel aan om te beginnen.',
      CREATE_NEW_PROFILE: 'Maak nieuw profiel aan',
      SELECT_LANGUAGE: 'Selecteer taal',
    },
    PLEDGE: {
      NAVBAR_TITLE: 'UniMe Belofte',
      TITLE_1: 'Geen rare',
      TITLE_2: 'toestanden',
      SUBTITLE: 'Hier is onze belofte aan jou',
      ITEM_1: {
        TITLE: 'Wij delen jouw gegevens niet',
        DESCRIPTION:
          'Jouw gegevens zijn van jou en alleen jij bepaalt met wie je ze deelt. Punt. Sterker nog, jouw gegevens komen nooit in aanraking met een van onze systemen — tenzij je kiest voor een van de cloudopslag opties.',
      },
      ITEM_2: {
        TITLE: 'We zullen geen trackers toevoegen',
        DESCRIPTION:
          'We volgen je acties niet achter de schermen. Punt. Niet om te testen of om welke reden dan ook. Dat beloven we. We verzamelen ook geen anonieme apparaatinformatie or gebruiksstatistieken. Deze beslissing maakt de ontwikkeling van de app wat moeilijker voor ons, maar we geloven dat het de juiste beslissing is.',
      },
      ITEM_3: {
        TITLE: 'Je bezit jouw informatie',
        DESCRIPTION: 'Wij geloven dat het tijd wordt dat jij weer de baas wordt van je eigen persoonlijke informatie.',
      },
    },
    TERMS: {
      NAVBAR_TITLE: 'Algemene Voorwaarden',
      TITLE_1: 'Hier zijn de minder interessante',
      TITLE_2: 'dingen',
      SUBTITLE: 'Ja, we weten het. Wij raden het toch aan deze informatie aandachtig te lezen.',
      OWNERSHIP: {
        TITLE: 'Eigendom',
        DESCRIPTION: 'Ik begrijp dat ik volledig verantwoordelijk ben voor mijn gegevens',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Personalisatie',
      NAME: {
        TITLE_1: 'Laten we beginnen! Kies een',
        TITLE_2: 'profielnaam',
        SUBTITLE: 'Jouw profielinformatie verlaat nooit jouw apparaat',
        INPUT_PLACEHOLDER: 'Profielnaam invoeren',
      },
      THEME: {
        TITLE_1: 'Kies het uiterlijk',
        TITLE_2: 'van je app',
        SUBTITLE: 'Ben jij meer een nachtuil?',
      },
      PICTURE: {
        TITLE_1: 'Stel een',
        TITLE_2: 'profielfoto in',
        SUBTITLE: 'Maak het je eigen.',
      },
      SKIP: {
        TITLE: 'Sla personalisatie over',
        TEXT: 'Weet je het zeker? Je kunt het uiterlijk van de app later aanpassen in de instellingen.',
        CONFIRM: 'Ja',
        ABORT: 'Nee, ga door',
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: 'Wachtwoord',
      TITLE_1: 'Stel je nieuwe',
      TITLE_2: 'wachtwoord in',
      SUBTITLE: 'Je moet een sterk wachtwoord kiezen om je gegevens veilig te versleutelen.',
      INPUT_PLACEHOLDER: 'Voer wachtwoord in',
      CONFIRM: {
        NAVBAR_TITLE: 'Wachtwoord Bevestigen',
        TITLE_1: 'Bevestig je nieuwe',
        TITLE_2: 'wachtwoord',
        SUBTITLE: 'Je moet je wachtwoord bevestigen om er zeker van te zijn dat je het correct hebt getypt.',
        INPUT_PLACEHOLDER: 'Typ je wachtwoord opnieuw in',
        MATCH: 'Wachtwoorden komen overeen',
        NO_MATCH: 'Wachtwoorden komen niet overeen',
      },
      COMPLETED: {
        NAVBAR_TITLE: 'Wachtwoord Instellen',
        TITLE_1: 'Je UniMe profiel is nu',
        TITLE_2: 'beschermd',
        MESSAGE_1: 'Veilig & Beveiligd',
        MESSAGE_2: 'Goed Gedaan',
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
      TITLE: 'App opnieuw instellen',
      DESCRIPTION: 'Weet je zeker dat je de app opnieuw wilt instellen en alle gegevens wilt verwijderen?',
      CONFIRM: 'Ja, verwijder alles',
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
        NAVBAR_TITLE: 'Bewerk profielnaam',
        INPUT_PLACEHOLDER: 'Profielnaam invoeren',
        CONFIRM: 'Wijzigingen opslaan',
      },
      DISPLAY_PICTURE: {
        TITLE: 'Profielfoto',
        EDIT: 'Bewerk',
        CHANGE: 'Selecteer een nieuwe afbeelding',
      },
      DELETE_PROFILE: {
        TITLE: 'Verwijder profiel',
      },
    },
    APP: {
      TITLE: 'App instellingen',
      NAVBAR_TITLE: 'App Instellingen',
      LANGUAGE: {
        TITLE: 'Taal',
        NAVBAR_TITLE: 'Selecteer taal',
        COMING_SOON: 'Binnenkort beschikbaar',
      },
      THEME: {
        TITLE: 'Thema',
        NAVBAR_TITLE: 'Selecteer Thema',
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
      TITLE: 'Momenteel is het hier wat rustig',
      SUBTITLE: 'Wat denk je ervan om nieuwe credentials toe te voegen om je digitale "me" te starten?',
    },
    FAVORITES: 'Mijn favorieten',
  },
  ACTIVITY: {
    NAVBAR_TITLE: 'Verbonden',
    TABS: {
      CONNECTIONS: 'Verbindingen',
      TIMELINE: 'Tijdlijn',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Overzicht',
      DATA: 'Data',
      ACTIVITY: 'Activiteit',
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
  BADGE: {
    DETAILS: {
      VALID: 'Geldig',
      ISSUED_BY: 'Uitgegeven door',
      DESCRIPTION: 'Beschrijving',
      METADATA: 'Metadata',
    },
  },
  CANCEL: 'Annuleren',
  CLOSE: 'Sluiten',
  CONTINUE: 'Doorgaan',
  SKIP: 'Overslaan',
} satisfies Translation;

export default nl;
