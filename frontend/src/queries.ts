import gql from 'graphql-tag';

export const languages = gql`
query {
  languages {
    id
    code
    name
    parentId
    createdAt
    createdBy
  }
}
`;

export const keys = gql`
  query {
    keys {
      id
      key
      description
      createdAt
      createdBy
    }
  }
`;

export const keyWithTranslations = gql`
  query KeyWithWithTranslations($key: String!) {
    key(key: $key) {
      id
      key
      description
      createdAt
      createdBy
      translations {
        id
        languageId
        keyId
        value
        createdAt
        updatedAt
        createdBy
      }
    }
    
    languages {
      id
      code
      name
      parentId
    }
  
  }

`;

export const createKey = gql`
  mutation createKey($key: NewKey!) {
    createKey(key: $key) {
      id
      key
      description
      createdAt
      createdBy
    }
  } 
`;

export const deleteKey = gql`
  mutation deleteKey($key: String!) {
    deleteKey(key: $key)
  } 
`;

export const translations = gql`
  query translations($key: String!) {
    translations(key: $key) {
       language,
       key,
       value,
       created_at,
       updated_at,
       created_by,
    }
  }
`;

export const translate = gql`
  mutation translate($translation: NewTranslation!) {
    translate(translation: $translation) {
       id
       languageId,
       keyId,
       value,
       createdAt,
       updatedAt,
       createdBy,
    }
  } 
`