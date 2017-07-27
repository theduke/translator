import * as types from './types';

const baseUrl
  = location.protocol+'//'+location.hostname+(location.port ? ':'+location.port: '');

const buildUrl = (path: string): string => baseUrl + '/' + path;

export const baseData = (): Promise<types.BaseData> => {
  const url = buildUrl('api/base-data');
  return fetch(url).then(r => {
    return r.json();
  })
    .then(data => {
      return data;
    });
};


export const translations = (key: string): Promise<types.TranslationsResponse> => {
  const url = buildUrl(`api/translations/${key}`);
  return fetch(url, {

  }).then(r => r.json())
};

export const command = (cmd: types.Command): Promise<types.CommandSuccess> => {
  const body = JSON.stringify(cmd);
  const url = buildUrl('api/command');
  return fetch(url, {
    method: 'POST',
    body,
    headers: {
      'Content-Type': 'application/json',
    },
  }).then(r => r.json())
    .then(data => {
      return data.error ? Promise.reject(data) : data;
    });
};
