import {bind} from 'decko';
import React from 'react';

import {BaseData, Key} from 'translator/types';

import NewKey from './NewKey';
import KeyTree from './KeyTree';
import KeySearch from './KeySearch';

export interface KeyTree {
  [name: string]: KeyTree | (Key & {_item: boolean});
}

function buildKeyTree(data: BaseData): KeyTree {

  // Sort keys by name first..
  const keys = data.keys.sort((ak, bk) => {
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

function buildKeyExport(data: BaseData): KeyTree {

  // Sort keys by name first..
  const keys = data.keys.sort((ak, bk) => {
    const a = ak.key;
    const b = bk.key;
    if (a < b) { return -1; }
    else if (a > b) { return 1; }
    else { return 0; }
  });

  const tree = {} as any;

  for (const item of keys) {
    let parts = item.key.split('.');

    let name = '';
    let subTree = tree;
    while (true) {
      [name, ...parts] = parts;

      if (parts.length === 0) {
        subTree[name] = item.key;
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
  data: BaseData;
  onKeyAdded: (key: Key) => void;
}

interface State {
  openKeys: string[];
}

class Overview extends React.Component<Props, State> {
  public state = {
    openKeys: [],
  };

  public render() {
    const {data, onKeyAdded} = this.props;
    const tree = buildKeyTree(data);
    const {openKeys} = this.state;

    return (
      <div className='h-100'>
        <div className='row'>
          <div className='col-9'>
            <KeySearch data={data} />

            <div>
              <h3>Browse</h3>

              <KeyTree tree={tree} open={openKeys} show={this.showTree} path='' />
            </div>

          </div>
          <div className='col-3'>
            <NewKey keyTree={tree} onAdded={onKeyAdded} />

            <div className='card mt-3'>
              <div className='card-block'>
                <h4 className='card-title'>Export</h4>

                <div>
                  <button className='btn btn-secondary' onClick={this.exportKeys}>
                    Export keys
                  </button>
                </div>

                <div className='mt-3'>
                  <p>Export all translations for a language</p>

                  <ul className='list-group'>
                    {
                      data.languages.map(l => {
                        return (
                            <a
                              key={l.id}
                              href={`/export/translations/${l.id}`}
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
  private exportKeys() {
    const exp = buildKeyExport(this.props.data);
    const json = JSON.stringify(exp);

    const w = window.open();
    w.document.write(json);
  }

  @bind
  showTree(key: string) {
    this.setState({
      openKeys: key.split('.'),
    });
  }
}

export default Overview;
