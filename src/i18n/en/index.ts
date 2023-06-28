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
  RESET_APP: 'Reset app',
  NO_HISTORY: `You haven't used any credentials yet.`,
  SHARE_CREDENTIALS_TITLE: 'Select information you want to share',
  SHARE_CREDENTIALS_CONFIRM: 'Share',
  CANCEL: 'Cancel',
  PROFILE_NAME: 'Name',
  PASSWORD: 'Password',
  CHANGE_LATER: 'You can change this later.',
  STRONG_PASSWORD: 'Please choose a strong password.'
} satisfies BaseTranslation;

export default en;
