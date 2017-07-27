import React from 'react';
import {withRouter, RouteComponentProps} from 'react-router';

import {bind} from 'decko';
import {Key} from 'translator/types';
import {command} from 'translator/api';

interface Props {
  keys: Key[];
  onAdded: (key: Key) => void;
}

type RoutedProps = Props & RouteComponentProps<any>;


interface State {
  key: string;
  description: string;
  error: string | null;
  loading: boolean;

  // Used to delay display of error message until after a small timeout to improve the UX.
  // Note that it is cleared in componentWillUnmount.
  errorTimerHandle: number | null;
}

class NewKey extends React.Component<RoutedProps, State> {
  public state = {
    key: '',
    description: '',
    error: null,
    loading: false,
    errorTimerHandle: null,
  };

  public render() {
    const {key, description, loading, error, errorTimerHandle} = this.state;

    const canSubmit = !!key && !loading && !error;

    const showError = !!error && errorTimerHandle === null;

    return (
      <div>
        <h4 className='mb-3'>New Key</h4>


        <form>
          <div className="form-group">
            <input
              type='text'
              placeholder='Key...'
              className='form-control'
              onChange={this.onKeyChange}
              value={key}
            />
          </div>

          <div className="form-group">
            <label>Key description (optional)</label>
            <input
              type='text'
              placeholder='Description...'
              className='form-control'
              onChange={this.onDescriptionChange}
              value={description}
            />
          </div>

          {
            showError && <div className='alert alert-danger'>{error}</div>
          }

          <button
            type='submit'
            className='btn btn-primary mr-2'
            onClick={this.onSubmit}
            disabled={!canSubmit}
          >
            Create
          </button>

          <button
            className='btn btn-info'
            onClick={this.onSubmitAndEdit}
            disabled={!canSubmit}
          >
            Create &amp; Edit
          </button>
        </form>
      </div>
    );
  }

  public componentWillUnmount() {
    const handle = this.state.errorTimerHandle;
    if (handle !== null) {
      clearTimeout(handle);
    }
  }

  @bind
  private onKeyChange(e: React.ChangeEvent<HTMLInputElement>) {
    const key = e.target.value.trim();

    let error = null;

    // Check key validity.
    if (key !== '') {
      const re = /^[a-zA-Z\d]+(\.[a-zA-Z\d]+)*$/;
      if (re.exec(key) === null) {
        if (key[key.length - 1] !== '.') {
          error = 'Invalid key (only a-z / A-Z / 0-9 / . are allowed)';
        } else {
          error = 'Keys cannot end with a dot.'
        }
      }

      // Check key uniqueness.
      for (const item of this.props.keys) {
        if (item.key === key) {
          error = 'Key already exists!';
          break;
        } else if (item.key.startsWith(key)) {
          error = `Invalid key: nested child key : ${item.key} already exists`;
          break;
        } else if (key.startsWith(item.key)) {
          // error = `Invalid key: parent key : ${item.key} already exists`;
          break;
        }
      }
    }

    // Clear old timer handle if it exists.
    const oldHandle = this.state.errorTimerHandle;
    if (oldHandle !== null) {
      clearTimeout(oldHandle);
    }

    const errorTimerHandle = setTimeout(() => {
      this.setState((s) => {
        return {
          ...s,
          errorTimerHandle: null,
        };
      });
    }, 700);


    this.setState({
      key,
      error,
      errorTimerHandle,
    });
  }

  @bind
  private onDescriptionChange(e: React.ChangeEvent<HTMLInputElement>) {
    const description = e.target.value.trim();
    this.setState({description});
  }

  @bind
  private onSubmit() {
    const {key, description} = this.state;

    this.setState({loading: true, error: null});

    return command({
      cmd: 'CreateKey',
      data: {
        key,
        description: description || null,
      },
    }).then(() => {
      this.setState({
        loading: false,
        key: '',
        description: '',
      });
      this.props.onAdded({
        key,
        description: description || null,
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

  @bind
  private onSubmitAndEdit() {
    const key = this.state.key;
    this.onSubmit().then(() => {
      this.props.history.push(`/translate/${key}`);
    });
  }
}

export default withRouter<Props>(NewKey);