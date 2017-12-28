import React from 'react';
import {graphql} from 'react-apollo';
import {bind} from 'decko';

import {NewTranslation, Translation} from 'translator/types'
import * as queries from 'translator/queries';
import * as types from 'translator/types';

interface Props {
  keyId: string;
  keyName: string;
  lang: types.Language;
  translation?: Translation | null;
  save: (trans: NewTranslation) => Promise<any>;
}

interface State {
  value: string;
  dirty: boolean;
  saving: boolean;
  error: string | null;
}

class Item extends React.Component<Props, State> {

  constructor(props: Props, ctx: any) {
    super(props, ctx);

    this.state = {
      value: '',
      dirty: false,
      saving: false,
      error: null,
    };
  }

  public render() {
    const {lang, translation} = this.props;
    const {value, saving, dirty} = this.state;

    const curValue = dirty ? value : (translation ? translation.value : '');

    let showSaveButton = false;
    if (dirty) {
      if (value && !translation) {
        showSaveButton = true;
      } else if (translation && translation.value != value) {
        showSaveButton = true;
      }
    }

    const saveBtnDisabled = showSaveButton && saving;
    const saveBtnIconCls = saveBtnDisabled ? 'fa fa-spin' : 'fa fa-floppy-o';

    return (
      <li className='list-group-item'>
        <div className='row w-100'>
          <div className='col-10'>
            <textarea
              className='w-100 form-control'
              value={curValue}
              onChange={this.onValueChange}
              rows={3}
            />

          </div>
            <div className='col-2'>
                <div className='text-center'>
                    <h4>
                        {lang.code}
                    </h4>
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
        </div>

      </li>
    );
  }

  @bind
  onValueChange(e: React.ChangeEvent<HTMLTextAreaElement>) {
    this.setState({
      value: e.target.value,
      dirty: true,
    });
  }

  @bind
  onSave() {
    this.setState({saving: true});

    const value = this.state.value;

    const data = {
      keyId: this.props.keyId,
      languageId: this.props.lang.id,
      value,
    };

    this.props.save(data)
      .then(() => {
        this.setState({
          saving: false,
          value: '',
          dirty: false,
        });

      }).catch((e: any) => {
        this.setState({
          saving: false,
          error: e.toString(),
        });
      });
  }
}

export default graphql(queries.translate, {
    props: ({mutate}: any) => ({
        save: (translation: types.NewTranslation) => mutate({variables: {translation}}),
    }),
    options: (props: any) => ({
        update: (store: any, {data: {translate}}: any) => {
          const spec = {
            query: queries.keyWithTranslations,
            variables: {key: props.keyName},
          };
          let changed = false;
          const data = store.readQuery(spec);
          data.key.translations = data.key.translations.map((t: Translation) => {
            if (t.languageId === translate.languageId) {
              changed = true;
              return translate;
            } else {
              return t;
            }
          });
          if (!changed) {
            data.key.translations.push(translate);
          }
          store.writeQuery({...spec, data});
        }
    }),
})(Item as any);