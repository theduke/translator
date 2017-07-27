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
}

class Translate extends React.Component<RoutedProps, State> {

  public state = {
    loading: true,
    key: null,
    translations: [],
    error: null,
    innerLoading: false,
    innerError: null,
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
    const {loading, translations,  error, innerLoading, innerError} = this.state;
    const keyItem: Key | null = this.state.key;

    let content;

    if (loading || error) {
      content = (
        <div className='tr-Center'>
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


      content = (
        <div>
          <h2 className='text-center mb-3'>{key.key}</h2>

          {
            innerError && <div className='alert alert-danger'>{innerError}</div>
          }

          <div className='row'>
            <div className='col-10'>

              <ul className='list-group'>
                {
                  languages.map(l => {
                    return (
                      <Item
                        lang={l.id}
                        keyName={this.props.keyName}
                        onTranslationAdded={this.onTranslationAdded}
                        translation={translations.find((t: Translation) => t.language === l.id)}
                      />
                    );
                  })
                }

              </ul>

            </div>
            <div className='col-2'>
              <button
                className='btn btn-md btn-danger'
                onClick={this.onDelete}
                disabled={!canDelete}
              >
                <i className='fa fa-trash pr-2' style={{color: 'white'}} />
                Delete key
              </button>

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
  private onDelete() {
    this.setState({innerLoading: false});

    command({
      cmd: 'DeleteKey',
      data: {
        key: this.props.keyName,

      },
    }).then(() => {
      this.setState({innerLoading: false});
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
        innerError: err,
      });
    });

  }
}

export default withRouter<Props>(Translate);