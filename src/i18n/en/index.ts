import type { BaseTranslation } from '../i18n-types';

const en = {
  HI: 'Hi {name:string}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Welcome',
  PROMPT_NAME: 'Please enter your name',
  CREATE_IDENTITY: 'Create new identity',
  CREATE_IDENTITY_SUCCESS_TITLE: 'You have just created a new digital identity!',
  CREATE_IDENTITY_SUCCESS_BODY:
    'Continue by adding more information about yourself or scan a credential.',
  APP_SETTINGS: 'App settings',
  YOUR_DIDS: 'Your DIDs',
  RESET_APP: 'Reset app'
} satisfies BaseTranslation;

export default en;
