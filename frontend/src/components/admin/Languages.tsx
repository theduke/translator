import React from 'react';
import * as types from 'translator/types';

import Language from './Language';
import NewLanguage from './NewLanguage';
import {bind} from 'decko';
import {command} from 'translator/api';

interface Props {
  languages: types.Language[];
  languageAdded: (lang: types.Language) => void;
  languageRemoved: (langId: string) => void;
}

interface State {
  loading: boolean;
  error: string | null;
}

class Languages extends React.Component<Props, State> {

  public state = {
    loading: false,
    error: null,
  };

  public render() {
    const {languages, languageAdded} = this.props;
    const {loading, error} = this.state;

    return (
      <div>
        <h4 className='text-center mb-3'>Languages</h4>

        {
          error && <div className="alert alert-danger">{error}</div>
        }

        <ul className = 'list-group'>
          {
            languages.map(l => <Language language={l} delete={this.deleteLanguage} canDelete={!loading}/>)
          }
          <NewLanguage languageAdded={languageAdded} />
        </ul>
      </div>
    );
  }

  @bind
  private deleteLanguage(id: string) {
    this.setState({
      error: null,
      loading: true,
    });

    command({
      cmd: 'DeleteLanguage',
      data: {
        id,
      },
    }).then(() => {
      this.setState({
        loading: false,
      });
      this.props.languageRemoved(id);
    }).catch((e) => {
      let err;
      if (e && e.error && e.error.code) {
        err = e.error.code;
      } else {
        err = e + '';
      }
      this.setState({
        error: err,
      });
    });

  }
}

export default Languages;
