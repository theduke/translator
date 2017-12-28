
export interface ApiToken {
  token: string;
  createdBy: string;
}

export interface Key {
  id: string;
  created_at: number;
  created_by: string | null;
  description: string | null;
  key: string;
}


/**
 * Compiled regular expression for validating key parts.
 */
const keyValidationRegex = new RegExp('^[a-z]+([a-z\d_\-]*[a-z\d]+)?$');

/**
 * Validate a key name.
 * @returns {string | null} null if valid, an error string if invalid.
 */
export function validateKey(key: string): string | null {
    const valid = key.split('.').every(keyValidationRegex.test, keyValidationRegex);
    return valid ? null : "Invalid key (must start with a letter, must end with letter or number, can only contain letters, numbers, '_' and '-'";
}

export interface Language {
  createdAt: number;
  createdBy: string | null;
  id: string;
  code: string;
  name: string;
  parentId: string | null;
}

export interface Translation {
  id: string;
  created_at: number;
  updated_at: number;
  created_by: string | null;
  keyId: string;
  languageId: string;
  value: string;
}

export interface NewTranslation {
  languageId: string;
  keyId: string;
  value: string;
}

export interface TranslationsResponse {
  key: Key;
  translations: Translation[];
}

export interface BaseData {
  keys: Key[];
  languages: Language[];
}

export interface NewLanguageData {
    id: string,
    name: string,
    parent_id: string | null;
}

