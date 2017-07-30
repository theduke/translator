import {bind} from 'decko';
import React from 'react';
import {gql, graphql} from 'react-apollo';

import * as queries from 'translator/queries';


// import {Language} from '../../types';

export interface State {
  loading: boolean;
  error: string | null;
  id: string;
  name: string;
  parent: string;
}

export interface Props {
  create: (data: {
    id: string,
    name: string,
    parentId: string | null,
  }) => Promise<any>;
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

    this.props.create({
      id,
      name,
      parentId: parent || null,
    }).then(() => {
      this.setState({
        loading: false,
        id: '',
        name: '',
        parent: '',
      });

    }).catch(e => {
      this.setState({
        loading: false,
        error: e.toString(),
      });
    });
  }
}

const createMutation = gql`
mutation CreateLanguage($lang: NewLanguage!) {
  createLanguage(lang: $lang) {
    id
    name
    parentId
  }
}
`;


export default graphql(createMutation, {
  props: ({mutate}) => ({
    create: (data: any) => {
      return (mutate as any)({
        variables: {lang: data},

        update: (store: any, { data: { createLanguage } }: any) => {
          // Read the data from our cache for this query.
          const data = store.readQuery({ query: queries.languages });
          // Add our comment from the mutation to the end.
          data.languages.push(createLanguage);
          // Write our data back to the cache.
          store.writeQuery({ query: queries.languages, data });
        },

      });
    },
  }),
})(NewLanguage);
