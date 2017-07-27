import React from 'react';

import * as types from 'translator/types';

import Languages from './Languages';
import {Language} from 'translator/types';

interface Props {
  data: types.BaseData;
  languageAdded: (lang: Language) => void;
  languageRemoved: (langId: string) => void;
}

interface State {
  deletingLanguage: boolean;
}

class Admin extends React.Component<Props, State> {

  public state = {
    deletingLanguage: false,
  };

  public render() {
    const props = this.props;

    return (
      <div>
        <h2 className='text-center'>Admin</h2>
        <div className='row'>
          <div className='col-6'>
            <Languages
              languages={props.data.languages}
              languageAdded={props.languageAdded}
              languageRemoved={props.languageRemoved}
            />

          </div>
          <div className='col-6'>

          </div>

        </div>
      </div>
    );
  }
}

export default Admin;