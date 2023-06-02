import type { Translation } from '../i18n-types';

const de = {
  HI: 'Hallo {name}! Bitte hinterlasse einen Stern, wenn dir das Projekt gefällt: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Willkommen',
  PROMPT_NAME: 'Bitte gib deinen Namen ein',
  CREATE_IDENTITY: 'Neue Identiät anlegen',
  CREATE_IDENTITY_SUCCESS:
    'Du hast eine neue digitale Identität angelegt! Du kannst nun weitere Informationen über dich hinzufügen.'
} satisfies Translation;

export default de;
