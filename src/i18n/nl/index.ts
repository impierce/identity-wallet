import type { Translation } from '../i18n-types';

const nl = {
  HI: 'Hallo {name}!',
  WELCOME: 'Welkom',
  PROMPT_NAME: 'Voer je naam in',
  CREATE_IDENTITY: 'Nieuwe identiteit creëren',
  CREATE_IDENTITY_SUCCESS_TITLE: 'Je hebt een nieuwe digitale identiteit gecreëerd!',
  CREATE_IDENTITY_SUCCESS_BODY:
    'Ga verder door meer informatie over jezelf toe te voegen of een Credential te scannen.',
  APP_SETTINGS: 'App-instellingen',
  YOUR_DIDS: 'Jouw DIDs',
  RESET_APP: 'App resetten',
  NO_HISTORY: 'Je hebt nog geen Credentials gebruikt.',
  SHARE_CREDENTIALS_TITLE: 'Selecteer informatie die je wilt delen',
  SHARE_CREDENTIALS_CONFIRM: 'Delen',
  CANCEL: 'Annuleren',
} satisfies Translation;

export default nl;
