import React from 'react';
import {Link} from 'react-router-dom';

interface Props {
}

interface State {
}

class Admin extends React.Component<Props, State> {

  public render() {
    return (
      <div>
        <h2 className='text-center'>Admin</h2>
        <div className='row'>
          <div className='col-6'>
            <Link to='/admin/languages' className='btn btn-primary btn-xl'>
              Languages
            </Link>
          </div>
          <div className='col-6'>
            { /* <Route path='/admin/users' render={() => <Users />} /> */ }
          </div>

        </div>
      </div>
    );
  }
}

export default Admin;

