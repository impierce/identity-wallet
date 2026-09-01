import type { Translation } from '../i18n-types';

const es_ES = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Bienvenidos a',
      WHAT_IS_UNIME_1: 'UniMe conecta tu mundo digital, de forma segura y protegida.',
      WHAT_IS_UNIME_2: 'Para empezar, crea un perfil de identidad completamente nuevo.',
      CREATE_NEW_PROFILE: 'Crea un perfil nuevo',
      SELECT_LANGUAGE: 'Selecciona el idioma',
    },
    PLEDGE: {
      NAVBAR_TITLE: 'El compromiso UniMe',
      TITLE_1: 'Nada de',
      TITLE_2: 'cosas raras',
      SUBTITLE: 'Este es nuestro compromiso contigo.',
      ITEM_1: {
        TITLE: 'No compartiremos tus datos',
        DESCRIPTION:
          'Tus datos son tuyos y solo tú decides con quién los compartes. Punto. De hecho, tus datos ni siquiera entran en nuestros sistemas, a menos que selecciones una de las opciones de almacenaje en la nube.',
      },
      ITEM_2: {
        TITLE: 'No añadiremos rastreadores',
        DESCRIPTION:
          'No rastreamos tu actividad en segundo plano. Punto. Ni para hacer pruebas ni por ningún otro motivo. Este es nuestro compromiso. Tampoco capturamos ninguna información anónima del dispositivo ni estadísticas de uso. Esta decisión hace que desarrollar la aplicación nos resulte algo más difícil, pero creemos que es la decisión correcta.',
      },
      ITEM_3: {
        TITLE: 'Tu información es tuya',
        DESCRIPTION: 'Creemos que ya es hora de que vuelvas a ser el dueño de tu información personal.',
      },
    },
    TERMS: {
      NAVBAR_TITLE: 'Términos y condiciones',
      TITLE_1: 'Aquí está lo menos interesante',
      TITLE_2: 'cosas',
      SUBTITLE: 'Sí, ya lo sabemos. Sin embargo, recomendamos que leas esta información con atención.',
      T_AND_C: {
        TITLE: 'Términos y condiciones',
        DESCRIPTION: 'He leído y estoy de acuerdo con los términos y condiciones.',
        DIALOG_TITLE: 'Términos de uso de UniMe',
        LAST_UPDATED: 'Última actualización: 10 de septiembre de 2025',
        TL_DR:
          'Como cualquier proveedor de aplicaciones, estamos legalmente obligados a ofrecerte algunos términos y condiciones importantes. Hemos hecho nuestro mejor esfuerzo para mantener todo claro y directo. Aquí está la versión corta: UniMe te da el control. No recopila tus datos, respeta tu privacidad y te permite decidir qué información almacenar y compartir. Tu identidad y tus datos permanecen contigo, al igual que la responsabilidad de gestionarlos. No almacenamos tus datos ni podemos acceder a ellos, por lo que cualquier pérdida de datos causada por la pérdida de tu dispositivo o el acceso a la app también es tu responsabilidad. A gran poder, gran responsabilidad. Aquí está todo lo que debes saber:',
        // Note: this translation had not been verified by a legal expert.
        FULL: {
          AGREEMENT: {
            TITLE: 'Aceptación de los Términos',
            DESCRIPTION:
              'Gracias por elegir UniMe, una Billetera de Identidad Digital de la UE (aplicación móvil) desarrollada y mantenida por Impierce Technologies B.V. ("nosotros"). Estos Términos de Uso ("Términos") regulan tu acceso y uso de la aplicación móvil UniMe (el "Servicio"), ya sea descargada desde el Apple App Store, Google Play Store u otra plataforma. Al instalar, acceder o usar el Servicio, reconoces que has leído, entendido y aceptado quedar vinculado por estos Términos y nuestra Política de Privacidad. Si no estás de acuerdo con alguna parte de estos Términos, por favor no uses el Servicio. Si usas el Servicio en nombre de una organización, declaras y garantizas que estás autorizado a aceptar estos Términos en nombre de dicha organización.',
          },
          DEFINITIONS: {
            TITLE: 'Definiciones',
            DESCRIPTION:
              'El término Servicio se refiere a la aplicación móvil UniMe Identity Wallet e incluye todas las funciones, contenidos o servicios proporcionados dentro de ella. Datos del Usuario significa cualquier dato, credencial o información que almacenes o compartas usando el Servicio. El término Dispositivo se refiere al teléfono móvil o hardware en el que la app UniMe está instalada y se utiliza.',
          },
          USER_RESPONSIBILITIES: {
            TITLE: 'Responsabilidades del Usuario',
            DESCRIPTION:
              'Eres responsable de mantener la confidencialidad y seguridad de tu dispositivo y de cualquier credencial almacenada dentro del Servicio, así como de asegurar que cualquier información que almacenes o compartas usando el Servicio sea precisa y esté actualizada. Aceptas usar el Servicio cumpliendo con todas las leyes y regulaciones aplicables. Impierce Technologies B.V. no es responsable por la pérdida de datos debido a la pérdida de tu dispositivo o acceso no autorizado.',
          },
          DATA_OWNERSHIP: {
            TITLE: 'Propiedad de los Datos y Privacidad',
            DESCRIPTION:
              ' UniMe se basa en los principios fundamentales de control del usuario y privacidad de datos. Nuestra arquitectura está diseñada para que no podamos acceder, recopilar, almacenar o procesar los Datos del Usuario que gestionas dentro de la app. Todos tus Datos del Usuario se almacenan local y de manera segura en tu Dispositivo. Empleamos medidas de seguridad robustas dentro de la aplicación para proteger tus datos. Sin embargo, la protección final de tus datos también depende de ti, manteniendo la seguridad general de tu Dispositivo personal, por ejemplo, usando un código fuerte y evitando instalar software de fuentes no confiables. Para mantener la integridad de tus credenciales, UniMe puede verificar periódicamente su validez. Esto se realiza contactando directamente al emisor de la credencial desde tu Dispositivo para confirmar si la credencial ha sido revocada. Este proceso ocurre automáticamente en tu Dispositivo y no involucra a Impierce Technologies B.V. Cuando es posible, UniMe prioriza métodos descentralizados para estas verificaciones.',
          },
          DATA_VISIBILITY: {
            TITLE: 'Visibilidad de Datos por Plataformas de Apps',
            DESCRIPTION:
              'Cuando descargas o usas UniMe a través de un App Store oficial, como Apple App Store o Google Play, la plataforma puede recopilar datos técnicos y de uso limitados. Esto puede incluir detalles como el tipo de dispositivo, eventos de instalación o fallos de la app, y la configuración de región o idioma de tu dispositivo. Esta información se recopila según las políticas del App Store correspondiente. Impierce no recopila datos adicionales de uso directamente desde tu dispositivo.',
          },
          INTELLECTUAL_PROPERTY_RIGHTS: {
            TITLE: 'Derechos de Propiedad Intelectual',
            DESCRIPTION:
              'Todos los derechos de propiedad intelectual de UniMe y materiales relacionados pertenecen a Impierce Technologies B.V. o a nuestros licenciantes. UniMe se proporciona bajo la licencia Apache 2.0, lo que significa que puedes usar, modificar y distribuir el software siempre que cumplas con los términos de esa licencia. Para más información, consulta la licencia completa Apache 2.0.',
          },
          PROHIBITED_ACTIVITIES: {
            TITLE: 'Actividades Prohibidas',
            DESCRIPTION:
              'Aceptas no intentar obtener acceso no autorizado al Servicio o a sistemas relacionados, no interrumpir ni interferir con el rendimiento o seguridad del Servicio, ni usar el Servicio para fines ilegales, dañinos o fraudulentos.',
          },
          THIRD_PARTY_SERVICES: {
            TITLE: 'Servicios de Terceros',
            DESCRIPTION:
              'Nuestro Servicio puede vincularse o integrarse con plataformas de terceros. Estas plataformas no están controladas ni operadas por nosotros, y no respaldamos ni asumimos responsabilidad por su contenido, acciones o prácticas de datos. Cualquier servicio o interacción que decidas realizar con proveedores externos es bajo tu propio riesgo y está sujeto a sus propios términos y políticas. Te recomendamos revisarlas cuidadosamente antes de continuar. El Servicio se proporciona “tal como está” y “según disponibilidad” sin ningún tipo de garantía. No garantizamos un uso ininterrumpido o libre de errores del Servicio.',
          },
          LIABILITY: {
            TITLE: 'Limitación de Responsabilidad',
            DESCRIPTION:
              'En la máxima medida permitida por la ley, Impierce Technologies B.V. no es responsable de daños indirectos, incidentales, especiales o consecuentes. Esto incluye, entre otros, pérdida de datos, ganancias, oportunidades de negocio o reputación derivados de tu uso o incapacidad de usar el Servicio; conducta, contenido o errores de terceros; servicios, contenidos o acciones de plataformas de terceros accedidas a través del Servicio; o cualquier contenido obtenido de o mediante el Servicio.',
          },
          INDEMNIFICATION: {
            TITLE: 'Indemnización',
            DESCRIPTION:
              'Aceptas indemnizar y mantener indemne a Impierce Technologies B.V. y sus afiliados frente a cualquier reclamo, pérdida o gasto resultante de tu uso del Servicio, tu violación de estos Términos de Uso o la violación de cualquier derecho de otra persona o entidad.',
          },
          MODIFICATIONS: {
            TITLE: 'Modificaciones a los Términos de Uso',
            DESCRIPTION:
              'Podemos actualizar estos Términos de Uso periódicamente. La fecha “Última actualización” arriba refleja la versión más reciente. Los cambios significativos se comunicarán a través de la app o nuestro sitio web. El uso continuado del Servicio significa que aceptas los Términos de Uso revisados.',
          },
          LAW_AND_JURISDIFICATION: {
            TITLE: 'Ley Aplicable y Jurisdicción',
            DESCRIPTION:
              'Estos Términos de Uso se rigen por la ley neerlandesa. Cualquier disputa se resolverá en los tribunales de los Países Bajos.',
          },
          SEVERABILITY: {
            TITLE: 'Divisibilidad',
            DESCRIPTION:
              'Si alguna parte de estos Términos de Uso se considera inválida, el resto seguirá en pleno vigor.',
          },
          LANGUAGE: {
            TITLE: 'Idioma',
            DESCRIPTION:
              'Estos Términos de Uso están disponibles en varios idiomas para conveniencia. En caso de conflicto, prevalece la versión en inglés.',
          },
          ENTIRE_AGREEMENT: {
            TITLE: 'Acuerdo Completo',
            DESCRIPTION:
              'Estos Términos de Uso, junto con nuestra Política de Privacidad, constituyen el acuerdo completo entre tú e Impierce Technologies B.V. con respecto al uso de UniMe.',
          },
          CONTACT: {
            TITLE: 'Contáctanos',
            DESCRIPTION:
              '¿Tienes comentarios o preguntas? Siempre buscamos mejorar. Si algo no está claro o podría expresarse mejor, no dudes en comunicarte a contact@impierce.com. Al usar UniMe, confirmas que has leído, comprendido y aceptado estos Términos de Uso. Seguimos comprometidos a ofrecer herramientas digitales seguras y centradas en la privacidad que te empoderen.',
          },
        },
      },
      OWNERSHIP: {
        TITLE: 'Propiedad de los datos',
        DESCRIPTION: 'Entiendo que soy el único responsable de mis datos.',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Personalización',
      NAME: {
        TITLE_1: '¡Adelante! Escoge un',
        TITLE_2: 'nombre de perfil',
        SUBTITLE: 'La información de tu perfil nunca saldrá de tu dispositivo.',
        INPUT_PLACEHOLDER: 'Introduce un nombre de perfil',
      },
      PICTURE: {
        TITLE_1: 'Establece una imagen',
        TITLE_2: 'foto',
        SUBTITLE: 'Personalízala.',
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: 'Contraseña',
      TITLE_1: 'Establece tu nueva',
      TITLE_2: 'contraseña',
      SUBTITLE: 'Debes escoger una contraseña fuerte para encriptar tus datos de manera segura.',
      INPUT_PLACEHOLDER: 'Introduce una contraseña',
      CONFIRM: {
        NAVBAR_TITLE: 'Confirma la contraseña',
        TITLE_1: 'Confirma tu nueva',
        TITLE_2: 'contraseña',
        SUBTITLE: 'Tienes que confirmar tu contraseña para asegurarte de que la has escrito correctamente.',
        INPUT_PLACEHOLDER: 'Vuelve a escribir tu contraseña',
        MATCH: 'Las contraseñas coinciden',
        NO_MATCH: 'Las contraseñas no coinciden',
      },
      BIOMETRICS: {
        TITLE: 'Activar {type}',
        DESCRIPTION: '¿Quieres configurar {type} para desbloquear la app?',
        CONFIRM: 'Sí, usar {type}',
        DECIDE_LATER: 'Decide más tarde',
      },
      COMPLETED: {
        NAVBAR_TITLE: 'Contraseña establecida',
        TITLE_1: 'Tu perfil UniMe ya está',
        TITLE_2: 'protegido',
        MESSAGE_1: 'Seguro y protegido.',
        MESSAGE_2: 'Buen trabajob',
      },
    },
  },
  SETTINGS: {
    NAVBAR_TITLE: 'Ajustes',
    PROFILE: {
      TITLE: 'Mi perfil',
      PROFILE_NAME: {
        TITLE: 'Nombre del perfil',
        NAVBAR_TITLE: 'Cambia el nombre del perfil',
        INPUT_PLACEHOLDER: 'Ingresa un nombre de perfil',
        CONFIRM: 'Actualiza',
      },
      DISPLAY_PICTURE: {
        EDIT: 'Edita',
        CHANGE: 'Selecciona una foto del perfil',
        REMOVE: 'Borra',
      },
      DELETE_PROFILE: {
        TITLE: 'Borra el perfil',
      },
    },
    APP: {
      TITLE: 'Configuración de la aplicación',
      NAVBAR_TITLE: 'Configuración de la aplicación',
      LANGUAGE: {
        TITLE: 'Idioma',
        NAVBAR_TITLE: 'Seleccionar idioma',
        COMING_SOON: 'Próximamente',
      },
      THEME: {
        LABEL: 'Tema',
        NAVBAR_TITLE: 'Seleccionar tema',
        TITLE_1: 'Escoge el aspecto de tu',
        TITLE_2: 'aplicación',
        SUBTITLE: '¿Te gusta más la noche?',
      },
      SECURITY: {
        LABEL: 'Seguridad',
        NAVBAR_TITLE: 'Seguridad',
        SWITCH_LABEL: 'Desbloquear con {type}',
        BIOMETRIC_TYPE: {
          ANDROID: {
            FACE_ID: 'reconocimiento facial',
            TOUCH_ID: 'huella dactilar',
          },
          IOS: {
            FACE_ID: 'Face ID',
            TOUCH_ID: 'Touch ID',
          },
          GENERIC: 'biometría',
        },
        ENABLE: {
          DIALOG_TITLE: 'Activar {type}',
          DIALOG_CONTENT: 'Introduce tu contraseña para activar {type}',
        },
        DISABLE: {
          DIALOG_TITLE: 'Desactivar {type}',
          DIALOG_CONTENT: 'Introduce tu contraseña para desactivar {type}',
        },
      },
      PASSWORD: {
        TITLE: 'Contraseña',
      },
      ONBOARDING_JOURNEY: {
        TITLE: 'Proceso de incorporación',
        BUTTON_TEXT: 'Reinicia',
      },
      HINTS_AND_TIPS: {
        TITLE: 'Sugerencias y consejos',
        BUTTON_TEXT: 'Restablece',
      },
      DEVELOPER_MODE: {
        TITLE: 'Modo desarrollador',
      },
    },
    BACKUP_RECOVERY: {
      TITLE: 'Copia de seguridad y recuperación',
    },
    LOG_OUT: {
      TITLE: 'Cierra la sesión',
    },
    THEME: {
      SYSTEM: 'Sistema',
      LIGHT: 'Luz',
      DARK: 'Oscuro',
    },
    PASSWORD: {
      POLICY: {
        TITLE: 'Tu contraseña debe contener',
        UPPERCASE_LETTER: 'letra mayúscula',
        LOWERCASE_LETTER: 'letra minúscula',
        NUMBER: 'número',
        CHARACTERS: 'caracteres',
      },
    },
    RESET_APP: {
      TITLE: 'Restablece la aplicación',
      DESCRIPTION: '¿Seguro que quieres restablecer la aplicación y eliminar todos los datos?',
      CONFIRM: 'Sí, elimina todo',
      CANCEL: 'No, conserva mi perfil',
    },
    ACCOUNT: 'Cuenta',
    SUPPORT: {
      TITLE: 'Soporte',
      ABOUT: {
        TITLE: 'Sobre UniMe',
        NAVBAR_TITLE: 'Sobre UniMe',
        SPECIFICATIONS: 'Especificaciones',
        VERSION: 'Versión',
        LICENSE: 'Licencia',
        BUILT_WITH: 'Construida con Tauri',
      },
      FEEDBACK: {
        TITLE: 'Envía feedback',
      },
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Introduce tu contraseña',
    BUTTON_TEXT: 'Desbloquea la billetera',
    FORGOT_PASSWORD: '¿Has olvidado tu contraseña?',
  },
  ME: {
    BOTTOM_NAVIGATION_TITLE: 'Yo',
    GREETINGS: {
      GREETING_0: 'Hola',
      GREETING_1: 'Qué tal',
      GREETING_2: 'Cómo estás',
      GREETING_3: 'Bienvenido de nuevo',
      GREETING_4: 'Hola',
    },
    EMPTY_CREDENTIALS: {
      TITLE: 'Esto está un poco tranquilo',
      SUBTITLE: '¿Qué tal si añades algunas credenciales para empezar tu nuevo yo digital?',
    },
    FAVORITES: 'Mis favoritos',
    MY_DATA: 'Mis datos',
  },
  ACTIVITY: {
    BOTTOM_NAVIGATION_TITLE: 'Actividad',
    NAVBAR_TITLE: 'Conectado',
    TABS: {
      CONNECTIONS: 'Conexiones',
      HISTORY: 'Historial',
    },
  },
  SCAN: {
    BOTTOM_NAVIGATION_TITLE: 'Escanea',
    TITLE_1: 'Escanea un',
    TITLE_2: 'código QR',
    SUBTITLE: 'Coloca un código QR en la pantalla para iniciar una interacción.',
    PERMISSION_DENIED: 'No tenemos permiso para acceder a la cámara',
    OPEN_SETTINGS: 'Abre la pantalla de ajustes',
    CREDENTIAL_OFFER: {
      NAVBAR_TITLE: 'Oferta de credenciales',
      DESCRIPTION: 'te ofrece las siguientes credenciales',
      ACCEPT: 'Acepta las credenciales',
    },
    CONNECTION_REQUEST: {
      NAVBAR_TITLE: 'Solicitud de conexión',
      TITLE: 'Nueva conexión',
      DESCRIPTION: 'Acepta únicamente las nuevas conexiones que reconozcas y en las que confíes',
      KNOWN_CONNECTION: 'Conexión conocida',
      FIRST_INTERACTION: 'Primera interacción: {duration}',
      LAST_INTERACTION: 'Última interacción: {duration}',
      INTERACTIONS: 'Interacciones',
      ACCEPT: 'Acepta la conexión',
      CERTIFICATIONS: 'Certificaciones',
      CERTIFICATION: 'Certificación',
      AND_MORE: '{names}, y {count} más',
      SHOW_MORE: 'Ver más',
      SEE_ALL: 'Ver todas',
      ECOSYSTEMS: 'Ecosistemas',
      ECOSYSTEM_MEMBERS: '{count} {{miembro|miembros}}',
      ECOSYSTEM_MEMBERS_HEADING: 'Miembros',
      ECOSYSTEM_ABOUT: 'Acerca de',
      ECOSYSTEM_OWNER: 'Propietario del ecosistema',
      ECOSYSTEM_COUNT: '{count} {{ecosistema|ecosistemas}}',
    },
    SHARE_CREDENTIALS: {
      NAVBAR_TITLE: 'Comparte datos',
      DESCRIPTION: 'solicita las siguientes credenciales',
      REQUESTED: 'Solicitadas',
      APPROVE: 'Aprueba la petición',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Resumen',
      DATA: 'Datos',
      ACTIVITY: 'Actividad',
    },
    SUMMARY: {
      EMPTY: 'Todavía sin conexiones.',
      TITLE: 'Conectado a',
      FIRST_CONNECTED: 'Primer conectado',
      LAST_CONNECTED: 'Último conectado',
    },
    DATA: {
      EMPTY: 'Todavía sin datos.',
    },
  },
  HISTORY: {
    EMPTY: 'Todavía sin actividad.',
    DATA_RECEIVED: 'Datos recibidos de',
    DATA_SHARED: 'Datos compartidos con',
    CONNECTION_ADDED: 'Conectado a',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Busca algo',
    NO_QUERY: {
      TITLE: '¿Qué quieres que busquemos?',
      DESCRIPTION: 'Busca aquí alguna de tus credenciales e insignias.',
    },
    NO_RESULTS: {
      TITLE: 'No se ha encontrado ningún resultado',
      DESCRIPTION: 'Intenta buscar otra cosa.',
    },
    RECENT_SEARCHES: 'Búsquedas recientes',
  },
  CREDENTIAL: {
    NAVBAR_TITLE: 'Información de las credenciales',
    DETAILS: {
      VALID: 'Válidas',
      INVALID: 'Inválidas',
      UNVERIFIED: 'No verificadas',
      ISSUED_BY: 'Emitidas por',
      SELF_SIGNED: 'Autofirmadas',
      DESCRIPTION: 'Descripción',
      OPEN_BADGES: {
        RECIPIENT: 'Destinatario',
        RESULT: 'Resultado',
        ALIGNMENT: 'Alineación',
        SKILL: 'Habilidad',
        OCCUPATION: 'Ocupación',
        FRAMEWORK_LINK: 'Ver definición',
        CRITERIA: 'Criterio',
        VALUE: 'Nota',
      },
    },
    RENDERER: {
      PID: {
        NAME: 'Nombre',
        NATIONALITY: 'Nacionalidad',
        BIRTH_DATE: 'Fecha de nacimiento',
        BIRTH_PLACE: 'Lugar de nacimiento',
      },
    },
    ACTIONS: {
      EDIT: {
        MENU_BUTTON: 'Editar nombre para mostrar',
        CONFIRM_BUTTON: 'Actualizar nombre para mostrar',
      },
      DELETE: {
        MENU_BUTTON: 'Borra las credenciales',
        TITLE: 'Borra las credenciales',
        DESCRIPTION: '¿Seguro que quieres borrar estas credenciales de tu billetera? Esta acción no puede deshacerse.',
        CONFIRM_BUTTON: 'Borra',
      },
    },
  },
  ADD_CREDENTIALS: {
    BUTTON: 'Añadir',
    NAVBAR_TITLE: 'Añadir datos',
    VALUE_REQUIRED: 'Obligatorio',
    LABEL_DISCLAIMER: 'Esto sólo lo ves tú',
    FAVORITES_TOGGLE_LABEL: 'Añadir a favoritos',
    ADDRESS: {
      TITLE: 'Dirección',
      DESCRIPTION: 'Añade tu dirección de casa o trabajo',
      INFO: {
        NAVBAR_TITLE: 'Añadir dirección',
        TITLE: 'Dirección',
        DESCRIPTION: 'Antes de empezar',
        ITEM_0: {
          TITLE: 'Todas tus direcciones en un solo lugar',
          DESCRIPTION:
            'Guarda tu casa, oficina o cualquier otra dirección de forma segura en tu billetera, lista para usarse al instante en las plataformas en línea compatibles.',
        },
        ITEM_1: {
          TITLE: 'Toca, confirma y listo',
          DESCRIPTION:
            'Cuando un servicio solicite una dirección, simplemente escanea su código o toca “Conectar”, elige la dirección que quieres compartir y confirma. Hecho en un abrir y cerrar de ojos.',
        },
        ITEM_2: {
          TITLE: 'Bloqueado y seguro',
          DESCRIPTION:
            'Tus direcciones son solo para tus ojos hasta que decidas compartirlas. Cada intercambio está cifrado, registrado y requiere tu aprobación, dándote una visión completa de quién tiene tus datos.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Añadir dirección',
        LABEL: '¿Cómo quieres llamar a esta dirección?',
        LABEL_PLACEHOLDER: 'Dirección de casa',
        RESIDENT_ADDRESS_LABEL: 'Dirección',
        RESIDENT_COUNTRY_LABEL: 'País',
        RESIDENT_COUNTRY_PLACEHOLDER: 'Elige un país',
        RESIDENT_COUNTRY_NO_MATCH: 'No se encontraron países',
        RESIDENT_STATE_LABEL: 'Estado',
        RESIDENT_STREET_LABEL: 'Calle',
        RESIDENT_HOUSE_NUMBER_LABEL: 'Número',
        RESIDENT_POSTAL_CODE_LABEL: 'Código postal',
        RESIDENT_CITY_LABEL: 'Ciudad',
        CREATE_BUTTON: 'Añadir dirección',
      },
    },
    EMAIL: {
      TITLE: 'Correo electrónico',
      DESCRIPTION: 'Verifica tu correo personal o de trabajo',
      INFO: {
        NAVBAR_TITLE: 'Correo verificado',
        TITLE: 'Correo verificado',
        DESCRIPTION: 'Antes de empezar',
        ITEM_0: {
          TITLE: 'Demuestra que realmente eres tú',
          DESCRIPTION:
            'Un correo electrónico verificado funciona como tu apretón de manos digital. Da a tus amigos, colegas y servicios la confianza de que eres genuinamente tú, aumentando la confianza en tus interacciones en línea.',
        },
        ITEM_1: {
          TITLE: 'Siempre tienes el control',
          DESCRIPTION:
            'Tú decides exactamente cuándo y dónde usar tu correo electrónico verificado. Cada intercambio está cifrado y registrado en tu historial de actividad.',
        },
        ITEM_2: {
          TITLE: 'Seguro y privado por defecto',
          DESCRIPTION:
            'Te enviamos un código de un solo uso a tu correo electrónico. Lo introduces, y la credencial segura permanece bloqueada en tu teléfono, sin compartirse nunca sin tu aprobación directa.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Verifica tu correo',
        LABEL: '¿Cómo quieres llamar a este correo?',
        LABEL_PLACEHOLDER: 'Correo personal',
        VALUE_LABEL: 'Correo electrónico',
        VALUE_PLACEHOLDER: 'nombre.apellido@example.com',
        VALUE_PATTERN_ERROR: 'Por favor, introduce un correo válido',
        BUTTON_SEND: 'Enviar correo de verificación',
        BUTTON_SEND_AGAIN: 'Reenviar correo de verificación',
        CHECK_EMAIL: 'Por favor, revisa tu bandeja de entrada e introduce el código abajo.',
        EXPIRED_ERROR: 'El código de verificación ha expirado',
      },
    },
    PROFILE: {
      TITLE: 'Perfil',
      DESCRIPTION: 'Añade información sobre ti',
      INFO: {
        NAVBAR_TITLE: 'Tu perfil',
        TITLE: 'Perfil UniMe',
        DESCRIPTION: 'Antes de empezar',
        ITEM_0: {
          TITLE: 'Omite los formularios tediosos',
          DESCRIPTION:
            'Tu perfil de UniMe funciona como una identificación digital. Úsalo para completar automáticamente tus datos al instante en los sitios compatibles y elimina el desorden de tu vida en línea.',
        },
        ITEM_1: {
          TITLE: 'Compartir al instante',
          DESCRIPTION:
            'Cuando se soliciten tus datos, simplemente escanea un código QR o toca “Conectar”, revisa exactamente qué información se está solicitando y apruébala. Todo listo en segundos.',
        },
        ITEM_2: {
          TITLE: 'Privado desde el diseño',
          DESCRIPTION:
            'Tu información permanece bloqueada en tu dispositivo y nunca se almacena en la nube. Cada intercambio está cifrado de extremo a extremo. Tus datos siempre son tuyos.',
        },
      },
      ADD: {
        NAVBAR_TITLE: 'Tu perfil',
        LABEL: '¿Cómo quieres llamar a este perfil?',
        LABEL_PLACEHOLDER: 'Perfil personal',
        FIRST_NAME_LABEL: 'Nombre',
        FIRST_NAME_PLACEHOLDER: 'Tu nombre',
        MIDDLE_NAME_LABEL: 'Segundo nombre(s)',
        MIDDLE_NAME_PLACEHOLDER: 'Tus segundos nombres',
        LAST_NAME_LABEL: 'Apellido',
        LAST_NAME_PLACEHOLDER: 'Tu apellido',
        BIRTH_DATE_LABEL: 'Fecha de nacimiento',
        BIRTH_DATE_PLACEHOLDER: 'Selecciona tu fecha de nacimiento',
        BIRTH_PLACE_LABEL: 'Lugar de nacimiento',
        BIRTH_PLACE_PLACEHOLDER: 'Tu lugar de nacimiento',
        NATIONALITY_LABEL: 'Nacionalidad',
        CREATE_BUTTON: 'Crear perfil',
      },
    },
  },
  SORT: {
    TITLE: 'Clasificación',
    PREFERENCES: {
      LIST_VIEW: 'Vista de lista',
      GRID_VIEW: 'Vista de cuadrícula',
      ALPHABETICAL: 'Alfabético',
      DATE_ISSUED: 'Fecha de emisión',
      DATE_ADDED: 'Fecha añadida',
    },
    ORDER: {
      A_Z: 'A to Z',
      Z_A: 'Z to A',
      NEWEST: 'Primero la más nueva',
      OLDEST: 'Primero la más antigua',
    },
  },
  DOMAIN_LINKAGE: {
    PILL_VERIFIED: 'Dominio verificado',
    PILL_UNTRUSTED: 'Dominio no confiable',
    PILL_UNVERIFIED: 'Dominio sin verificar',
  },
  ERROR: {
    TITLE: '¡Vaya!',
    DEFAULT_MESSAGE: 'Algo salió mal. Vuelve a intentarlo.',
  },
  CANCEL: 'Cancela',
  CLOSE: 'Cierra',
  DISCARD: 'Descarta',
  CONTINUE: 'Continua',
  SKIP: 'Saltar',
  ACCEPT: 'Acepta',
  REJECT: 'Rechaza',
  // TODO: AI generated to fix TS errors in journeys.
  GETTING_STARTED: {
    SKIP_TITLE: 'Salta el paso de empezar',
    SKIP_TEXT: '¿Seguro que quieres saltarte la guía de cómo empezar?',
  },
} satisfies Translation;

export default es_ES;
