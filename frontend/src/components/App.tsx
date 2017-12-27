import React from 'react';
import { ApolloProvider } from 'react-apollo'
import {Route as OriginalRoute, HashRouter, NavLink} from 'react-router-dom';

// Need to overwrite route as any due to weird type checking issues with
// typescript 2.4.
const Route = OriginalRoute as any;


import {ApiToken} from '../types';

import Login from './Login';
import {bind} from 'decko';

import Admin from './admin/Admin';
import Languages from './admin/Languages';
import Overview from 'translator/components/overview/Overview';
import Translate from 'translator/components/translate/Translate';

interface Props {
  client: any;
  initialToken: ApiToken | null;
}

interface State {
    token: ApiToken | null;
}

class App extends React.Component<Props, State> {

  constructor(props: Props, ctx: any) {
    super(props, ctx);

    this.state = {
      token: props.initialToken,
    };
  }

  public render() {
    const {token} = this.state;

    let content;
    let nav;

    // FIXME: remove any hack.
    const Translate2 = Translate as any;

    if (!token) {
      content = <Login onLogin={this.onLogin} />;
    } else {
      content = (
        <div>
          <Route path='/' exact render={() => <Overview /> } />
          <Route path='/admin' exact render={() => <Admin /> } />
          <Route path='/languages' render={() => <Languages />} />
          <Route
            path='/translate/:key'
            render={ ({match}: any) => ( <Translate2 keyName={match.params.key} /> ) } />
        </div>
      );

      nav = (
        <ul className='navbar-nav mr-auto' style={{flexDirection: 'row'}}>
          <li className='nav-item mr-3'>
            <NavLink className='nav-link' to={'/languages'}>Languages</NavLink>
          </li>
          <li className='nav-item'>
            <NavLink className='nav-link' to={'/admin'}>Admin</NavLink>
          </li>
        </ul>
      );
    }

    return (
      <ApolloProvider client={this.props.client}>
        <HashRouter>
          <div className='tr-App'>
            <nav className='navbar navbar-inverse bg-inverse' style={{flexDirection: 'row'}}>
              <NavLink className='navbar-brand' to='/'>Translator</NavLink>
                {nav}
            </nav>

            <div className='container pt-4'>
              {content}
            </div>
          </div>
        </HashRouter>
      </ApolloProvider>
    );
  }

  /**
   * Login handler.
   * @param session
   */
  @bind
  private onLogin(token: ApiToken) {
    localStorage.setItem('token', JSON.stringify(token));
    this.setState({token});
  }
}
export default App;
