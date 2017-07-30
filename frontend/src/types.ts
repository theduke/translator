
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
  name: string;
  parentId: string | null;
}

export interface Translation {
  created_at: number;
  updated_at: number;
  created_by: string | null;
  key: string;
  language: string;
  value: string;
}

export interface NewTranslation {
  language: string;
  key: string;
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

