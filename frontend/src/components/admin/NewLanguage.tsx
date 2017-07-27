import React from 'react';

import {Language} from '../../types';
import {bind} from 'decko';
import {command} from 'translator/api';

export interface State {
  loading: boolean;
  error: string | null;
  id: string;
  name: string;
  parent: string;
}

export interface Props {
  languageAdded: (lang: Language) => void;
}

export class NewLanguage extends React.Component<Props, State> {

  public state = {
    loading: false,
    error: null,
    id: '',
    name: '',
    parent: '',
  };

  public render() {
    const {loading, error, id, name, parent} = this.state;

    const canSubmit = !!id && !!name && !loading;

    return (
      <li className='list-group-item'>
        <form className='' onSubmit={this.onSubmit}>
          <h5>New Language</h5>

          {
            error && <div className='alert alert-danger'>{error}</div>
          }

          <div className='form-group'>
            <label>Language ID</label>
            <input
              type='text'
              className='form-control'
              placeholder='Id...'
              value={id}
              onChange={this.onChangeId} />
          </div>

          <div className='form-group'>
            <label>Name</label>
            <input
              type='text'
              className='form-control'
              placeholder='Name...'
              value={name}
              onChange={this.onChangeName} />
          </div>

          <div className='form-group'>
            <label>Parent</label>
            <select value={parent} onChange={this.onChangeParent} className='form-control'>
              <option value=''>No parent</option>

            </select>
          </div>

          <button type='submit' className='btn btn-primary' disabled={!canSubmit}>
            Create
          </button>
        </form>
      </li>
    );
  }

  @bind
  onChangeId(e: React.ChangeEvent<HTMLInputElement>) {
    this.setState({
      id: e.target.value.trim(),
    });
  }

  @bind
  onChangeName(e: React.ChangeEvent<HTMLInputElement>) {
    this.setState({
      name: e.target.value.trim(),
    });
  }

  @bind
  onChangeParent(e: React.ChangeEvent<HTMLSelectElement>) {
    this.setState({
      parent: e.target.value,
    });
  }

  @bind
  private onSubmit(e: React.FormEvent<HTMLFormElement>) {
    const {id, name, parent} = this.state;

    e.preventDefault();

    this.setState({loading: true, error: null});

    command({
      cmd: 'CreateLanguage',
      data: {
        id,
        name,
        parent_id: parent || null,
      },
    }).then(() => {
      this.setState({
        loading: false,
        id: '',
        name: '',
        parent: '',
      });
      this.props.languageAdded({
        id,
        name,
        parent_id: parent || null,
        created_at: 0,
        created_by: '',
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
}
export default NewLanguage;
