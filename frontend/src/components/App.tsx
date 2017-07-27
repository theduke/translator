import React from 'react';
import {Icon} from 'react-fa';
import {Route as OriginalRoute, HashRouter, NavLink} from 'react-router-dom';

// Need to overwrite route as any due to weird type checking issues with
// typescript 2.4.
const Route = OriginalRoute as any;


import {BaseData, Session, Language, Key} from '../types';

import Login from './Login';
import {bind} from 'decko';
import {baseData} from 'translator/api';

import Admin from './admin/Admin';
import Overview from 'translator/components/overview/Overview';
import Translate from 'translator/components/translate/Translate';

interface State {
    session: Session | null;

    data: BaseData | null;
    error: string | null;
}

class App extends React.Component<{}, State> {

  public state = {
    session: null,
    data: null,
    error: null,
  };

  constructor(props: {}, ctx: any) {
    super(props, ctx);
  }

  componentDidMount() {
    this.onLogin({
      username: 'admin',
      token: '',
    })
  }

  public render() {
    const {data, error, session} = this.state;

    let content;
    let nav;

    if (!session) {
      content = <Login onLogin={this.onLogin} />;
    } else {
      if (!data) {
        content = (
          <div className='tr-Center'>
            {
              error ? (
                <div className='alert alert-danger'>{error}</div>
              ) : (
                <Icon spin name='spinner' />
              )
            }
          </div>

        );
      } else {
        const d = data as BaseData;
        content = (
          <div>
            <Route
              path='/admin'
              render={() => {
                return (
                  <Admin data={d}
                         languageAdded={this.languageAdded}
                         languageRemoved={this.languageRemoved}  />
                )
              }}
            />

            <Route
              path='/'
              exact
              render={() => (
                <Overview
                  data={d}
                  onKeyAdded={this.onKeyAdded}
                />
              )}
            />

            <Route
              path='/translate/:id'
              render={({match}: any) => {
                const key = match.params.id;
                return (
                  <Translate
                    keyName={match.params.id}
                    languages={d.languages}
                    onDeleted={() => this.onKeyDeleted(key)} />
                );
              }} />
          </div>
        );

        nav = (
          <ul className='navbar-nav mr-auto'>
            <li className='nav-item'>
              <NavLink className='nav-link' to={'/admin'}>Admin</NavLink>
            </li>
          </ul>
        );
      }
    }

    return (
      <HashRouter>
        <div className='tr-App'>
          <nav className='navbar navbar-inverse bg-inverse'>
            <NavLink className='navbar-brand' to='/'>Translator</NavLink>
              {nav}
          </nav>

          <div className='container pt-4'>
            {content}
          </div>
        </div>
      </HashRouter>
    );
  }

  /**
   * Login handler.
   * @param session
   */
  @bind
  private onLogin(session: Session) {
    this.setState({session});
    baseData().then(data => {
      this.setState({
        data,
      });
    }).catch(e => {
      let err;
      if (e && e.error && e.error.code) {
        err = e.error.code;
      } else {
        err = e + '';
      }
      this.setState({error: err});
    });
  }

  @bind
  private languageAdded(lang: Language) {
    this.setState((state) => {

      const data = state.data || {languages: []};
      const langs = data.languages;

      return {
        ...state,
        data: {
          ...state.data,
          languages: [...langs, lang],
        },
      };
    });
  }

  @bind
  private languageRemoved(langId: string) {
    this.setState((state) => {

      const keep = (lang: Language) => lang.id !== langId;

      const data = state.data || {languages: []};
      const oldLangs = data.languages;
      const langs = (oldLangs as any).filter(keep);

      return {
        ...state,
        data: {
          ...state.data,
          languages: langs,
        },
      };
    });
  }

  @bind
  private onKeyAdded(key: Key) {
    this.setState((s) => {
      const data = s.data || {keys: []};

      return {
        ...s,
        data: {
          ...s.data,
          keys: [...data.keys, key],
        },
      };
    });
  }

  @bind
  private onKeyDeleted(key: string) {
    this.setState((state) => {

      const keep = (k: Key) => k.key !== key;

      const data = state.data || {keys: []};
      const oldKeys = data.keys;
      const keys = (oldKeys as any).filter(keep);

      return {
        ...state,
        data: {
          ...state.data,
          keys: keys,
        },
      };
    });
  }

}
export default App;
