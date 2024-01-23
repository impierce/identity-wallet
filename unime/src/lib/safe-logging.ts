// Because logging of svelte can end up in Tauri when stringified. This function will provide some safety

const defaultBadWords = ['password', 'pass', 'token', 'auth', 'secret', 'passphrase', 'card'];

const safeStringifyRecursively = function (object: any, replacer: any, space: any): any {
  for (const [key, value] of Object.entries(object)) {
    if (typeof value === 'object') {
      object[key] = safeStringifyRecursively(value, replacer, space);
    } else if (defaultBadWords.indexOf(key.toLowerCase()) !== -1) {
      object[key] = '*****';
    }
  }

  return object;
};

export const sanitize_stringify = function (value: any, replacer?: any, space?: string | number | undefined): string {
  if (typeof value === 'object') {
    const result = safeStringifyRecursively({ ...value }, replacer, space);
    return JSON.stringify(result, replacer, space);
  } else {
    return JSON.stringify(value, replacer, space);
  }
};
