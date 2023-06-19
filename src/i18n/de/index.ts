import type { Translation } from '../i18n-types';

const de = {
  HI: 'Hallo {name}! Bitte hinterlasse einen Stern, wenn dir das Projekt gefällt: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Willkommen',
  PROMPT_NAME: 'Bitte gib deinen Namen ein',
  CREATE_IDENTITY: 'Neue Identiät anlegen',
  CREATE_IDENTITY_SUCCESS_TITLE: 'Du hast eine neue digitale Identität angelegt!',
  CREATE_IDENTITY_SUCCESS_BODY:
    'Du kannst nun weitere Informationen über dich hinzufügen oder einen Credential scannen.',
  APP_SETTINGS: 'App-Einstellungen',
  YOUR_DIDS: 'Deine DIDs',
  RESET_APP: 'App zurücksetzen',
  NO_HISTORY: `Du hast noch keine Credentials verwendet.`
} satisfies Translation;

export default de;
