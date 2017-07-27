import React from 'react';
import {Command, Translation} from 'translator/types';
import {bind} from 'decko';
import {command} from 'translator/api';

interface Props {
  keyName: string;
  lang: string;
  translation?: Translation | null;

  onTranslationAdded: (translation: Translation) => void;
}

interface State {
  value: string;
  saving: boolean;
  error: string | null;
}

class Item extends React.Component<Props, State> {

  constructor(props: Props, ctx: any) {
    super(props, ctx);

    this.state = {
      value: props.translation ? props.translation.value : '',
      saving: false,
      error: null,
    };
  }

  public render() {
    const {lang, translation} = this.props;
    const {value, saving} = this.state;

    const showSaveButton = !!value &&  (!translation || value != translation.value);
    const saveBtnDisabled = showSaveButton && saving;
    const saveBtnIconCls = saveBtnDisabled ? 'fa fa-spin' : 'fa fa-floppy-o';

    return (
      <li className='list-group-item'>
        <div className='row w-100'>
          <div className='col-3'>
            <div>
              <h4>
                {lang}
              </h4>
            </div>
          </div>
          <div className='col-7'>
            <textarea
              className='w-100 form-control'
              value={value}
              onChange={this.onValueChange}
              rows={3}
            />

          </div>
          <div className='col-2'>
            {
              showSaveButton && (
                <button
                  className='btn btn-primary btn-lg'
                  disabled={saveBtnDisabled}
                  onClick={this.onSave}
                >
                  <i className={saveBtnIconCls} style={{color: 'white'}} />
                </button>
              )
            }

          </div>
        </div>

      </li>
    );
  }

  @bind
  onValueChange(e: React.ChangeEvent<HTMLTextAreaElement>) {
    this.setState({
      value: e.target.value,
    });
  }

  @bind
  onSave() {
    this.setState({saving: true});

    const data = {
      key: this.props.keyName,
      lang: this.props.lang,
      value: this.state.value,
    };

    let cmd: Command;
    if (this.props.translation) {
      cmd = {
        cmd: 'UpdateTranslation',
        data,
      };
    } else {
      cmd = {
        cmd: 'CreateTranslation',
        data,
      };
    }

    command(cmd)
      .then(() => {
        this.setState({
          saving: false,
        });

        this.props.onTranslationAdded({
          ...data,
          language: this.props.lang,
          created_at: 0,
          created_by: '',
          updated_at: 0,
        });

      }).catch(e => {
        let err;
        if (e && e.error && e.error.code) {
          err = e.error.code;
        } else {
          err = e + '';
        }

        this.setState({
          saving: false,
          error: err,
        });
      });
  }
}

export default Item;