// Because logging of svelte can end up in Tauri when stringified. This function will provide some safety

// Its from a package online, its just the keywords that are often used for sensitive data. Its not a bulletproof solution.
const defaultBadWords = ['password', 'pass', 'token', 'auth', 'secret', 'passphrase', 'card'];

const sanitizeStringifyRecursively = function (object: any, replacer: any, space: any): any {
  if (object === null || object === undefined) {
    return object;
  }

  for (const [key, value] of Object.entries(object)) {
    if (typeof value === 'object') {
      object[key] = sanitizeStringifyRecursively(value, replacer, space);
    } else if (defaultBadWords.indexOf(key.toLowerCase()) !== -1) {
      object[key] = '*****';
    }
  }

  return object;
};

export const sanitizeStringify = function (value: any, replacer?: any, space?: string | number | undefined): string {
  if (typeof value === 'object') {
    let clone = structuredClone(value);
    const result = sanitizeStringifyRecursively(clone, replacer, space);
    return JSON.stringify(result, replacer, space);
  } else {
    return JSON.stringify(value, replacer, space);
  }
};
