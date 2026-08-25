import type { Translation } from '../i18n-types';

const sv_FI = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Välkommen till',
      WHAT_IS_UNIME_1: 'UniMe kopplar ihop din digitala värld säkert.',
      WHAT_IS_UNIME_2: 'Skapa en helt ny identitetsprofil för att komma igång.',
      CREATE_NEW_PROFILE: 'Skapa ny profil',
      SELECT_LANGUAGE: 'Välj språk',
    },
    PLEDGE: {
      NAVBAR_TITLE: 'UniMe-löfte',
      TITLE_1: 'Inget fuffens',
      TITLE_2: 'här',
      SUBTITLE: 'Här är vårt löfte till dig.',
      ITEM_1: {
        TITLE: 'Vi delar inte dina data',
        DESCRIPTION:
          'Dina data tillhör dig och bara du bestämmer vem du delar dem med. Punkt. Dina data rör inte våra system om du inte väljer ett molnlagringsalternativ.',
      },
      ITEM_2: {
        TITLE: 'Vi lägger inte in spårare',
        DESCRIPTION:
          'Vi spårar inte dina handlingar i bakgrunden. Punkt. Inte för test eller något annat. Vi samlar inte heller in anonym enhets- eller användningsstatistik.',
      },
      ITEM_3: {
        TITLE: 'Du äger din information',
        DESCRIPTION: 'Det är dags att du åter blir ägare till din personliga information.',
      },
    },
    TERMS: {
      NAVBAR_TITLE: 'Villkor',
      TITLE_1: 'Här är den mindre',
      TITLE_2: 'roliga delen',
      SUBTITLE: 'Vi rekommenderar ändå att du läser detta noggrant.',
      T_AND_C: {
        TITLE: 'Villkor',
        DESCRIPTION: 'Jag har läst och godkänner villkoren.',
        DIALOG_TITLE: 'UniMe Användarvillkor',
        LAST_UPDATED: 'Senast uppdaterad: 10 september 2025',
        TL_DR:
          'Liksom alla appleverantörer är vi enligt lag skyldiga att informera dig om vissa viktiga villkor. Vi har gjort vårt bästa för att formulera dem på ett tydligt och koncist sätt. Här är en kort sammanfattning: UniMe ger dig kontrollen. Appen samlar inte in dina data, respekterar din integritet och låter dig bestämma vilken information som ska lagras och delas. Din identitet och dina data förblir dina, liksom ansvaret för att hantera dem. Vi lagrar inte och har inte tillgång till dina data, så eventuella dataförluster till följd av att du tappar bort din enhet eller förlorar tillgången till appen är också ditt ansvar. Med stor makt följer stort ansvar. Här är allt du behöver veta:',
        // Note: this translation had not been verified by a legal expert.
        FULL: {
          AGREEMENT: {
            TITLE: 'Godkännande av villkoren',
            DESCRIPTION:
              'Tack för att du har valt UniMe, en digital identitetsplånbok (mobilapp) för EU som utvecklats och underhålls av Impierce Technologies B.V. (”vi”, ”oss” eller ”vår”). Dessa användarvillkor (”villkoren”) reglerar din åtkomst till och användning av mobilappen UniMe (”tjänsten”), oavsett om den laddats ned från Apple App Store, Google Play Store eller någon annan plattform. Genom att installera, komma åt eller använda Tjänsten bekräftar du att du har läst, förstått och samtyckt till att vara bunden av dessa Villkor och vår integritetspolicy. Om du inte samtycker till någon del av dessa Villkor, vänligen använd inte Tjänsten. Om du använder Tjänsten på uppdrag av en organisation, intygar och garanterar du att du är behörig att acceptera dessa Villkor på organisationens vägnar.',
          },
          DEFINITIONS: {
            TITLE: 'Definitioner',
            DESCRIPTION:
              'Termen Tjänst avser mobilapplikationen UniMe Identity Wallet och inkluderar alla funktioner, innehåll eller tjänster som tillhandahålls inom den. Användardata avser alla data, inloggningsuppgifter eller information som du lagrar eller delar med hjälp av Tjänsten. Termen Enhet avser den mobiltelefon eller hårdvara på vilken UniMe-appen är installerad och används.',
          },
          USER_RESPONSIBILITIES: {
            TITLE: 'Användarens ansvar',
            DESCRIPTION:
              'Du är ansvarig för att upprätthålla sekretessen och säkerheten för din enhet och alla inloggningsuppgifter som lagras i Tjänsten, samt för att säkerställa att all information som du lagrar eller delar med hjälp av Tjänsten är korrekt och uppdaterad. Du samtycker till att använda Tjänsten i enlighet med alla tillämpliga lagar och förordningar. Impierce Technologies B.V. ansvarar inte för dataförlust på grund av förlust av enhet eller obehörig åtkomst.',
          },
          DATA_OWNERSHIP: {
            TITLE: 'Dataägande och integritet',
            DESCRIPTION:
              'UniMe bygger på grundprinciperna om användarkontroll och dataintegritet. Vår arkitektur är utformad så att vi inte kan komma åt, samla in, lagra eller behandla de användardata som du hanterar i appen. Alla dina användardata lagras lokalt och säkert på din enhet. Vi använder robusta säkerhetsåtgärder inom applikationen för att skydda dina data. Det ultimata skyddet av dina data beror dock också på att du upprätthåller den övergripande säkerheten för din personliga enhet, till exempel genom att använda ett starkt lösenord och inte installera programvara från opålitliga källor. För att upprätthålla integriteten hos dina inloggningsuppgifter kan UniMe regelbundet kontrollera deras giltighet. Detta görs genom att kontakta utfärdaren av inloggningsuppgifterna direkt från din enhet för att bekräfta om en inloggningsuppgift har återkallats av utfärdaren. Denna process sker automatiskt på din enhet och involverar inte Impierce Technologies B.V. När det är möjligt prioriterar UniMe att använda decentraliserade metoder för dessa kontroller.',
          },
          DATA_VISIBILITY: {
            TITLE: 'Datavisning efter appplattformar',
            DESCRIPTION:
              'När du laddar ner eller använder UniMe via en officiell appbutik, såsom Apple App Store eller Google Play, kan plattformen samla in begränsad teknisk information och användningsdata. Detta kan inkludera uppgifter som din enhetstyp, appinstallationer eller krascher samt region- eller språkinställningar för din enhet. Denna information samlas in enligt appbutikens egna policyer. Impierce samlar inte in någon ytterligare användningsdata från själva enheten.',
          },
          INTELLECTUAL_PROPERTY_RIGHTS: {
            TITLE: 'Immateriella rättigheter',
            DESCRIPTION:
              'Alla immateriella rättigheter till UniMe och relaterat material ägs av Impierce Technologies B.V. eller våra licensgivare. UniMe tillhandahålls under Apache 2.0-licensen, vilket innebär att du kan använda, modifiera och distribuera programvaran så länge du följer villkoren i den licensen.  För mer information, se den fullständiga Apache 2.0-licensen.',
          },
          PROHIBITED_ACTIVITIES: {
            TITLE: 'Förbjudna aktiviteter',
            DESCRIPTION:
              'Du samtycker till att inte försöka få obehörig åtkomst till Tjänsten eller några relaterade system, störa eller påverka Tjänstens prestanda eller säkerhet, eller använda Tjänsten för olagliga, skadliga eller bedrägliga ändamål.',
          },
          THIRD_PARTY_SERVICES: {
            TITLE: 'Tredjepartstjänster',
            DESCRIPTION:
              'Vår tjänst kan länka till eller integreras med tredjepartsplattformar. Dessa plattformar kontrolleras eller drivs inte av oss, och vi stöder inte eller tar ansvar för deras innehåll, åtgärder eller datapraxis. Alla tjänster eller interaktioner som du väljer att delta i med tredjepartsleverantörer sker på egen risk och omfattas av deras egna villkor och policyer. Vi rekommenderar att du läser igenom dessa noggrant innan du fortsätter. Tjänsten tillhandahålls ”i befintligt skick” och ”i mån av tillgänglighet” utan några garantier av något slag. Vi garanterar inte oavbruten, felfri användning av Tjänsten.',
          },
          LIABILITY: {
            TITLE: 'Ansvarsbegränsning',
            DESCRIPTION:
              'I den utsträckning som lagen tillåter är Impierce Technologies B.V. inte ansvarigt för några indirekta, tillfälliga, särskilda eller följdskador. Detta inkluderar, men är inte begränsat till, förlust av data, vinster, affärsmöjligheter eller goodwill till följd av din användning av eller oförmåga att använda Tjänsten; beteende, innehåll eller fel från tredje part; tjänster, innehåll eller åtgärder från tredjepartsplattformar som nås via Tjänsten; eller innehåll som erhållits från eller via Tjänsten.',
          },
          INDEMNIFICATION: {
            TITLE: 'Skadeersättning',
            DESCRIPTION:
              'Du samtycker till att ersätta och hålla Impierce Technologies B.V. och dess dotterbolag skadeslösa från alla anspråk, förluster eller utgifter som uppstår till följd av din användning av Tjänsten, ditt brott mot dessa Användarvillkor eller ditt brott mot någon annan persons eller enhets rättigheter.',
          },
          MODIFICATIONS: {
            TITLE: 'Ändringar av användarvillkoren',
            DESCRIPTION:
              'Vi kan uppdatera dessa användarvillkor från tid till annan. Datumet för ”Senast uppdaterad” ovan avser den senaste versionen. Väsentliga ändringar kommer att meddelas via appen eller vår webbplats. Fortsatt användning av tjänsten innebär att du accepterar de reviderade användarvillkoren.',
          },
          LAW_AND_JURISDIFICATION: {
            TITLE: 'Tillämplig och jurisdiktion',
            DESCRIPTION:
              'Dessa användarvillkor regleras av nederländsk lag. Eventuella tvister ska lösas i nederländsk domstol.',
          },
          SEVERABILITY: {
            TITLE: 'Avskiljbarhet',
            DESCRIPTION: 'Om någon del av dessa användarvillkor befinns ogiltig, förblir resten i full kraft.',
          },
          LANGUAGE: {
            TITLE: 'Språk',
            DESCRIPTION:
              'Dessa användarvillkor finns tillgängliga på flera språk för din bekvämlighet. Vid eventuella konflikter gäller den engelska versionen.',
          },
          ENTIRE_AGREEMENT: {
            TITLE: 'Hela avtalet',
            DESCRIPTION:
              'Dessa användarvillkor utgör tillsammans med vår integritetspolicy det fullständiga avtalet mellan dig och Impierce Technologies B.V. avseende användningen av UniMe.',
          },
          CONTACT: {
            TITLE: 'Kontakta oss',
            DESCRIPTION:
              'Har du feedback eller frågor? Vi strävar alltid efter att förbättra oss. Om något är oklart eller skulle kunna sägas bättre, tveka inte att kontakta oss på contact@impierce.com. Genom att använda UniMe bekräftar du att du har läst, förstått och godkänner dessa användarvillkor. Vi fortsätter att sträva efter att leverera säkra, integritetsfokuserade digitala verktyg som ger dig möjlighet att ta kontroll.',
          },
        },
      },
      OWNERSHIP: {
        TITLE: 'Dataägande',
        DESCRIPTION: 'Jag förstår att jag själv ansvarar för mina data.',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Anpassning',
      NAME: {
        TITLE_1: 'Nu kör vi! Välj ett',
        TITLE_2: 'profilnamn',
        SUBTITLE: 'Din profilinformation lämnar aldrig din enhet.',
        INPUT_PLACEHOLDER: 'Ange ett profilnamn',
      },
      PICTURE: {
        TITLE_1: 'Ställ in en',
        TITLE_2: 'profilbild',
        SUBTITLE: 'Gör den personlig.',
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: 'Lösenord',
      TITLE_1: 'Skapa ditt nya',
      TITLE_2: 'lösenord',
      SUBTITLE: 'Skapa ett starkt lösenord för att kryptera dina data.',
      INPUT_PLACEHOLDER: 'Ange ett lösenord',
      CONFIRM: {
        NAVBAR_TITLE: 'Bekräfta lösenord',
        TITLE_1: 'Bekräfta ditt nya',
        TITLE_2: 'lösenord',
        SUBTITLE: 'Säkerställ att du skrev rätt.',
        INPUT_PLACEHOLDER: 'Skriv lösenordet igen',
        MATCH: 'Lösenordet stämmer',
        NO_MATCH: 'Lösenordet stämmer inte',
      },
      BIOMETRICS: {
        TITLE: 'Aktivera {type}',
        DESCRIPTION: 'Vill du ställa in {type} för att låsa upp appen?',
        CONFIRM: 'Ja, använd {type}',
        DECIDE_LATER: 'Bestäm senare',
      },
      COMPLETED: {
        NAVBAR_TITLE: 'Lösenord skapat',
        TITLE_1: 'Din UniMe-profil är nu',
        TITLE_2: 'skyddad',
        MESSAGE_1: 'Säker & trygg.',
        MESSAGE_2: 'Snyggt jobbat',
      },
    },
  },
  SETTINGS: {
    NAVBAR_TITLE: 'Inställningar',
    PROFILE: {
      TITLE: 'Min profil',
      PROFILE_NAME: {
        TITLE: 'Profilnamn',
        NAVBAR_TITLE: 'Ändra profilnamn',
        INPUT_PLACEHOLDER: 'Ange ett profilnamn',
        CONFIRM: 'Uppdatera',
      },
      DISPLAY_PICTURE: {
        EDIT: 'Redigera',
        CHANGE: 'Välj en profilbild',
        REMOVE: 'Ta bort',
      },
      DELETE_PROFILE: {
        TITLE: 'Ta bort profil',
      },
    },
    APP: {
      TITLE: 'Appinställningar',
      NAVBAR_TITLE: 'Appinställningar',
      LANGUAGE: {
        TITLE: 'Språk',
        NAVBAR_TITLE: 'Välj språk',
        COMING_SOON: 'Kommer snart',
      },
      THEME: {
        LABEL: 'Tema',
        NAVBAR_TITLE: 'Välj tema',
        TITLE_1: 'Välj appens',
        TITLE_2: 'utseende',
        SUBTITLE: 'Är du mer av en nattuggla?',
      },
      SECURITY: {
        LABEL: 'Säkerhet',
        NAVBAR_TITLE: 'Säkerhet',
        SWITCH_LABEL: 'Lås upp med {type}',
        BIOMETRIC_TYPE: {
          ANDROID: {
            FACE_ID: 'ansiktsigenkänning',
            TOUCH_ID: 'fingeravtryck',
          },
          IOS: {
            FACE_ID: 'Face ID',
            TOUCH_ID: 'Touch ID',
          },
          GENERIC: 'biometri',
        },
        ENABLE: {
          DIALOG_TITLE: 'Aktivera {type}',
          DIALOG_CONTENT: 'Ange ditt lösenord för att aktivera {type}.',
        },
        DISABLE: {
          DIALOG_TITLE: 'Inaktivera {type}',
          DIALOG_CONTENT: 'Ange ditt lösenord för att inaktivera {type}.',
        },
      },
      PASSWORD: {
        TITLE: 'Lösenord',
      },
      ONBOARDING_JOURNEY: {
        TITLE: 'Onboardingresa',
        BUTTON_TEXT: 'Starta om',
      },
      HINTS_AND_TIPS: {
        TITLE: 'Tips',
        BUTTON_TEXT: 'Återställ',
      },
      DEVELOPER_MODE: {
        TITLE: 'Utvecklarläge',
      },
    },
    BACKUP_RECOVERY: {
      TITLE: 'Säkerhetskopiering och återställning',
    },
    LOG_OUT: {
      TITLE: 'Logga ut',
    },
    THEME: {
      SYSTEM: 'System',
      LIGHT: 'Ljust',
      DARK: 'Mörkt',
    },
    PASSWORD: {
      POLICY: {
        TITLE: 'Ditt lösenord måste innehålla',
        UPPERCASE_LETTER: 'stor bokstav',
        LOWERCASE_LETTER: 'liten bokstav',
        NUMBER: 'siffra',
        CHARACTERS: 'tecken',
      },
    },
    RESET_APP: {
      TITLE: 'Återställ app',
      DESCRIPTION: 'Är du säker på att du vill återställa appen och ta bort all data?',
      CONFIRM: 'Ja, ta bort allt',
      CANCEL: 'Nej, behåll min profil',
    },
    ACCOUNT: 'Konto',
    SUPPORT: {
      TITLE: 'Support',
      ABOUT: {
        TITLE: 'Om UniMe',
        NAVBAR_TITLE: 'Om UniMe',
        SPECIFICATIONS: 'Specifikationer',
        VERSION: 'Version',
        LICENSE: 'Licens',
        BUILT_WITH: 'Byggd med Tauri',
      },
      FEEDBACK: {
        TITLE: 'Skicka feedback',
      },
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Ange ditt lösenord',
    BUTTON_TEXT: 'Lås upp plånbok',
    FORGOT_PASSWORD: 'Glömt lösenord?',
  },
  ME: {
    BOTTOM_NAVIGATION_TITLE: 'Jag',
    GREETINGS: {
      GREETING_0: 'Hej',
      GREETING_1: 'Hur är läget',
      GREETING_2: 'Hur mår du',
      GREETING_3: 'Välkommen tillbaka',
      GREETING_4: 'Hallå',
    },
    EMPTY_CREDENTIALS: {
      TITLE: 'Lite tomt här',
      SUBTITLE: 'Varför inte få viss data verifierad för att komma igång?',
    },
    FAVORITES: 'Mina favoriter',
    MY_DATA: 'Mina uppgifter',
  },
  ACTIVITY: {
    BOTTOM_NAVIGATION_TITLE: 'Aktivitet',
    NAVBAR_TITLE: 'Anslutna',
    TABS: {
      CONNECTIONS: 'Anslutningar',
      HISTORY: 'Historik',
    },
  },
  SCAN: {
    BOTTOM_NAVIGATION_TITLE: 'Skanna',
    TITLE_1: 'Skanna en',
    TITLE_2: 'QR-kod',
    SUBTITLE: 'För in en QR-kod i bilden för att starta en interaktion.',
    PERMISSION_DENIED: 'Ingen behörighet till kameran',
    OPEN_SETTINGS: 'Öppna inställningar',
    CREDENTIAL_OFFER: {
      NAVBAR_TITLE: 'Intygsserbjudande',
      DESCRIPTION: 'erbjuder dig följande intyg',
      ACCEPT: 'Acceptera intyg',
    },
    CONNECTION_REQUEST: {
      NAVBAR_TITLE: 'Anslutningsförfrågan',
      TITLE: 'Ny anslutning',
      DESCRIPTION: 'Acceptera bara anslutningar du känner igen och litar på',
      KNOWN_CONNECTION: 'Känd anslutning',
      FIRST_INTERACTION: 'Första interaktionen: {duration}',
      LAST_INTERACTION: 'Senaste interaktionen: {date}',
      INTERACTIONS: 'Interaktioner',
      SHARED_DATA: 'Delade data',
      RECEIVED_DATA: 'Mottagna data',
      ACCEPT: 'Acceptera anslutning',
      CERTIFICATIONS: 'Certifieringar',
      CERTIFICATION: 'Certifiering',
      CERTIFICATION_COUNT: '{count} {{count:Certifiering|Certifieringar}}',
      SHOW_MORE: 'Visa mer',
      SHOW_LESS: 'Visa mindre',
    },
    SHARE_CREDENTIALS: {
      NAVBAR_TITLE: 'Dela data',
      DESCRIPTION: 'begär följande intyg',
      REQUESTED: 'Begärt',
      APPROVE: 'Godkänn förfrågan',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Översikt',
      DATA: 'Data',
      ACTIVITY: 'Aktivitet',
    },
    SUMMARY: {
      EMPTY: 'Inga anslutningar ännu.',
      TITLE: 'Ansluten till',
      FIRST_CONNECTED: 'Först ansluten',
      LAST_CONNECTED: 'Senast ansluten',
    },
    DATA: {
      EMPTY: 'Ingen data ännu.',
    },
  },
  HISTORY: {
    EMPTY: 'Ingen aktivitet ännu.',
    DATA_RECEIVED: 'Mottog data från',
    DATA_SHARED: 'Delade data med',
    CONNECTION_ADDED: 'Ansluten till',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Sök efter något',
    NO_QUERY: {
      TITLE: 'Vad ska vi söka efter?',
      DESCRIPTION: 'Sök efter alla dina intyg och förtroendemärken här.',
    },
    NO_RESULTS: {
      TITLE: 'Inga resultat',
      DESCRIPTION: 'Försök söka efter något annat.',
    },
    RECENT_SEARCHES: 'Senaste sökningar',
  },
  CREDENTIAL: {
    NAVBAR_TITLE: 'Intygsinformation',
    DETAILS: {
      VALID: 'Giltig',
      INVALID: 'Ogiltig',
      UNVERIFIED: 'Overifierad',
      ISSUED_BY: 'Utfärdad av',
      SELF_SIGNED: 'dig själv',
      DESCRIPTION: 'Beskrivning',
      OPEN_BADGES: {
        RECIPIENT: 'Mottagare',
        RESULT: 'Resultat',
        ALIGNMENT: 'Justering',
        SKILL: 'Färdighet',
        OCCUPATION: 'Yrke',
        FRAMEWORK_LINK: 'Visa definition',
        CRITERIA: 'Kriterier',
        VALUE: 'Betyg',
      },
    },
    RENDERER: {
      PID: {
        NAME: 'Namn',
        NATIONALITY: 'Nationalitet',
        BIRTH_DATE: 'Födelsedatum',
        BIRTH_PLACE: 'Födelseort',
      },
    },
    ACTIONS: {
      EDIT: {
        MENU_BUTTON: 'Uppdatera visningsnamn',
        CONFIRM_BUTTON: 'Uppdatera visningsnamn',
      },
      DELETE: {
        MENU_BUTTON: 'Ta bort intyg',
        TITLE: 'Ta bort intyg',
        DESCRIPTION: 'Är du säker på att du vill ta bort detta intyg från din plånbok? Detta kan inte ångras.',
        CONFIRM_BUTTON: 'Ta bort',
      },
    },
  },
  ADD_CREDENTIALS: {
    BUTTON: 'Lägg till',
    NAVBAR_TITLE: 'Lägg till intyg',
    VALUE_REQUIRED: 'Obligatoriskt',
    LABEL_DISCLAIMER: 'Syns bara för dig',
    FAVORITES_TOGGLE_LABEL: 'Lägg till i favoriter',
    ADDRESS: {
      TITLE: 'Adress',
      DESCRIPTION: 'Lägg till en arbets- eller privatadress',
      INFO: {
        NAVBAR_TITLE: 'Lägg till adress',
        TITLE: 'Adress',
        DESCRIPTION: 'Innan du börjar',
        ITEM_0: {
          TITLE: 'Alla dina adresser på ett ställe',
          DESCRIPTION:
            'Förvara ditt hem, kontor eller andra adresser säkert i din plånbok, redo att användas direkt på stödda onlineplattformar.',
        },
        ITEM_1: {
          TITLE: 'Tryck, bekräfta och klart',
          DESCRIPTION:
            'När en tjänst ber om en adress, skanna deras kod eller tryck på ”Anslut”, välj vilken adress du vill dela och bekräfta. Klart på ett ögonblick.',
        },
        ITEM_2: {
          TITLE: 'Låst och säkert',
          DESCRIPTION:
            'Dina adresser är bara för dina ögon tills du väljer att dela dem. Varje delning är krypterad, loggad och kräver ditt godkännande, vilket ger dig full överblick över vem som har dina uppgifter.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Lägg till adress',
        LABEL: 'Vad vill du kalla den här adressen?',
        LABEL_PLACEHOLDER: 'Privatadress',
        RESIDENT_ADDRESS_LABEL: 'Adress',
        RESIDENT_COUNTRY_LABEL: 'Land',
        RESIDENT_COUNTRY_PLACEHOLDER: 'Välj ett land',
        RESIDENT_COUNTRY_NO_MATCH: 'Inga länder hittades',
        RESIDENT_STATE_LABEL: 'Delstat',
        RESIDENT_STREET_LABEL: 'Gata',
        RESIDENT_HOUSE_NUMBER_LABEL: 'Husnummer',
        RESIDENT_POSTAL_CODE_LABEL: 'Postnummer',
        RESIDENT_CITY_LABEL: 'Stad',
        CREATE_BUTTON: 'Lägg till adress',
      },
    },
    EMAIL: {
      TITLE: 'E-post',
      DESCRIPTION: 'Verifiera din privata eller arbetsadress',
      INFO: {
        NAVBAR_TITLE: 'Verifierad e-post',
        TITLE: 'Verifierad e-post',
        DESCRIPTION: 'Innan du börjar',
        ITEM_0: {
          TITLE: 'Bevisa att det verkligen är du',
          DESCRIPTION:
            'En verifierad e-postadress fungerar som ditt digitala handslag. Det ger vänner, kollegor och tjänster förtroendet att det verkligen är du, vilket stärker tilliten i dina onlineinteraktioner.',
        },
        ITEM_1: {
          TITLE: 'Du har alltid kontrollen',
          DESCRIPTION:
            'Du bestämmer själv exakt när och var du använder din verifierade e-post. Varje delning är krypterad och loggas i din aktivitets­historik.',
        },
        ITEM_2: {
          TITLE: 'Säker och privat som standard',
          DESCRIPTION:
            'Vi skickar en engångskod till din e-post. Du anger den, och den säkra autentiseringen förblir låst på din telefon och delas aldrig utan ditt uttryckliga godkännande.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Skaffa verifierad e-post',
        LABEL: 'Vad vill du kalla denna adress?',
        LABEL_PLACEHOLDER: 'Privat e-post',
        VALUE_LABEL: 'E-post',
        VALUE_PLACEHOLDER: 'fornamn.efternamn@example.com',
        VALUE_PATTERN_ERROR: 'Ange en giltig e-postadress',
        BUTTON_SEND: 'Skicka verifieringsmail',
        BUTTON_SEND_AGAIN: 'Skicka igen',
        CHECK_EMAIL: 'Kontrollera din inkorg och ange koden nedan.',
        EXPIRED_ERROR: 'Verifieringskoden har gått ut',
      },
    },
    PROFILE: {
      TITLE: 'Profil',
      DESCRIPTION: 'Lägg till information om dig själv',
      INFO: {
        NAVBAR_TITLE: 'Din profil',
        TITLE: 'UniMe-profil',
        DESCRIPTION: 'Innan du börjar',
        ITEM_0: {
          TITLE: 'Hoppa över tråkiga formulär',
          DESCRIPTION:
            'Din UniMe-profil fungerar som ett digitalt ID. Använd den för att automatiskt fylla i uppgifter direkt på stödda webbplatser och slipp onödigt krångel i ditt onlineliv.',
        },
        ITEM_1: {
          TITLE: 'Omedelbar delning',
          DESCRIPTION:
            'När dina uppgifter efterfrågas, skanna helt enkelt en QR-kod eller tryck på ”Anslut”, kontrollera exakt vilken information som begärs och godkänn. Klart på några sekunder.',
        },
        ITEM_2: {
          TITLE: 'ntegritet från början',
          DESCRIPTION:
            'Din information förblir låst på din enhet och lagras aldrig i molnet. Varje delning är end-to-end-krypterad. Dina data är alltid dina egna.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Din profil',
        LABEL: 'Vad vill du kalla den här profilen?',
        LABEL_PLACEHOLDER: 'Personlig profil',
        FIRST_NAME_LABEL: 'Förnamn',
        FIRST_NAME_PLACEHOLDER: 'Ditt förnamn',
        MIDDLE_NAME_LABEL: 'Mellannamn',
        MIDDLE_NAME_PLACEHOLDER: 'Ytterligare förnamn',
        LAST_NAME_LABEL: 'Efternamn',
        LAST_NAME_PLACEHOLDER: 'Ditt efternamn',
        BIRTH_DATE_LABEL: 'Födelsedatum',
        BIRTH_DATE_PLACEHOLDER: 'Ditt födelsedatum',
        BIRTH_PLACE_LABEL: 'Födelseort',
        BIRTH_PLACE_PLACEHOLDER: 'Din födelseort',
        NATIONALITY_LABEL: 'Medborgarskap',
        CREATE_BUTTON: 'Skapa profil',
      },
    },
  },
  SORT: {
    TITLE: 'Sortering',
    PREFERENCES: {
      LIST_VIEW: 'Listvy',
      GRID_VIEW: 'Rutnätsvy',
      ALPHABETICAL: 'Alfabetisk',
      DATE_ISSUED: 'Utfärdandedatum',
      DATE_ADDED: 'Tillagt datum',
    },
    ORDER: {
      A_Z: 'A till Ö',
      Z_A: 'Ö till A',
      NEWEST: 'Nyast först',
      OLDEST: 'Äldst först',
    },
  },
  DOMAIN_LINKAGE: {
    PILL_VERIFIED: 'Verifierad domän',
    PILL_UNTRUSTED: 'Ej betrodd domän',
    PILL_UNVERIFIED: 'Overifierad domän',
  },
  ERROR: {
    TITLE: 'Hoppsan!',
    DEFAULT_MESSAGE: 'Något gick fel. Försök igen.',
  },
  CANCEL: 'Avbryt',
  CLOSE: 'Stäng',
  DISCARD: 'Kasta',
  CONTINUE: 'Fortsätt',
  SKIP: 'Hoppa över',
  ACCEPT: 'Acceptera',
  REJECT: 'Avvisa',
  GETTING_STARTED: {
    SKIP_TITLE: 'Hoppa över guiden',
    SKIP_TEXT: 'Är du säker på att du vill hoppa över guiden?',
  },
} satisfies Translation;

export default sv_FI;
