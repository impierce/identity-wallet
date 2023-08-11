import type { BaseTranslation } from '../i18n-types';

const en = {
  HI: 'Hi {name:string}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Welcome to your UniMe',
  PROMPT_NAME: 'Please enter your name',
  CREATE_IDENTITY: 'Create new identity',
  EMPTY_CREDENTIALS_LIST_TITLE: `It's a bit quiet in here`,
  EMPTY_CREDENTIALS_LIST_SUBTITLE: 'Why not add some credentials to start your new digital me?',
  GETTING_STARTED_TITLE: 'Shall we get started?',
  GETTING_STARTED_SUBTITLE: 'Start your first steps to add some credentials to your "Me".',
  GETTING_STARTED_DIALOG_0_TITLE: 'Complete new goals',
  GETTING_STARTED_DIALOG_0_TEXT: 'Start your mission here! Goals will lead you through important features and possibilities of UniMe app.',
  GETTING_STARTED_DIALOG_1_TITLE: 'Exploring made fun',
  GETTING_STARTED_DIALOG_1_TEXT: `On completing goals, you'll receive an awesome new achievement badge and level up your first steps.`,
  GETTING_STARTED_SKIP_TITLE: '',
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
  STRONG_PASSWORD: 'Please choose a strong password.',
  FAVORITES: 'Favorites',
  MY_DATA: 'My data',
  ADD: 'Add',
  CONTINUE: 'Continue',
  SKIP: 'Skip'
} satisfies BaseTranslation;

export default en;
