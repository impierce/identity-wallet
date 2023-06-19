import type { Translation } from '../i18n-types';

const nl = {
  HI: 'Hallo {name}!',
  WELCOME: 'Welkom',
  PROMPT_NAME: 'Voer uw naam in',
  CREATE_IDENTITY: 'Nieuwe identiteit creëren',
  CREATE_IDENTITY_SUCCESS_TITLE: 'U heeft zojuist een nieuwe digitale identiteit aangemaakt!',
  CREATE_IDENTITY_SUCCESS_BODY:
    'Ga verder door meer informatie over uzelf toe te voegen of een Credential te scannen.',
  APP_SETTINGS: 'App-instellingen',
  YOUR_DIDS: 'Uw DIDs',
  RESET_APP: 'App resetten',
  NO_HISTORY: `U heeft nog geen Credentials gebruikt.`
} satisfies Translation;

export default nl;
