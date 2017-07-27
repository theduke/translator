
export interface Session {
  username: string;
  token: string;
}

export interface Key {
  created_at: number;
  created_by: string | null;
  description: string | null;
  key: string;
}

export interface Language {
  created_at: number;
  created_by: string | null;
  id: string;
  name: string;
  parent_id: string | null;
}

export interface Translation {
  created_at: number;
  updated_at: number;
  created_by: string | null;
  key: string;
  language: string;
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

export interface Login {
  cmd: 'Login',
  data: {
    username: string;
    password: string;
  };
}

export interface CreateUser {
  cmd: 'CreateUser',
  data: {
    username: string;
    role: 'admin' | 'user';
    password: string;
  }
}

export interface UpdateUser {
  cmd: 'UpdateUser',
  data: {
    username: string;
    password: string;
  }
}

export interface DeleteUser {
  cmd: 'DeleteUser',
  data: {
    username: string;
  }
}

export interface NewLanguageData {
    id: string,
    name: string,
    parent_id: string | null;
}

export interface CreateLanguage {
  cmd: 'CreateLanguage';
  data: NewLanguageData;
}

export interface DeleteLanguage {
  cmd: 'DeleteLanguage';
  data: {
    id: string;
  };
}

export interface CreateKey {
  cmd: 'CreateKey';
  data: {
    key: string;
    description: string | null;
  };
}

export interface DeleteKey {
  cmd: 'DeleteKey';
  data: {
    key: string;
  };
}

export interface CreateTranslation {
  cmd: 'CreateTranslation';
  data: {
    lang: string;
    key: string;
    value: string;
  };
}

export interface UpdateTranslation {
  cmd: 'UpdateTranslation';
  data: {
    lang: string;
    key: string;
    value: string;
  };
}

export interface DeleteTranslation {
  cmd: 'DeleteTranslation';
  data: {
    lang: string;
    key: string;
  };
}

export type Command
  = Login
  | CreateUser
  | UpdateUser
  | DeleteUser
  | CreateLanguage
  | DeleteLanguage
  | CreateKey
  | DeleteKey
  | CreateTranslation
  | UpdateTranslation
  | DeleteTranslation;

export interface CommandSuccess {
  data: any;
}

export interface CommandError {
  error: {
    code: string;
  }
}

export type CommandResult = CommandSuccess | CommandError;
