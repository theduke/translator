import React from 'react';
import {graphql} from 'react-apollo';

import * as types from 'translator/types';
import * as queries from 'translator/queries';
import {bind} from "decko";

export interface OwnProps {
    keyItem: types.Key;
    cancel: () => void;
}

interface ApolloProps {
    submit: (newKey: string) => Promise<{}>;
}

type Props = OwnProps & ApolloProps;


interface State {
    newKey: string;
    validationError: string | null;
    apiError: string | null;
    loading: boolean;
}

class RenameKeyComponent extends React.Component<Props, State> {

    state: State = {
        newKey: '',
        validationError: null,
        apiError: null,
        loading: false,
    };


    public render() {
        const {cancel} = this.props;
        const {newKey, validationError, apiError, loading} = this.state;
        const canSubmit = validationError === null && !loading && newKey.length > 0;
        const canCancel = !loading;

        const errorMsg = validationError || apiError;

        return (
            <div className='card'>
                <div className="card-block">
                    <form>
                        <div className="form-group row">
                            <label className='col-2 col-form-label'>Rename: </label>

                            <input
                                type='text'
                                placeholder='New key...'
                                className='form-control col'
                                onChange={this.onChange}
                                value={newKey}
                              />

                            <div className="btn-group col-3">
                                <button
                                    className='btn btn-info'
                                    type="button"
                                    onClick={this.submit}
                                    disabled={!canSubmit}
                                >
                                    <i className='fa fa-pencil-square-o pr-2' style={{color: 'white'}} />
                                    Rename
                                </button>
                                <button
                                    className="btn btn-default"
                                    type="button"
                                    onClick={cancel}
                                    disabled={!canCancel}
                                >
                                    Cancel
                                </button>
                            </div>
                        </div>

                    </form>

                    {
                        errorMsg ? (
                            <div className="alert alert-danger">{errorMsg}</div>
                        ) : null
                    }
                </div>
            </div>
        );
    }

    @bind
    private onChange(ev: React.ChangeEvent<HTMLInputElement>) {
        const newKey = ev.target.value.trim();

        const error = this.props.keyItem.key === newKey ? "Can't rename key to existing name" : types.validateKey(newKey);

        this.setState({
            newKey,
            validationError: error,
            apiError: null,
        });
    }

    @bind
    private submit() {
        this.setState({loading: true});
        this.props.submit(this.state.newKey)
            .then(() => this.props.cancel())
            .catch((e) => {
                this.setState({
                    loading: false,
                    apiError: e.toString(),
                });
            });
    }
}

export const RenameKey = graphql<{}, OwnProps>(queries.renameKey, {
    props: ({mutate, ownProps}) => ({
        submit: (newKey: string): Promise<{}> => {
            return (mutate as any)({variables: {id: ownProps.keyItem.id, newKey}});
        },
    }),
})(RenameKeyComponent);
