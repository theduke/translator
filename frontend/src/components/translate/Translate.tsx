import {bind} from 'decko';
import React from 'react';
import {Icon} from 'react-fa';
import {withRouter, RouteComponentProps} from 'react-router';

import {Key, Language, Translation} from 'translator/types';
import {command, translations} from 'translator/api';

import Item from './Item';

interface Props {
  keyName: string;
  languages: Language[];

  onDeleted: () => void;
}

type RoutedProps = Props & RouteComponentProps<any>;

interface State {
  loading: boolean;
  key: Key | null,
  translations: Translation[];
  error: string | null;

  innerLoading: boolean;
  innerError: string | null;

  deleteConfirm: boolean;
}

class Translate extends React.Component<RoutedProps, State> {

  public state = {
    loading: true,
    key: null,
    translations: [],
    error: null,
    innerLoading: false,
    innerError: null,
    deleteConfirm: false,
  };

  public componentDidMount() {
    translations(this.props.keyName).then(data => {
      this.setState({
        loading: false,
        key: data.key,
        translations: data.translations,
      });
    }).catch(e => {
      let err;
      if (e && e.error && e.error.code) {
        err = e.error.code;
      } else {
        err = e + '';
      }

      this.setState({
        loading: false,
        error: err,
      });
    });
  }

  public render() {
    const languages = this.props.languages;
    const {loading, translations,  error, innerLoading, innerError, deleteConfirm} = this.state;
    const keyItem: Key | null = this.state.key;

    let content;

    if (loading || error) {
      content = (
        <div className='tr-Center' key='loading'>
          {
            error ? (
              <div className='alert alert-danger'>{error}</div>
            ) : (
              <Icon spin name='spinner'/>
            )
          }
        </div>
      );
    } else if (keyItem !== null) {
      const key: Key = this.state.key as any as Key;

      const canDelete = !innerLoading;

      const items = languages.length > 0 ?
          languages.map(l => {
          return (
            <Item
              key={l.id}
              lang={l.id}
              keyName={this.props.keyName}
              onTranslationAdded={this.onTranslationAdded}
              onTranslationUpdated={this.onTranslationUpdated}
              translation={translations.find((t: Translation) => t.language === l.id)}
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

      content = (
        <div>
          <h2 className='text-center mb-3'>{key.key}</h2>

          {
            innerError && <div className='alert alert-danger'>{innerError}</div>
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
    }

    return (
      <div>
        {content}
      </div>
    );
  };

  @bind
  private onTranslationAdded(trans: Translation) {
    this.setState((s) => {
      return {
        ...s,
        translations: [...s.translations, trans],
      };
    });
  }

  @bind
  private onTranslationUpdated(trans: Translation) {
    this.setState((s) => {
      return {
        ...s,
        translations: s.translations.map(t => {
          return t.language === trans.language ? trans : t;
        })
      };
    });
  }

  @bind
  private onDelete() {
    if (!this.state.deleteConfirm) {
      this.setState({deleteConfirm: true});
      return;
    }

    this.setState({innerLoading: false});

    command({
      cmd: 'DeleteKey',
      data: {
        key: this.props.keyName,

      },
    }).then(() => {
      this.setState({
        innerLoading: false,
        deleteConfirm: false,
      });
      // Go back to main page.
      this.props.history.push('/');

    }).catch(e => {
      let err;
      if (e && e.error && e.error.code) {
        err = e.error.code;
      } else {
        err = e + '';
      }

      this.setState({
        innerLoading: false,
        deleteConfirm: false,
        innerError: err,
      });
    });

  }
}

export default withRouter<Props>(Translate);