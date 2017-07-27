import React from 'react';

import {NavLink} from 'react-router-dom';

interface KeyTreeProps {
  path: string;
  tree: any;
  open: string[];

  show: (key: string) => void;
}

const KeyTree = (props: KeyTreeProps) => {
  const {tree, open, show, path} = props;
  // const depth = path.split('.').length;

  const items: React.ReactNode[] = Object.keys(tree).map(name => {
    const subtree = tree[name];
    const nestedPath = path ? path + '.' + name : name;

    if (subtree._item) {
      // Actual item.

      return (
        <div key={name} className="mb-2">
          <NavLink to={`/translate/${nestedPath}`} className='btn btn-sm btn-primary'>
            <i className='fa fa-dot-circle-o pr-2' />

            <span className="">
              {name}
            </span>
          </NavLink>
        </div>
      );

    } else if (open.length > 0 && open[0] === name) {
      // Open subtree.
      const [_, ...remainingOpen] = open;
      _ + 1;
      return (
        <div key={name} className="mb-2">
          <div>
            <span className='mr-2'>
              {name}
            </span>
            <i className='fa fa-folder-open'></i>
          </div>
          <div className='w-100'></div>

          <div>
            <KeyTree
              tree={subtree}
              open={remainingOpen}
              path={nestedPath}
              show={show} />
          </div>

        </div>
      );
    } else {
      // Closed subtree.
      return (
        <div key={name} className="mb-2">
          <button className='btn btn-sm btn-secondary' onClick={() => show(nestedPath)}>
            <span className='mr-2'>
              {name}
            </span>
            <i className='fa fa-plus'></i>
          </button>
        </div>
      );
    }
  });

  return (
    <div className='ml-3'>

      <div>
        {items}
      </div>
    </div>
  );
};

export default KeyTree;