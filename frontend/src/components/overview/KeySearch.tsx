import {bind} from 'decko';
import React from 'react';
import {NavLink} from 'react-router-dom';

import {Key} from 'translator/types';

interface Props {
  keys: Key[];
}

interface State {
  term: string;
  results: Key[];
}

class KeySearch extends React.Component<Props, State> {
  public state = {
    term: '',
    results: [],
  };

  public render() {
    const {term} = this.state;
    const results = this.state.results;

    const showResults = !!term;

    const items = results.length > 0 ? (
      results.map((k: Key, idx) => {
        return (
          <NavLink className='btn btn-primary btn-md ml-2 mr-2 mb-2'
                   key={idx}
                   to={`/translate/${k.key}`}
          >
            <i key='icon' className='fa fa-dot-circle-o pr-2' />
            {k.key}
          </NavLink>
        );
      })
    ) : (
      <p>No matching keys found.</p>
    );

    return (
      <div className="mb-3 card">
        <div className="card-block">
          <h3 className="card-title">Key Search</h3>
          <div>
            <input
              type='text'
              value={term}
              onChange={this.onChange}
              placeholder='Enter key name...'
              className='w-100 form-control'/>
          </div>

          { showResults && (
            <ul className='list-group'>
              <li className='list-group-item'>
                {items}
              </li>
            </ul>
          )
          }

        </div>
      </div>
    );
  }

  @bind
  onChange(e: React.ChangeEvent<HTMLInputElement>) {
    const term = e.target.value.trim();
    const results = !term ? [] : this.props.keys.filter(k => {
      return k.key.search(term) !== -1;
    });
    this.setState({term, results});
  }
}

export default KeySearch;