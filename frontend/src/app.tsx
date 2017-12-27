import React from 'react';

import { ApolloClient } from 'apollo-client';
import { InMemoryCache } from 'apollo-cache-inmemory';
import { createHttpLink } from 'apollo-link-http';
import { setContext } from 'apollo-link-context';

import {render} from 'react-dom';
import App from "./components/App";

import 'styles/app.scss';
import {ApiToken} from 'translator/types';

function launch() {

  const httpLink = createHttpLink({ uri: '/api/graphql' });
  const middlewareLink = setContext(() => {
      // get the authentication token from local storage if it exists
      const tokenJson = localStorage.getItem('token');
      const token = tokenJson ? JSON.parse(tokenJson) : null;

      if (token) {
        return {
            headers: { authorization: 'Bearer ' + token},
        };
      } else {
        return {};
      }
  });
  const link: any = middlewareLink.concat(httpLink);

  const client = new ApolloClient({
      link,
      cache: new InMemoryCache(),
  });

  /*
  const client = new ApolloClient({
    networkInterface: iface,
    dataIdFromObject: (o: any) => {
      console.log(o);
      switch (o.__typename) {
        case 'Key':
          return `Key:${o.id}`;
        case 'Translation':
          return `Translation:${o.id}`;
        default:
          return `${o.__typename}:${o.id}`;
      }
    },
  });
  */

  const tokenJson = localStorage.getItem('token');
  const token: ApiToken | null = tokenJson ? JSON.parse(tokenJson) : null;

  render(
    <App client={client} initialToken={token} />,
    document.getElementById('app-container'),
  );
}
launch();
