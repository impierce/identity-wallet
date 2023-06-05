import type { BaseTranslation } from '../i18n-types';

const en = {
  HI: 'Hi {name:string}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Welcome',
  PROMPT_NAME: 'Please enter your name',
  CREATE_IDENTITY: 'Create new identity',
  CREATE_IDENTITY_SUCCESS: 'You have just created a new digital identity',
  AUTHENTICATE: 'Send your requested data',
  ENTER_CLAIMS: 'Enter your requested data',
} satisfies BaseTranslation;

export default en;
