import type { Translation } from '../i18n-types';

const de = {
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
    SKIP_TEXT: 'Bist du sicher? Du kannst das Onboarding in den App-Einstellungen wieder aktivieren.'
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
  ADD: 'Hinzufügen',
  CONTINUE: 'Weiter',
  SKIP: 'Überspringen'
} satisfies Translation;

export default de;
