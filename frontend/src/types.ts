
export interface ApiToken {
  token: string;
  createdBy: string;
}

export interface Key {
  created_at: number;
  created_by: string | null;
  description: string | null;
  key: string;
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

