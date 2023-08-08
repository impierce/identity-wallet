import type { Translation } from '../i18n-types';

const de = {
  HI: 'Hallo {name}! Bitte hinterlasse einen Stern, wenn dir das Projekt gefällt: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Willkommen',
  PROMPT_NAME: 'Bitte gib deinen Namen ein',
  CREATE_IDENTITY: 'Neue Identität anlegen',
  EMPTY_CREDENTIALS_LIST_TITLE: 'Hier ist es noch etwas leer',
  EMPTY_CREDENTIALS_LIST_SUBTITLE:
    'Warum fügst du nicht ein paar Credentials hinzu, um mit deiner neuen digitalen Identität zu starten?',
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
  STRONG_PASSWORD: 'Bitte wähle ein starkes Passwort.'
} satisfies Translation;

export default de;
