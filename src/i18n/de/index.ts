import type { Translation } from '../i18n-types';

const de = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Willkommen bei',
      WHAT_IS_UNIME_1: 'UniMe verbindet deine digitale Welt, sicher und geschützt.',
      WHAT_IS_UNIME_2: 'Erstelle ein brandneues Profil, um loszulegen.',
      CREATE_NEW_PROFILE: 'Neues Profil erstellen',
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
      THEME: {
        TITLE_1: 'Wähle das',
        TITLE_2: 'Erscheinungsbild',
        SUBTITLE: 'Bist du eher eine Nachteule?',
      },
      PICTURE: {
        TITLE_1: 'Wähle ein',
        TITLE_2: 'Profilbild',
        SUBTITLE: 'Mach es zu deinem.',
      },
      SKIP: {
        TITLE: 'Personalisierung überspringen',
        TEXT: 'Bist du sicher? Du kannst das Erscheinungsbild der App später in den Einstellungen anpassen.',
        CONFIRM: 'Ja',
        ABORT: 'Nein, lass uns weitermachen',
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
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Passwort eingeben',
    BUTTON_TEXT: 'Wallet entsperren',
    FORGOT_PASSWORD: 'Passwort vergessen?',
  },
  HI: 'Hallo {name}! Bitte hinterlasse einen Stern, wenn dir das Projekt gefällt: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Willkommen bei deinem UniMe',
  PROMPT_NAME: 'Bitte gib deinen Namen ein',
  CREATE_IDENTITY: 'Neue Identität anlegen',
  EMPTY_CREDENTIALS_LIST_TITLE: 'Hier ist es noch etwas leer',
  EMPTY_CREDENTIALS_LIST_SUBTITLE:
    'Warum fügst du nicht ein paar Credentials hinzu, um mit deiner neuen digitalen Identität loszulegen?',
  GETTING_STARTED: {
    TITLE: 'Sollen wir starten?',
    SUBTITLE: 'Mache deine ersten Schritte, um ein paar Credentials zu deiner digitalen Identität hinzuzufügen.',
    DIALOG_0_TITLE: 'Neue Ziele abschließen',
    DIALOG_0_TEXT:
      'Starte deine Mission hier! Ziele führen dich durch wichtige Funktionen und Möglichkeiten der UniMe App.',
    DIALOG_1_TITLE: 'Erkunden macht Spaß',
    DIALOG_1_TEXT:
      'Wenn du Ziele abschließt, erhältst du ein tolles neues Abzeichen und steigerst deine ersten Schritte.',
    SKIP_TITLE: 'Onboarding überspringen',
    SKIP_TEXT: 'Bist du sicher? Du kannst das Onboarding in den App-Einstellungen wieder aktivieren.',
  },
  APP_SETTINGS: 'App-Einstellungen',
  YOUR_DIDS: 'Deine DIDs',
  RESET_APP: 'App zurücksetzen',
  NO_HISTORY: 'Du hast noch keine Credentials verwendet.',
  SHARE_CREDENTIALS_TITLE: 'Wähle Informationen aus, die du teilen möchtest',
  SHARE_CREDENTIALS_CONFIRM: 'Freigeben',
  CANCEL: 'Abbrechen',
  PROFILE_NAME: 'Name',
  PASSWORD: 'Passwort',
  CHANGE_LATER: 'Du kannst den Namen später ändern.',
  STRONG_PASSWORD: 'Bitte wähle ein starkes Passwort.',
  FAVORITES: 'Favoriten',
  MY_DATA: 'Meine Daten',
  ACCOUNT: 'Account',
  SUPPORT: 'Support',
  ADD: 'Hinzufügen',
  CONTINUE: 'Weiter',
  SKIP: 'Überspringen',
} satisfies Translation;

export default de;
