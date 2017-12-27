import {bind} from 'decko';
import React from 'react';
import {graphql} from 'react-apollo';
import {Icon} from 'react-fa';
import {withRouter, RouteComponentProps} from 'react-router';

import compose from 'lodash/flowRight';

import {Language, Translation} from 'translator/types';
import * as queries from 'translator/queries';

import Item from './Item';

interface Props {
  keyName: string;

  data: {
    loading: boolean;
    error: {} | null;
    languages: Language[];
    key: {
      key: string;
      id: string;
      description: string;
      translations: Translation[];
    }
  };

  deleteKey: () => Promise<any>;
}

type RoutedProps = Props & RouteComponentProps<any>;

interface State {
  deleteError: string | null;
  deleteLoading: boolean;

  deleteConfirm: boolean;
}

class Translate extends React.Component<RoutedProps, State> {

  public state = {
    deleteLoading: false,
    deleteError: null,
    deleteConfirm: false,
  };

  public render() {
    const {deleteConfirm} = this.state;

    const keyData = this.props.data;
    let {loading, error} = keyData;
    if (loading || error) {
      return (
        <div className='tr-Center' key='loading'>
          {
            error ? (
              <div className='alert alert-danger'>{error.toString()}</div>
            ) : (
              <Icon spin name='spinner'/>
            )
          }
        </div>
      );
    }

    loading = this.state.deleteLoading;
    error = this.state.deleteError;

    const canDelete = !loading;
    const key = keyData.key;
    const languages = keyData.languages;
    const translations = key.translations;

    // FIXME: remove any hack.
    const Item2 = Item as any;

    const items = languages.length ? languages.map(l => {
        return (
          <Item2
            key={l.id}
            lang={l}
            keyName={this.props.keyName}
            keyId={this.props.data.key.id}
            translation={translations.find(t => t.languageId === l.id)}
          />
        );
      }) : (
        <li className="list-group-item">No languages configured yet.</li>
      );

    const deleteButton = !deleteConfirm || !canDelete ? (
      <button
        className='btn btn-md btn-danger'
        onClick={this.onDelete}
        disabled={!canDelete}
      >
        <i className='fa fa-trash pr-2' style={{color: 'white'}} />
        Delete key
      </button>
    ) : (

      <div>
        <div className="alert alert-danger">
          Deleting this key will delete all translations for it and cannot be
          undone.
        </div>
        <button
          className='btn btn-md btn-danger'
          onClick={this.onDelete}
        >
          <i className='fa fa-trash pr-2' style={{color: 'white'}} />
          Really Delete!
        </button>
      </div>

    );

    return (
      <div>
        <h2 className='text-center mb-3'>{key.key}</h2>

        {
          error && <div className='alert alert-danger'>{error}</div>
        }

        <div className='row'>
          <div className='col-10'>

            <ul className='list-group'>
              {items}

            </ul>

          </div>
          <div className='col-2'>
            {deleteButton}
          </div>
        </div>
      </div>
    );
  };

  @bind
  private onDelete() {
    if (!this.state.deleteConfirm) {
      this.setState({deleteConfirm: true});
      return;
    }

    this.setState({deleteLoading: true});


    this.props.deleteKey().then(() => {
      this.setState({
        deleteLoading: false,
        deleteConfirm: false,
      });
      // Go back to main page.
      this.props.history.push('/');

    }).catch((e: {}) => {
      this.setState({
        deleteLoading: false,
        deleteConfirm: false,
        deleteError: e.toString(),
      });
    });
  }
}

const Routed = withRouter(Translate);

export default compose(
  graphql(queries.keyWithTranslations, {
    options: ({ keyName }: any) => ({ variables: { key: keyName } }),
  }),

  graphql(queries.deleteKey, {
    props: ({mutate, ownProps}) => ({
      deleteKey: () => {
        return (mutate as any)({
          variables: { key: (ownProps as any).data.key.id},

          refetchQueries: [{query: queries.keys}],

        });
      },
    }),
  })

)(Routed as any);