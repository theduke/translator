import {bind} from 'decko';
import React from 'react';

import {BaseData, Key} from 'translator/types';

import NewKey from './NewKey';
import KeyTree from './KeyTree';
import KeySearch from './KeySearch';

function buildKeyTree(data: BaseData): any {

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
         subTree[name] = {...item, _item: true};
         break;
       } else {
         if (!(name in subTree)) {
           subTree[name] = {};
         }
         subTree = subTree[name];
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
            <NewKey keys={data.keys} onAdded={onKeyAdded} />
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

export default Overview;
