import type { Translation } from '../i18n-types';

const nl = {
  HI: 'Hallo {name}!',
  WELCOME: 'Welkom',
  PROMPT_NAME: 'Voer uw naam in',
  CREATE_IDENTITY: 'Nieuwe identiteit creëren',
  CREATE_IDENTITY_SUCCESS:
    'U heeft zojuist een nieuwe digitale identiteit gecreëerd! U kunt nu meer informatie over uzelf toevoegen.'
} satisfies Translation;

export default nl;
