import type { Translation } from '../i18n-types';

const de = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Willkommen bei',
      WHAT_IS_UNIME_1: 'UniMe verbindet deine digitale Welt, sicher und geschützt.',
      WHAT_IS_UNIME_2: 'Erstelle ein brandneues Profil, um loszulegen.',
      CREATE_NEW_PROFILE: 'Neues Profil erstellen',
      SELECT_LANGUAGE: 'Sprache auswählen',
    },
    PLEDGE: {
      NAVBAR_TITLE: 'UniMes Versprechen',
      TITLE_1: 'Keine halben',
      TITLE_2: 'Sachen',
      SUBTITLE: 'Hier ist unser Versprechen an dich.',
      ITEM_1: {
        TITLE: 'Wir teilen deine Daten nicht',
        DESCRIPTION:
          'Deine Daten gehören dir und nur du entscheidest, mit wem du sie teilst. Punkt. Tatsächlich landen deine Daten nie auf einem unserer Systeme - es sei denn, du entscheidest dich für eine der Cloud-Speicheroptionen.',
      },
      ITEM_2: {
        TITLE: 'Wir nutzen keine Tracker',
        DESCRIPTION:
          'Wir verfolgen deine Aktionen nicht im Hintergrund. Punkt. Nicht zum Testen oder aus anderen Gründen. Das ist unser Versprechen. Wir sammeln auch keine anonymen Geräteinformationen oder Nutzungsstatistiken. Diese Entscheidung macht die Entwicklung der App für uns etwas schwieriger, aber wir glauben, dass es die richtige Entscheidung ist.',
      },
      ITEM_3: {
        TITLE: 'Du besitzt deine Informationen',
        DESCRIPTION:
          'Wir glauben, dass es an der Zeit ist, dass du wieder der Besitzer deiner eigenen persönlichen Informationen wirst.',
      },
    },
    TERMS: {
      NAVBAR_TITLE: 'Nutzungsbedingungen',
      TITLE_1: 'Hier sind die weniger interessanten',
      TITLE_2: 'Dinge',
      SUBTITLE: 'Ja, das muss leider sein. Wir empfehlen dir trotzdem, diese Informationen sorgfältig zu lesen.',
      OWNERSHIP: {
        TITLE: 'Verantwortung',
        DESCRIPTION: 'Ich verstehe, dass ich allein für meine Daten verantwortlich bin.',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Personalisierung',
      NAME: {
        TITLE_1: 'Los gehts! Wähle einen',
        TITLE_2: 'Profilnamen',
        SUBTITLE: 'Deine Profilinformationen verlassen niemals dein Gerät.',
        INPUT_PLACEHOLDER: 'Gib einen Profilnamen ein',
      },
      PICTURE: {
        TITLE_1: 'Wähle ein',
        TITLE_2: 'Profilbild',
        SUBTITLE: 'Mach es zu deinem.',
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: 'Passwort',
      TITLE_1: 'Wähle dein neues',
      TITLE_2: 'Passwort',
      SUBTITLE: 'Du musst ein starkes Passwort wählen, um deine Daten sicher zu verschlüsseln.',
      INPUT_PLACEHOLDER: 'Gib ein Passwort ein',
      CONFIRM: {
        NAVBAR_TITLE: 'Passwort bestätigen',
        TITLE_1: 'Bitte bestätige dein neues',
        TITLE_2: 'Passwort',
        SUBTITLE: 'Du musst dein Passwort bestätigen, um sicherzustellen, dass du es richtig eingegeben hast.',
        INPUT_PLACEHOLDER: 'Gib dein Passwort erneut ein',
        MATCH: 'Passwörter stimmen überein',
        NO_MATCH: 'Passwörter stimmen nicht überein',
      },
      COMPLETED: {
        NAVBAR_TITLE: 'Passwort gesetzt',
        TITLE_1: 'Dein UniMe Profil ist jetzt',
        TITLE_2: 'geschützt',
        MESSAGE_1: 'Sicher & geschützt.',
        MESSAGE_2: 'Gut gemacht',
      },
    },
  },
  SETTINGS: {
    NAVBAR_TITLE: 'Einstellungen',
    PROFILE: {
      TITLE: 'Mein Profil',
      PROFILE_NAME: {
        TITLE: 'Profilname',
        NAVBAR_TITLE: 'Profilname ändern',
        INPUT_PLACEHOLDER: 'Gib einen Profilnamen ein',
        CONFIRM: 'Aktualisieren',
      },
      DISPLAY_PICTURE: {
        EDIT: 'Ändern',
        CHANGE: 'Wähle ein Profilbild',
        REMOVE: 'Entfernen',
      },
      DELETE_PROFILE: {
        TITLE: 'Profil löschen',
      },
    },
    APP: {
      TITLE: 'App-Einstellungen',
      NAVBAR_TITLE: 'App-Einstellungen',
      LANGUAGE: {
        TITLE: 'Sprache',
        NAVBAR_TITLE: 'Sprache auswählen',
        COMING_SOON: 'Bald verfügbar',
      },
      THEME: {
        LABEL: 'Erscheinungsbild',
        NAVBAR_TITLE: 'Erscheinungsbild anpassen',
        TITLE_1: 'Wähle das',
        TITLE_2: 'Erscheinungsbild',
        SUBTITLE: 'Bist du eher eine Nachteule?',
      },
      PASSWORD: {
        TITLE: 'Passwort',
      },
      ONBOARDING_JOURNEY: {
        TITLE: 'Einführung',
        BUTTON_TEXT: 'Erneut starten',
      },
      HINTS_AND_TIPS: {
        TITLE: 'Hinweise und Tipps',
        BUTTON_TEXT: 'Zurücksetzen',
      },
      DEVELOPER_MODE: {
        TITLE: 'Entwicklermodus',
      },
    },
    BACKUP_RECOVERY: {
      TITLE: 'Sicherung und Wiederherstellung',
    },
    LOG_OUT: {
      TITLE: 'Ausloggen',
    },
    THEME: {
      SYSTEM: 'System',
      LIGHT: 'Hell',
      DARK: 'Dunkel',
    },
    PASSWORD: {
      POLICY: {
        TITLE: 'Dein Passwort muss enthalten',
        UPPERCASE_LETTER: 'Großbuchstaben',
        LOWERCASE_LETTER: 'Kleinbuchstaben',
        NUMBER: 'Zahl',
        CHARACTERS: 'Zeichen',
      },
    },
    RESET_APP: {
      TITLE: 'App zurücksetzen',
      DESCRIPTION: 'Bist du sicher, dass du die App zurücksetzen möchtest?',
      CONFIRM: 'Ja, alle Daten löschen',
      CANCEL: 'Nein, mein Profil behalten',
    },
    ACCOUNT: 'Account',
    SUPPORT: {
      TITLE: 'Support',
      ABOUT: {
        TITLE: 'Über UniMe',
        NAVBAR_TITLE: 'Über UniMe',
        BUILT_WITH: 'Gebaut mit Tauri',
      },
      FEEDBACK: {
        TITLE: 'Feedback senden',
      },
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Passwort eingeben',
    BUTTON_TEXT: 'Wallet entsperren',
    FORGOT_PASSWORD: 'Passwort vergessen?',
  },
  ME: {
    BOTTOM_NAVIGATION_TITLE: 'Me',
    GREETINGS: {
      GREETING_0: 'Hey',
      GREETING_1: "Wie geht's",
      GREETING_2: 'Willkommen zurück',
      GREETING_3: 'Hallo',
      GREETING_4: 'Hi',
    },
    DEMO: 'Besuche eine der folgenden Webseiten auf einem Desktop-Computer, um loszulegen.',
    CREDENTIAL_TABS: {
      ALL: 'Alle',
      DATA: 'Daten',
      BADGES: 'Badges',
    },
    EMPTY_CREDENTIALS: {
      TITLE: 'Hier ist es noch etwas leer',
      SUBTITLE: 'Warum fügst du nicht ein paar Daten zu deiner neuen digitalen Identität hinzu?',
    },
    FAVORITES: 'Meine Favoriten',
  },
  ACTIVITY: {
    BOTTOM_NAVIGATION_TITLE: 'Aktivität',
    NAVBAR_TITLE: 'Verbunden',
    TABS: {
      CONNECTIONS: 'Verbindungen',
      HISTORY: 'Verlauf',
    },
  },
  SCAN: {
    BOTTOM_NAVIGATION_TITLE: 'Scan',
    TITLE_1: 'Scanne einen',
    TITLE_2: 'QR-Code',
    SUBTITLE: 'Bringe einen QR-Code in das Sichtfeld, um eine Interaktion zu starten.',
    PERMISSION_DENIED: 'Keine Berechtigung zum Zugriff auf die Kamera',
    OPEN_SETTINGS: 'Einstellungen öffnen',
    CREDENTIAL_OFFER: {
      NAVBAR_TITLE: 'Credential-Angebot',
      DESCRIPTION: 'bietet dir die folgenden Credentials an',
      ACCEPT: 'Credentials annehmen',
    },
    CONNECTION_REQUEST: {
      NAVBAR_TITLE: 'Verbindungsanfrage',
      TITLE: 'Neue Verbindung',
      DESCRIPTION: 'Akzeptiere nur Verbindungen, die du erwartest und denen du vertraust.',
      CONNECTED_PREVIOUSLY: 'Zuvor verbunden',
      ACCEPT: 'Verbindung akzeptieren',
    },
    SHARE_CREDENTIALS: {
      NAVBAR_TITLE: 'Daten teilen',
      DESCRIPTION: 'fragt nach den folgenden Daten',
      REQUESTED: 'Angefragt',
      APPROVE: 'Anfrage genehmigen',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Übersicht',
      DATA: 'Daten',
      ACTIVITY: 'Verlauf',
    },
    SUMMARY: {
      TITLE: 'Verbunden mit',
      FIRST_CONNECTED: 'Erstmalig verbunden',
      LAST_CONNECTED: 'Zuletzt verbunden',
      EMPTY: 'Noch keine Verbindungen.',
    },
    DATA: {
      EMPTY: 'Noch keine Daten.',
    },
  },
  HISTORY: {
    EMPTY: 'Noch keine Aktivitäten.',
    DATA_RECEIVED: 'Daten empfangen von',
    DATA_SHARED: 'Daten geteilt mit',
    CONNECTION_ADDED: 'Verbunden mit',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Gib einen Suchbegriff ein',
    NO_QUERY: {
      TITLE: 'Wonach suchst du?',
      DESCRIPTION: 'Durchsuche deine Credentials und Badges.',
    },
    NO_RESULTS: {
      TITLE: 'Keine Ergebnisse',
      DESCRIPTION: 'Versuche es mit einem anderen Suchbegriff.',
    },
    RECENT_SEARCHES: 'Zuletzt gesucht',
  },
  CREDENTIAL: {
    NAVBAR_TITLE: 'Credential Informationen',
  },
  BADGE: {
    NAVBAR_TITLE: 'Badge Informationen',
    DETAILS: {
      VALID: 'Gültig',
      ISSUED_BY: 'Ausgestellt von',
      DESCRIPTION: 'Beschreibung',
      CONTENTS: 'Inhalt',
    },
  },
  SORT: {
    TITLE: 'Sortieren',
    PREFERENCES: {
      LIST_VIEW: 'Listenansicht',
      GRID_VIEW: 'Rasteransicht',
      ALPHABETICAL: 'Alphabetisch',
      DATE_ISSUED: 'Ausgabedatum',
      DATE_ADDED: 'Datum hinzugefügt',
    },
    ORDER: {
      A_Z: 'A bis Z',
      Z_A: 'Z bis A',
      NEWEST: 'Neueste zuerst',
      OLDEST: 'Älteste zuerst',
    },
  },
  DOMAIN_LINKAGE: {
    TITLE: 'Verifiziert',
    SUCCESS: 'UniMe konnte die Identität erfolgreich verifizieren, um dir einen sicheren Login zu ermöglichen.',
    FAILURE: 'UniMe konnte die Verknüpfung der Identität mit der Domain nicht überprüfen.',
    UNKNOWN: 'UniMe konnte keinen Nachweis über die verbundene Identität der Domain finden.',
    CAUTION: 'Mit Vorsicht fortfahren!',
  },
  ERROR: {
    TITLE: 'Hoppla!',
    DEFAULT_MESSAGE: 'Es ist ein Fehler aufgetreten. Bitte versuche es noch einmal.',
  },
  CANCEL: 'Abbrechen',
  CLOSE: 'Schließen',
  CONTINUE: 'Weiter',
  SKIP: 'Überspringen',
  REJECT: 'Ablehnen',
} satisfies Translation;

export default de;
