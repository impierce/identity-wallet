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
      SUBTITLE: 'Yes, we know. We still recommend you read this information carefully.',
      T_AND_C: {
        TITLE: 'Terms & Conditions',
        DESCRIPTION: 'I have read and agree to the Terms & Conditions.',
        DIALOG_TITLE: 'UniMe Terms of Use',
        LAST_UPDATED: 'Last Updated: September 10, 2025',
        TL_DR:
          'Like any app provider, we’re legally required to give you some important terms and conditions. We’ve done our best to keep things clear and to the point. Here’s the short version: UniMe puts you in control. It doesn’t collect your data, respects your privacy, and lets you decide what information to store and share. Your identity and data stay with you, and so does the responsibility for managing them. We don’t store nor can access your data, so any data loss caused by losing your device or access to the app is also your responsibility. With great power comes great responsibility. Here’s everything you should know:',
        FULL: {
          AGREEMENT: {
            TITLE: 'Acceptance of Terms',
            DESCRIPTION:
              'Thank you for choosing UniMe, an EU Digital Identity Wallet (mobile application) developed and maintained by Impierce Technologies B.V. ("we," "us," or "our"). These Terms of Use ("Terms") govern your access to and use of the UniMe mobile application (the "Service"), whether downloaded from the Apple App Store, Google Play Store, or any other platform. By installing, accessing, or using the Service, you acknowledge that you have read, understood, and agreed to be bound by these Terms and our Privacy Policy. If you do not agree to any part of these Terms, please do not use the Service. If you are using the Service on behalf of an organization, you represent and warrant that you are authorized to accept these Terms on that organization’s behalf.',
          },
          DEFINITIONS: {
            TITLE: 'Definitions',
            DESCRIPTION:
              'The term Service refers to the UniMe Identity Wallet mobile application and includes any features, content, or services provided within it. User Data means any data, credentials, or information that you store or share using the Service. The term Device refers to the mobile phone or hardware on which the UniMe app is installed and used.',
          },
          USER_RESPONSIBILITIES: {
            TITLE: 'User Responsibilities',
            DESCRIPTION:
              'You are responsible for maintaining the confidentiality and security of your device and any credentials stored within the Service, and for ensuring that any information you store or share using the Service is accurate and up-to-date. You agree to use the Service in compliance with all applicable laws and regulations. Impierce Technologies B.V. is not liable for any data loss due to device loss or unauthorized access.',
          },
          DATA_OWNERSHIP: {
            TITLE: 'Data Ownership and Privacy',
            DESCRIPTION:
              'UniMe is built on the core principles of user control and data privacy. Our architecture is designed so that we cannot access, collect, store, or process the User Data you manage within the app. All of your User Data is stored locally and securely on your Device. We employ robust security measures within the application to protect your data. However, the ultimate protection of your data also depends on you maintaining the overall security of your personal Device, for example by using a strong passcode and not installing software from untrusted sources. To maintain the integrity of your credentials, UniMe may periodically check their validity. This is done by contacting the credential issuer directly from your Device to confirm whether a credential has been revoked, by the issuer. This process happens automatically on your Device and does not involve Impierce Technologies B.V. Where available, UniMe prioritizes using decentralized methods for these checks.',
          },
          DATA_VISIBILITY: {
            TITLE: 'Data Visibility by App Platforms',
            DESCRIPTION:
              'When you download or use UniMe through an official App Store, such as the Apple App Store or Google Play, the platform may collect limited technical and usage data. This can include details like your device type, app installation or crash events, and the region or language settings of your device. This information is collected under the App Store’s own policies. Impierce does not collect any additional usage data from the device itself.',
          },
          INTELLECTUAL_PROPERTY_RIGHTS: {
            TITLE: 'Intellectual Property Rights',
            DESCRIPTION:
              'All intellectual property rights in UniMe and related materials are owned by Impierce Technologies B.V. or our licensors. UniMe is provided under the Apache 2.0 license, which means you can use, modify, and distribute the software as long as you comply with the terms of that license. For more information, please refer to the full Apache 2.0 License.',
          },
          PROHIBITED_ACTIVITIES: {
            TITLE: 'Prohibited Activities',
            DESCRIPTION:
              'You agree not to attempt to gain unauthorized access to the Service or any related systems, disrupt or interfere with the performance or security of the Service, or use the Service for any unlawful, harmful, or fraudulent purposes.',
          },
          THIRD_PARTY_SERVICES: {
            TITLE: 'Third-Party Services',
            DESCRIPTION:
              'Our Service may link to or integrate with third-party platforms. These platforms are not controlled or operated by us, and we do not endorse or assume responsibility for their content, actions, or data practices. Any services or interactions you choose to engage in with third-party providers are at your own risk and are subject to their own terms and policies. We recommend reviewing those carefully before proceeding. The Service is provided "as is" and "as available" with no warranties of any kind. We do not guarantee uninterrupted, error-free use of the Service.',
          },
          LIABILITY: {
            TITLE: 'Limitation of Liability',
            DESCRIPTION:
              'To the fullest extent permitted by law, Impierce Technologies B.V. is not liable for any indirect, incidental, special, or consequential damages. This includes, but is not limited to, loss of data, profits, business opportunities, or goodwill resulting from your use of or inability to use the Service; any conduct, content, or errors of third parties; any services, content, or actions of third-party platforms accessed through the Service; or any content obtained from or through the Service.',
          },
          INDEMNIFICATION: {
            TITLE: 'Indemnification',
            DESCRIPTION:
              'You agree to indemnify and hold harmless Impierce Technologies B.V. and its affiliates from any claims, losses, or expenses resulting from your use of the Service, your violation of these Terms of Use, or your violation of any rights of another person or entity.',
          },
          MODIFICATIONS: {
            TITLE: 'Modifications to the Terms of Use',
            DESCRIPTION:
              'We may update these Terms of Use from time to time. The "Last Updated" date above reflects the latest version. Significant changes will be communicated through the app or our website. Continued use of the Service means you accept the revised Terms of Use.',
          },
          LAW_AND_JURISDIFICATION: {
            TITLE: 'Governing Law and Jurisdiction',
            DESCRIPTION:
              'These Terms of Use are governed by Dutch law. Any disputes will be resolved in the courts of the Netherlands.',
          },
          SEVERABILITY: {
            TITLE: 'Severability',
            DESCRIPTION: 'If any part of these Terms of Use is found invalid, the rest will remain in full force.',
          },
          LANGUAGE: {
            TITLE: 'Language',
            DESCRIPTION:
              'These Terms of Use are available in multiple languages for convenience. In case of any conflict, the English version prevails.',
          },
          ENTIRE_AGREEMENT: {
            TITLE: 'Entire Agreement',
            DESCRIPTION:
              'These Terms of Use, along with our Privacy Policy, form the complete agreement between you and Impierce Technologies B.V. regarding the use of UniMe.',
          },
          CONTACT: {
            TITLE: 'Contact Us',
            DESCRIPTION:
              'Got feedback or a question? We’re always looking to improve. If anything’s unclear or could be said better, feel free to reach out to contact@impierce.com. By using UniMe, you confirm that you’ve read, understood, and agree to these Terms of Use. We remain committed to delivering secure, privacy-centric digital tools that empower you.',
          },
        },
      },
      OWNERSHIP: {
        TITLE: 'Data Ownership',
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
      PICTURE: {
        TITLE_1: 'Set a display',
        TITLE_2: 'picture',
        SUBTITLE: 'Make it yours.',
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
      BIOMETRICS: {
        TITLE: 'Enable {type:string}',
        DESCRIPTION: 'Do you want to set up {type:string} to unlock the app?',
        CONFIRM: 'Yes, use {type:string}',
        DECIDE_LATER: 'Decide later',
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
        EDIT: 'Edit',
        CHANGE: 'Select a profile picture',
        REMOVE: 'Remove',
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
        LABEL: 'Theme',
        NAVBAR_TITLE: 'Select Theme',
        TITLE_1: 'Choose your app',
        TITLE_2: 'appearance',
        SUBTITLE: 'Are you more of a night owl?',
      },
      SECURITY: {
        LABEL: 'Security',
        NAVBAR_TITLE: 'Security',
        SWITCH_LABEL: 'Unlock with {type:string}',
        BIOMETRIC_TYPE: {
          ANDROID: {
            FACE_ID: 'facial recognition',
            TOUCH_ID: 'fingerprint',
          },
          IOS: {
            FACE_ID: 'Face ID',
            TOUCH_ID: 'Touch ID',
          },
          GENERIC: 'biometrics',
        },
        ENABLE: {
          DIALOG_TITLE: 'Enable {type:string}',
          DIALOG_CONTENT: 'Please enter your password to enable {type:string}.',
        },
        DISABLE: {
          DIALOG_TITLE: 'Disable {type:string}',
          DIALOG_CONTENT: 'Please enter your password to disable {type:string}.',
        },
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
        NAVBAR_TITLE: 'About UniMe',
        SPECIFICATIONS: 'Specifications',
        VERSION: 'Version',
        LICENSE: 'License',
        BUILT_WITH: 'Built with Tauri',
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
    BOTTOM_NAVIGATION_TITLE: 'Me',
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
      SUBTITLE: 'Why not get some of your data verified to start your journey?',
    },
    FAVORITES: 'My favorites',
  },
  ACTIVITY: {
    BOTTOM_NAVIGATION_TITLE: 'Activity',
    NAVBAR_TITLE: 'Connected',
    TABS: {
      CONNECTIONS: 'Connections',
      HISTORY: 'History',
    },
  },
  SCAN: {
    BOTTOM_NAVIGATION_TITLE: 'Scan',
    TITLE_1: 'Scan a',
    TITLE_2: 'QR Code',
    SUBTITLE: 'Bring a QR Code into view of this screen to start an interaction.',
    PERMISSION_DENIED: 'No permission to access the camera',
    OPEN_SETTINGS: 'Open settings',
    CREDENTIAL_OFFER: {
      NAVBAR_TITLE: 'Credential Offer',
      DESCRIPTION: 'is offering you the following credentials',
      ACCEPT: 'Accept credentials',
    },
    CONNECTION_REQUEST: {
      NAVBAR_TITLE: 'Connection Request',
      TITLE: 'New connection',
      DESCRIPTION: 'Only accept new connections that you recognize and trust',
      CONNECTED_PREVIOUSLY: 'Connected previously',
      ACCEPT: 'Accept connection',
    },
    SHARE_CREDENTIALS: {
      NAVBAR_TITLE: 'Share Data',
      DESCRIPTION: 'requests the following credentials',
      REQUESTED: 'Requested',
      APPROVE: 'Approve request',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Summary',
      DATA: 'Data',
      ACTIVITY: 'Activity',
    },
    SUMMARY: {
      EMPTY: 'No connections yet.',
      TITLE: 'Connected to',
      FIRST_CONNECTED: 'First connected',
      LAST_CONNECTED: 'Last connected',
    },
    DATA: {
      EMPTY: 'No data yet.',
    },
  },
  HISTORY: {
    EMPTY: 'No activity yet.',
    DATA_RECEIVED: 'Received data from',
    DATA_SHARED: 'Shared data with',
    CONNECTION_ADDED: 'Connected to',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Search for something',
    NO_QUERY: {
      TITLE: 'What shall we search for?',
      DESCRIPTION: 'Search for any of your credentials and badges here.',
    },
    NO_RESULTS: {
      TITLE: 'No results found',
      DESCRIPTION: 'Try searching for something else.',
    },
    RECENT_SEARCHES: 'Recent searches',
  },
  CREDENTIAL: {
    NAVBAR_TITLE: 'Credential Information',
    DETAILS: {
      VALID: 'Valid',
      INVALID: 'Invalid',
      UNVERIFIED: 'Unverified',
      ISSUED_BY: 'Issued by',
      SELF_SIGNED: 'yourself',
      DESCRIPTION: 'Description',
      OPEN_BADGES: {
        ALIGNMENT: 'Alignment',
        CRITERIA: 'Criteria',
      },
    },
    RENDERER: {
      PID: {
        NAME: 'Name',
        NATIONALITY: 'Nationality',
        BIRTH_DATE: 'Date of birth',
        BIRTH_PLACE: 'Place of birth',
      },
    },
    ACTIONS: {
      EDIT: {
        MENU_BUTTON: 'Edit display name',
        CONFIRM_BUTTON: 'Update display name',
      },
      DELETE: {
        MENU_BUTTON: 'Delete credential',
        TITLE: 'Delete credential',
        DESCRIPTION: 'Are you sure you want to delete this credential from your wallet? This action cannot be undone.',
        CONFIRM_BUTTON: 'Delete',
      },
    },
  },
  ADD_CREDENTIALS: {
    BUTTON: 'Add',
    NAVBAR_TITLE: 'Add data',
    VALUE_REQUIRED: 'Required',
    LABEL_DISCLAIMER: 'Only seen by you',
    FAVORITES_TOGGLE_LABEL: 'Add to favorites',
    ADDRESS: {
      TITLE: 'Address',
      DESCRIPTION: 'Add your home or work address',
      INFO: {
        NAVBAR_TITLE: 'Verified address',
        TITLE: 'Verified address',
        DESCRIPTION: 'Before you start',
        ITEM_0: {
          TITLE: 'All your addresses in one place',
          DESCRIPTION:
            'Keep your home, office, or any other address securely in your wallet, ready to use instantly on supporting online platforms.',
        },
        ITEM_1: {
          TITLE: 'Tap, confirm, and you’re done',
          DESCRIPTION:
            'When a service asks for an address, just scan their code or tap ’Connect’, choose which address you want to share, and confirm. It’s done in a heartbeat.',
        },
        ITEM_2: {
          TITLE: 'Locked down and secure',
          DESCRIPTION:
            'Your addresses are for your eyes only until you decide to share them. Every share is encrypted, logged, and requires your approval, giving you a full overview of who has your details.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Add address',
        LABEL: 'How do you want to call this address?',
        LABEL_PLACEHOLDER: 'Home address',
        RESIDENT_ADDRESS_LABEL: 'Address',
        RESIDENT_COUNTRY_LABEL: 'Country',
        RESIDENT_COUNTRY_PLACEHOLDER: 'Choose a country',
        RESIDENT_COUNTRY_NO_MATCH: 'No countries found',
        RESIDENT_STATE_LABEL: 'State',
        RESIDENT_STREET_LABEL: 'Street',
        RESIDENT_HOUSE_NUMBER_LABEL: 'House number',
        RESIDENT_POSTAL_CODE_LABEL: 'Postal code',
        RESIDENT_CITY_LABEL: 'City',
        CREATE_BUTTON: 'Add address',
      },
    },
    EMAIL: {
      TITLE: 'Email',
      DESCRIPTION: 'Get your personal or work email verified',
      INFO: {
        NAVBAR_TITLE: 'Verified email',
        TITLE: 'Verified email',
        DESCRIPTION: 'Before you start',
        ITEM_0: {
          TITLE: 'Prove it’s really you',
          DESCRIPTION:
            'A verified email acts as your digital handshake. It gives friends, colleagues, and services confidence that it’s genuinely you, boosting trust in your online interactions.',
        },
        ITEM_1: {
          TITLE: 'You’re always in control',
          DESCRIPTION:
            'You decide exactly when and where to use your verified email. Each share is encrypted and logged in your activity history.',
        },
        ITEM_2: {
          TITLE: 'Secure and private, by default',
          DESCRIPTION:
            'We send a one-time code to your email. You enter it, and the secure credential stays locked on your phone, never shared without your direct approval.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Get verified email',
        LABEL: 'How do you want to call this email address?',
        LABEL_PLACEHOLDER: 'Personal email',
        VALUE_LABEL: 'Email',
        VALUE_PLACEHOLDER: 'firstname.lastname@example.com',
        VALUE_PATTERN_ERROR: 'Please provide a valid email address',
        BUTTON_SEND: 'Send verification email',
        BUTTON_SEND_AGAIN: 'Resend verification email',
        CHECK_EMAIL: 'Please check your email inbox and enter the code below.',
        EXPIRED_ERROR: 'Verification code expired',
      },
    },
    PROFILE: {
      TITLE: 'Profile',
      DESCRIPTION: 'Add information about yourself',
      INFO: {
        NAVBAR_TITLE: 'Your profile',
        TITLE: 'UniMe Profile',
        DESCRIPTION: 'Before you start',
        ITEM_0: {
          TITLE: 'Skip tedious forms',
          DESCRIPTION:
            'Your UniMe profile acts like a digital ID. Use it to auto-fill details instantly on supported sites, cutting the clutter from your online life.',
        },
        ITEM_1: {
          TITLE: 'Instant, sharing',
          DESCRIPTION:
            'When your details are requested, simply scan a QR code or tap ‘Connect’, check exactly which information is being requested, and approve. You’re done in seconds.',
        },
        ITEM_2: {
          TITLE: 'Private by design',
          DESCRIPTION:
            'Your information stays locked on your device, never stored in the cloud. Every share is end-to-end encrypted. Your data is always yours.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Your profile',
        LABEL: 'How do you want to call this profile?',
        LABEL_PLACEHOLDER: 'Personal Profile',
        FIRST_NAME_LABEL: 'First name',
        FIRST_NAME_PLACEHOLDER: 'Your first name',
        MIDDLE_NAME_LABEL: 'Middle name(s)',
        MIDDLE_NAME_PLACEHOLDER: 'Your middle name(s)',
        LAST_NAME_LABEL: 'Last name',
        LAST_NAME_PLACEHOLDER: 'Your last name',
        BIRTH_DATE_LABEL: 'Date of birth',
        BIRTH_DATE_PLACEHOLDER: 'Select your date of birth',
        BIRTH_PLACE_LABEL: 'Place of birth',
        BIRTH_PLACE_PLACEHOLDER: 'Your place of birth',
        NATIONALITY_LABEL: 'Nationality',
        CREATE_BUTTON: 'Create profile',
      },
    },
  },
  SORT: {
    TITLE: 'Sorting',
    PREFERENCES: {
      LIST_VIEW: 'List View',
      GRID_VIEW: 'Grid View',
      ALPHABETICAL: 'Alphabetical',
      DATE_ISSUED: 'Date Issued',
      DATE_ADDED: 'Date Added',
    },
    ORDER: {
      A_Z: 'A to Z',
      Z_A: 'Z to A',
      NEWEST: 'Newest first',
      OLDEST: 'Oldest first',
    },
  },
  DOMAIN_LINKAGE: {
    TITLE: 'Verified website',
    SUCCESS: 'UniMe successfully verified the identity to provide you with a secure login.',
    FAILURE: 'UniMe could not verify the linkage of the identity to the domain.',
    UNKNOWN: "UniMe could not find any proof of the domain's associated identity.",
    CAUTION: 'Proceed with caution!',
  },
  ERROR: {
    TITLE: 'Oops!',
    DEFAULT_MESSAGE: 'Something went wrong. Please try again.',
  },
  CANCEL: 'Cancel',
  CLOSE: 'Close',
  DISCARD: 'Discard',
  CONTINUE: 'Continue',
  SKIP: 'Skip',
  ACCEPT: 'Accept',
  REJECT: 'Reject',
  // TODO: AI generated to fix TS errors in journeys.
  GETTING_STARTED: {
    SKIP_TITLE: 'Skip Getting Started',
    SKIP_TEXT: 'Are you sure you want to skip the getting started guide?',
  },
} satisfies BaseTranslation;

export default en;
