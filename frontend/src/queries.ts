import {gql} from 'react-apollo';

export const languages = gql`
query {
  languages {
    id
    name
    parentId
  }
}
`;

export const keys = gql`
  query {
    keys {
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
      key
      description
      createdAt
      createdBy
      translations {
        language,
        key,
        value,
        createdAt,
        updatedAt,
        createdBy,
      }
    }
    
    languages {
      id
      name
      parentId
    }
  
  }

`;

export const createKey = gql`
  mutation createKey($key: NewKey!) {
    createKey(key: $key) {
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
       language,
       key,
       value,
       createdAt,
       updatedAt,
       createdBy,
    }
  } 
`