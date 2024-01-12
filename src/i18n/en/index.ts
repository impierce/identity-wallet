import type { BaseTranslation } from '../i18n-types';

const en = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Welcome to',
      WHAT_IS_UNIME_1: 'UniMe connects your digital world, safely and securely.',
      WHAT_IS_UNIME_2: 'Create a brand new identity profile to get started.',
      CREATE_NEW_PROFILE: 'Create new profile',
    },
    PLEDGE: {
      NAVBAR_TITLE: 'UniMe Pledge',
      TITLE_1: 'No funny',
      TITLE_2: 'business',
      SUBTITLE: "Here's our pledge to you.",
      ITEM_1: {
        TITLE: 'We will not share your data',
        DESCRIPTION:
          'Your data belongs to you and only you decide who you share it with. Period. In fact, your data never even touches any of our systems - unless you opt-in to one of the cloud storage options.',
      },
      ITEM_2: {
        TITLE: 'We will not add trackers',
        DESCRIPTION:
          "We do not track your actions behind the scenes. Period. Not for testing or any other reasons. That's our pledge. We also do not collect any anonymous device information or usage statistics. That decision makes developing the app a bit harder for us, but we believe it is the right decision.",
      },
      ITEM_3: {
        TITLE: 'You own your information',
        DESCRIPTION: "We believe that it's about time you become the owner of your own personal information again.",
      },
    },
    TERMS: {
      NAVBAR_TITLE: 'Terms & Conditions',
      TITLE_1: "Here's the less interesting",
      TITLE_2: 'stuff',
      SUBTITLE: 'Yeah, we know. We still recommend you read this information carefully.',
      OWNERSHIP: {
        TITLE: 'Ownership',
        DESCRIPTION: 'I understand that I am solely responsible for my data.',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Customization',
      NAME: {
        TITLE_1: "Let's go! Choose a",
        TITLE_2: 'profile name',
        SUBTITLE: 'Your profile information will never leave your device.',
        INPUT_PLACEHOLDER: 'Enter a profile name',
      },
      THEME: {
        TITLE_1: 'Choose your app',
        TITLE_2: 'appearance',
        SUBTITLE: 'Are you more of a night owl?',
      },
      PICTURE: {
        TITLE_1: 'Set a display',
        TITLE_2: 'picture',
        SUBTITLE: 'Make it yours.',
      },
      SKIP: {
        TITLE: 'Skip customization',
        TEXT: 'Are you sure? You can adjust the app appearance later in settings.',
        CONFIRM: 'Yes',
        ABORT: "No, let's continue",
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: 'Password',
      TITLE_1: 'Set your new',
      TITLE_2: 'password',
      SUBTITLE: 'You need to choose a strong password to securely encrypt your data.',
      INPUT_PLACEHOLDER: 'Enter a password',
      CONFIRM: {
        NAVBAR_TITLE: 'Confirm Password',
        TITLE_1: 'Please confirm your new',
        TITLE_2: 'password',
        SUBTITLE: 'You need to confirm your password to make sure you typed it correctly.',
        INPUT_PLACEHOLDER: 'Retype your password',
        MATCH: 'Passwords match',
        NO_MATCH: 'Passwords do not match',
      },
      COMPLETED: {
        NAVBAR_TITLE: 'Password Set',
        TITLE_1: 'Your UniMe profile is now',
        TITLE_2: 'protected',
        MESSAGE_1: 'Safe & Secure.',
        MESSAGE_2: 'Nice Job',
      },
    },
  },
  SETTINGS: {
    THEME: {
      SYSTEM: 'System',
      LIGHT: 'Light',
      DARK: 'Dark',
    },
    PASSWORD: {
      POLICY: {
        TITLE: 'Your password must contain',
        UPPERCASE_LETTER: 'uppercase letter',
        LOWERCASE_LETTER: 'lowercase letter',
        NUMBER: 'number',
        CHARACTERS: 'characters',
      },
    },
    RESET_APP: {
      TITLE: 'Reset app',
      DESCRIPTION: 'Are you sure you want to reset the app?',
      CONFIRM: 'Yes, delete everything',
      CANCEL: 'No, keep my profile',
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Enter your password',
    BUTTON_TEXT: 'Unlock wallet',
    FORGOT_PASSWORD: 'Forgot password?',
  },
  HI: 'Hi {name:string}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n',
  WELCOME: 'Welcome to your UniMe',
  PROMPT_NAME: 'Please enter your name',
  CREATE_IDENTITY: 'Create new identity',
  EMPTY_CREDENTIALS_LIST_TITLE: `It's a bit quiet in here`,
  EMPTY_CREDENTIALS_LIST_SUBTITLE: 'Why not add some credentials to start your new digital me?',
  GETTING_STARTED: {
    TITLE: 'Shall we get started?',
    SUBTITLE: 'Start your first steps to add some credentials to your "Me".',
    DIALOG_0_TITLE: 'Complete new goals',
    DIALOG_0_TEXT:
      'Start your mission here! Goals will lead you through important features and possibilities of UniMe app.',
    DIALOG_1_TITLE: 'Exploring made fun',
    DIALOG_1_TEXT: `On completing goals, you'll receive an awesome new achievement badge and level up your first steps.`,
    SKIP_TITLE: 'Skip onboarding',
    SKIP_TEXT: 'Are you sure? You can re-enable the onboarding later in the app settings.',
  },
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
  FAVORITES: 'My favorites',
  MY_DATA: 'My data',
  ACCOUNT: 'Account',
  SUPPORT: 'Support',
  ADD: 'Add',
  CONTINUE: 'Continue',
  SKIP: 'Skip',
} satisfies BaseTranslation;

export default en;
