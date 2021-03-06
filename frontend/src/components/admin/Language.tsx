import React from 'react';

import * as types from 'translator/types';

export interface LanguageProps {
  language: types.Language;
  delete: (lang: types.Language) => void;
  canDelete: boolean;
}

export const Language = (props: LanguageProps) => {
  const deleteLang = () => {
    props.delete(props.language);
  };

  return (
    <li className='list-group-item'>
      <div className='d-inline-block'>
        {props.language.code}
      </div>
      <div className='d-inline-block ml-2'>
        {props.language.name}
      </div>
      <button
        className='btn btn-danger btn-sm ml-2'
        type='submit'
        onClick={deleteLang}
        disabled={!props.canDelete}
      >
        <i className='fa fa-trash' />
      </button>
    </li>
  );
};

export default Language;
