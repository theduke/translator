import React from 'react';
import {graphql} from 'react-apollo';
import gql from 'graphql-tag';

import compose from 'lodash/flowRight';

import * as types from 'translator/types';
import * as queries from 'translator/queries';

import Language from './Language';
import NewLanguage from './NewLanguage';
import {bind} from 'decko';

interface Props {
  data: {
    languages: types.Language[];
    loading: boolean;
    error: {};
  };

  deleteLanguage: (id: string) => Promise<any>;
}

interface State {
  deleteLoading: boolean;
  deleteError: {} | null;
  confirmDeleteLanguage: types.Language | null;
}

class Languages extends React.Component<Props, State> {

  public state: State = {
    deleteLoading: false,
    deleteError: null,
    confirmDeleteLanguage: null,
  };


  public render() {
    const {languages, loading} = this.props.data;
    const {confirmDeleteLanguage} = this.state;

    const error = this.props.data.error || this.state.deleteError;

    const langs = languages || [];

    console.log(langs);
    return (
      <div>
        <h1 className='text-center mb-3'>Languages</h1>

        {
          error && <div className="alert alert-danger">{error.toString()}</div>
        }

        {
          confirmDeleteLanguage !== null ? (
            <div className="alert alert-danger">
              <p>
                Deleting a language will delete <b>ALL TRANSLATIONS</b> that have
                been created for it.
              </p>

              <p>Child languages will be orphaned.</p>

              <p>
                <b>Are you sure you want to delete "{confirmDeleteLanguage.code}"?</b>
              </p>

              <button
                className="btn btn-lg btn-danger mr-3"
                onClick={() => this.deleteLanguage(confirmDeleteLanguage)}
              >
                Yes, delete "{confirmDeleteLanguage.code}"
              </button>

              <button className="btn btn-secondary"
                      onClick={this.cancelDelete}>
                Cancel
              </button>
            </div>

          ) : null
        }

        <div className="row">
          <div className="col-7">
            <ul className = 'list-group'>
              {
                langs.map((l, i) => <Language
                  key={i}
                  language={l}
                  delete={this.deleteLanguage}
                  canDelete={!loading}/>)
              }
            </ul>
          </div>
          <div className="col-5">
            <NewLanguage />
          </div>
        </div>

      </div>
    );
  }

  @bind
  private cancelDelete() {
    this.setState({confirmDeleteLanguage: null});
  }

  @bind
  private deleteLanguage(lang: types.Language) {
    const cur = this.state.confirmDeleteLanguage;
    if (cur && cur.id === lang.id) {

    }
    if (!cur || cur.id !== lang.id) {
      this.setState({confirmDeleteLanguage: lang});
      return;
    }

    this.setState({
      deleteError: null,
      deleteLoading: true,
      confirmDeleteLanguage: null,
    });

    this.props.deleteLanguage(lang.id)
      .then(() => {
        this.setState({
          deleteLoading: false,
        });
      }).catch((e: {}) => {
        this.setState({
          deleteLoading: true,
          deleteError: e,
        });
      });
  }
}

export const deleteQuery = gql`
mutation DeleteLanguage($id: String!) {
  deleteLanguage(lang: $id)
}
`

export default compose(
  graphql(queries.languages),
  graphql(deleteQuery, {
    props: ({mutate}) => ({
      deleteLanguage: (id: string) => {
        return (mutate as any)({
          variables: {id},

          update: (store: any) => {
            const data = store.readQuery({ query: queries.languages });
            data.languages = data.languages.filter((l: any) => l.id !== id);
            store.writeQuery({ query: queries.languages, data });
          },
        });
      },
    }),
  }),
)(Languages as any);

