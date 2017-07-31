import {bind} from 'decko';
import React from 'react';
import {graphql, compose} from 'react-apollo';

import {Language, Key} from 'translator/types';
import * as queries from 'translator/queries';

import NewKey from './NewKey';
import KeyTree from './KeyTree';
import KeySearch from './KeySearch';

// import Languages from 'translator/components/admin/Languages';

export interface KeyTree {
  [name: string]: KeyTree | (Key & {_item: boolean});
}

function buildKeyTree(keys: Key[]): KeyTree {


  // Sort keys by name first..
  keys = [...keys].sort((ak, bk) => {
    const a = ak.key;
    const b = bk.key;
    if (a < b) { return -1; }
    else if (a > b) { return 1; }
    else { return 0; }
  });

  const tree: KeyTree = {};

  for (const item of keys) {
    let parts = item.key.split('.');

    let name = '';
    let subTree = tree;
    while (true) {
       [name, ...parts] = parts;

       if (parts.length === 0) {
         subTree[name] = {...item, _item: true};
         break;
       } else {
         if (!(name in subTree)) {
           subTree[name] = {};
         }
         subTree = subTree[name] as any;
       }
    }
  }
  return tree;
}

interface Props {
  languages: {
    loading: boolean;
    error: {} | null;
    languages: Language[];
  };
  keys: {
    loading: boolean;
    error: {} | null;
    keys: Key[];
  };
}

interface State {
  openKeys: string[];
}

class Overview extends React.Component<Props, State> {
  public state = {
    openKeys: [],
  };

  public render() {
    const props = this.props;
    const loading = props.languages.loading || props.keys.loading;
    const error = props.languages.error || props.keys.error;

    const {openKeys} = this.state;

    if (error) {
      return (
        <div className="tr-Center">
          <div className="alert alert-danger">
            {error}
          </div>
        </div>
      );
    } else if (loading) {
      return (
        <div className="tr-Center">
          Loading...
        </div>
      );
    }

    const keys = props.keys.keys;
    const languages = props.languages.languages;

    const tree = buildKeyTree(keys);

    const NewKey2 = NewKey as any;

    return (
      <div className='h-100'>
        <div className='row'>
          <div className='col-9'>

            <KeySearch keys={keys} />

            <div className="card">
              <div className="card-block">

                <h3 className="card-title">Browse</h3>

                <KeyTree tree={tree} open={openKeys} show={this.showTree} path='' />
              </div>
            </div>

          </div>
          <div className='col-3'>
            <NewKey2 keyTree={tree} />

            <div className='card mt-3'>
              <div className='card-block'>
                <h4 className='card-title'>Export</h4>

                <div>
                  <a
                    href='/export/translations/keys?format=json&pretty=true'
                    target='_blank'
                  >
                    Export keys
                  </a>
                </div>

                <div className='mt-3'>
                  <p>Export all translations for a language</p>

                  <ul className='list-group'>
                    {
                      languages.map(l => {
                        return (
                            <a
                              key={l.id}
                              href={`/export/translations/${l.id}?format=json&pretty=true`}
                              target='_blank'
                            >
                              <li className='list-group-item'>
                                {l.id}
                              </li>
                            </a>
                          )
                      })
                    }
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>

      </div>
    );
  }

  @bind
  showTree(key: string) {
    this.setState({
      openKeys: key.split('.'),
    });
  }
}

export default compose(
  graphql(queries.languages, {name: 'languages'}),
  graphql(queries.keys, {name: 'keys'}),
)(Overview);
