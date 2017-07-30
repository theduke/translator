import {bind} from 'decko';
import React from 'react';
import { gql, graphql } from 'react-apollo';

import {ApiToken} from 'translator/types';

interface Props {
  onLogin: (token: ApiToken) => void;
  mutate: any;
}

interface State {
  username: string;
  password: string;
  loading: boolean;
  error: string | null;
}

class Login extends React.Component<Props, State> {
  public state = {
    username: '',
    password: '',
    loading: false,
    error: null,
  };

  public render() {
    const {username, password, loading, error} = this.state;

    const canSubmit = !!username && !!password && !loading;

    return (
      <div className='tr-Login'>
        <form onSubmit={this.onSubmit}>
          {
            error && <div className='alert alert-danger'>{error}</div>
          }
          <div className='form-group'>
            <label>Username</label>
            <input
              type='text'
              className='form-control'
              placeholder='Username...'
              value={username}
              onChange={this.onChangeUsername} />
          </div>
          <div className='form-group'>
            <label>Password</label>
            <input
              type='password'
              value={password}
              className='form-control'
              placeholder='password...'
              onChange={this.onChangePassword} />
          </div>
          <button
            disabled={!canSubmit}
            className='btn btn-primary'
            type='submit'
          >
            Login
          </button>
        </form>
      </div>
    );
  }

  @bind
  private onChangeUsername(e: React.ChangeEvent<HTMLInputElement>) {
    this.setState({username: e.target.value.trim()});
  }

  @bind
  private onChangePassword(e: React.ChangeEvent<HTMLInputElement>) {
    this.setState({password: e.target.value.trim()});
  }

  /**
   * onSubmit handler.
   * @param e
   */
  @bind
  private onSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const {username, password} = this.state;
    this.setState({
      loading: true,
      error: null,
    });
    this.props.mutate({
      variables: { user: username, pw: password },
    }).then((res:any ) => {
      this.props.onLogin(res);
    }).catch((e: any) => {
      this.setState({
        loading: false,
        error: e + '',
      });
    });
  }
}

const loginMutation = gql`
  mutation login($user: String!, $pw: String!) {
    login(user: $user, password: $pw) {
      token
      createdBy
      expiresAt
    }
  }
`;

export default graphql(loginMutation)(Login as any) as any;
