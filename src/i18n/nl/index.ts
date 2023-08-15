import type { Translation } from '../i18n-types';

const nl = {
  HI: 'Hallo {name}!',
  WELCOME: 'Welkom bij je UniMe',
  PROMPT_NAME: 'Voer je naam in',
  CREATE_IDENTITY: 'Nieuwe identiteit creëren',
  EMPTY_CREDENTIALS_LIST_TITLE: 'Het is hier een beetje stil',
  EMPTY_CREDENTIALS_LIST_SUBTITLE:
    'Waarom voeg je geen Credentials toe om je nieuwe digitale ik te starten?',
  GETTING_STARTED: {
    TITLE: 'Zullen we beginnen?',
    SUBTITLE: 'Maak je eerste stappen om wat Credentials toe te voegen aan je digitale ik.',
    DIALOG_0_TITLE: 'Nieuwe doelen bereiken',
    DIALOG_0_TEXT:
      'Start je missie hier! Doelen leiden je door belangrijke functies en mogelijkheden van de UniMe app.',
    DIALOG_1_TITLE: 'Verkenning maakt plezier',
    DIALOG_1_TEXT: `Door doelen te voltooien, ontvang je een geweldig nieuw prestatiebadge en level je je eerste stappen.`,
    SKIP_TITLE: 'Onboarding overslaan',
    SKIP_TEXT:
      'Weet je het zeker? Je kunt de onboarding opnieuw inschakelen in de app-instellingen.'
  },
  APP_SETTINGS: 'App-instellingen',
  YOUR_DIDS: 'Jouw DIDs',
  RESET_APP: 'App resetten',
  NO_HISTORY: 'Je hebt nog geen Credentials gebruikt.',
  SHARE_CREDENTIALS_TITLE: 'Selecteer de informatie die je wilt delen',
  SHARE_CREDENTIALS_CONFIRM: 'Delen',
  CANCEL: 'Annuleren',
  PROFILE_NAME: 'Naam',
  PASSWORD: 'Wachtwoord',
  CHANGE_LATER: 'Je kunt de naam later wijzigen.',
  STRONG_PASSWORD: 'Kies een sterk wachtwoord.',
  FAVORITES: 'Favorieten',
  MY_DATA: 'Mijn gegevens',
  ADD: 'Toevoegen',
  CONTINUE: 'Doorgaan',
  SKIP: 'Overslaan'
} satisfies Translation;

export default nl;
