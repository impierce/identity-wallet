// Because logging of svelte can end up in Tauri when stringified. This function will provide some safety

const defaultBadWords = ['password', 'pass', 'token', 'auth', 'secret', 'passphrase', 'card'];

const replacement = '*****';

const original = JSON.stringify;

const safeStringifyRecursively = function (object: any, replacer: any, space: any): any {
  for (const [key, value] of Object.entries(object)) {
    if (typeof value === 'object') {
      object[key] = safeStringifyRecursively(value, replacer, space);
    } else if (defaultBadWords.indexOf(key.toLowerCase()) !== -1) {
      object[key] = replacement;
    }
  }

  return object;
};

const safeStringify = function (value: any, replacer?: any, space?: string | number | undefined): string {
  if (typeof value === 'object') {
    const result = safeStringifyRecursively({ ...value }, replacer, space);
    return original(result, replacer, space);
  } else {
    return original(value, replacer, space);
  }
};

JSON.stringify = safeStringify;
