import type { BaseTranslation } from '../i18n-types';

const en = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Welcome to',
      WHAT_IS_UNIME_1: 'UniMe connects your digital world, safely and securely.',
      WHAT_IS_UNIME_2: 'Create a brand new identity profile to get started.',
      CREATE_NEW_PROFILE: 'Create new profile',
      SELECT_LANGUAGE: 'Select language',
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
    NAVBAR_TITLE: 'Settings',
    PROFILE: {
      TITLE: 'My profile',
      PROFILE_NAME: {
        TITLE: 'Profile name',
        NAVBAR_TITLE: 'Change profile name',
        INPUT_PLACEHOLDER: 'Enter a profile name',
        CONFIRM: 'Update',
      },
      DISPLAY_PICTURE: {
        TITLE: 'Display picture',
        EDIT: 'Edit',
        CHANGE: 'Select a profile picture',
      },
      DELETE_PROFILE: {
        TITLE: 'Delete profile',
      },
    },
    APP: {
      TITLE: 'App settings',
      NAVBAR_TITLE: 'App Settings',
      LANGUAGE: {
        TITLE: 'Language',
        NAVBAR_TITLE: 'Select Language',
        COMING_SOON: 'Coming soon',
      },
      THEME: {
        TITLE: 'Theme',
        NAVBAR_TITLE: 'Select Theme',
      },
      PASSWORD: {
        TITLE: 'Password',
      },
      ONBOARDING_JOURNEY: {
        TITLE: 'Onboarding journey',
        BUTTON_TEXT: 'Restart',
      },
      HINTS_AND_TIPS: {
        TITLE: 'Hints and tips',
        BUTTON_TEXT: 'Reset',
      },
      DEVELOPER_MODE: {
        TITLE: 'Developer mode',
      },
    },
    BACKUP_RECOVERY: {
      TITLE: 'Backup and recovery',
    },
    LOG_OUT: {
      TITLE: 'Log out',
    },
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
      DESCRIPTION: 'Are you sure you want to reset the app and remove all data?',
      CONFIRM: 'Yes, delete everything',
      CANCEL: 'No, keep my profile',
    },
    ACCOUNT: 'Account',
    SUPPORT: {
      TITLE: 'Support',
      ABOUT: {
        TITLE: 'About UniMe',
      },
      FEEDBACK: {
        TITLE: 'Send feedback',
      },
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Enter your password',
    BUTTON_TEXT: 'Unlock wallet',
    FORGOT_PASSWORD: 'Forgot password?',
  },
  ME: {
    GREETINGS: {
      GREETING_0: 'Hey',
      GREETING_1: "What's up",
      GREETING_2: 'How are you',
      GREETING_3: 'Welcome back',
      GREETING_4: 'Hello',
    },
    CREDENTIAL_TABS: {
      ALL: 'All',
      DATA: 'Data',
      BADGES: 'Badges',
    },
    EMPTY_CREDENTIALS: {
      TITLE: "It's a bit quiet in here",
      SUBTITLE: 'Why not add some credentials to start your new digital me?',
    },
    FAVORITES: 'My favorites',
  },
  ACTIVITY: {
    NAVBAR_TITLE: 'Connected',
    TABS: {
      CONNECTIONS: 'Connections',
      TIMELINE: 'Timeline',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Summary',
      DATA: 'Data',
      ACTIVITY: 'Activity',
    },
    SUMMARY: {
      TITLE: 'Connected to',
      FIRST_CONNECTED: 'First connected',
      LAST_CONNECTED: 'Last connected',
    },
    DATA: {
      EMPTY: 'No data yet.',
    },
  },
  TIMELINE: {
    EMPTY: 'No activity yet.',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Look for something',
    NO_QUERY: {
      TITLE: 'What shall we search for?',
      DESCRIPTION: 'Search for any of your credentials and badges here.',
    },
    NO_RESULTS: {
      TITLE: 'No results found',
      DESCRIPTION: 'Try searching for something else.',
    },
  },
  BADGE: {
    DETAILS: {
      VALID: 'Valid',
      ISSUED_BY: 'Issued by',
      DESCRIPTION: 'Description',
      CONTENTS: 'Contents',
    },
  },
  CANCEL: 'Cancel',
  CLOSE: 'Close',
  CONTINUE: 'Continue',
  SKIP: 'Skip',
} satisfies BaseTranslation;

export default en;
