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
  confirmDeleteLanguage: string | null;
}

class Languages extends React.Component<Props, State> {

  public state = {
    loading: false,
    error: null,
    confirmDeleteLanguage: null,
  };


  public render() {
    const {languages, languageAdded} = this.props;
    const {loading, error, confirmDeleteLanguage} = this.state;

    return (
      <div>
        <h4 className='text-center mb-3'>Languages</h4>

        {
          error && <div className="alert alert-danger">{error}</div>
        }

        {
          confirmDeleteLanguage ? (
            <div className="alert alert-danger">
              <p>
                Deleting a language will delete <b>ALL TRANSLATIONS</b> that have
                been created for it.
              </p>

              <p>It will also delete all <b>child languages</b>.</p>

              <p>
                <b>Are you sure you want to delete {confirmDeleteLanguage}?</b>
              </p>

              <button
                className="btn btn-lg btn-danger"
                onClick={() => this.deleteLanguage(confirmDeleteLanguage as any)}
              >
                Yes, delete {confirmDeleteLanguage}
              </button>
            </div>

          ) : null
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
    if (this.state.confirmDeleteLanguage === null) {
      this.setState({confirmDeleteLanguage: id});
      return;
    }

    this.setState({
      error: null,
      loading: true,
      confirmDeleteLanguage: null,
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
