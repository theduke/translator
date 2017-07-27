import {bind} from 'decko';
import React from 'react';
import {NavLink} from 'react-router-dom';

import {BaseData, Key} from 'translator/types';

interface Props {
  data: BaseData;
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

    return (
      <div className="mb-3">
        <h3>Search</h3>
        <div>
          <input
            type='text'
            value={term}
            onChange={this.onChange}
            placeholder='Key name search...'
            className='w-100 form-control'/>
        </div>

        { showResults && (
          <ul className='list-group'>
            <li className='list-group-item'>
            {
              results.map((k: Key) => {
                return (
                    <NavLink className='btn btn-primary btn-md ml-2 mr-2'
                      to={`/translate/${k.key}`}
                    >
                      <i className='fa fa-dot-circle-o pr-2' />
                      {k.key}
                    </NavLink>
                );
              })
            }
            </li>
          </ul>
        )
        }

      </div>
    );
  }

  @bind
  onChange(e: React.ChangeEvent<HTMLInputElement>) {
    const term = e.target.value.trim();
    const results = !term ? [] : this.props.data.keys.filter(k => {
      return k.key.search(term) !== -1;
    });
    this.setState({term, results});
  }
}

export default KeySearch;