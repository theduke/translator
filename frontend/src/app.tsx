import ApolloClient, { createNetworkInterface } from 'apollo-client'
import React from 'react';

import {render} from 'react-dom';
import App from "./components/App";

import 'styles/app.scss';
import {ApiToken} from 'translator/types';

function launch() {
  const iface = createNetworkInterface({
      uri: window.location.origin + '/api/graphql',
  });
  iface.use([{
    applyMiddleware(req, next) {
      if (!req.options.headers) {
        req.options.headers = {};  // Create the header object if needed.
      }
      // get the authentication token from local storage if it exists
      const tokenJson = localStorage.getItem('token');
      const token = tokenJson ? JSON.parse(tokenJson) : null;
      if (token && token.token) {
        req.options.headers.authorization = `Bearer ${token.token}`;
      }
      next();
    },
  }]);
  const client = new ApolloClient({
    networkInterface: iface,
    dataIdFromObject: (o: any) => {
      console.log(o);
      switch (o.__typename) {
        case 'Key':
          return `Key:${o.key}`;
        case 'Translation':
          return `Translation:${o.key}-${o.language}`;
        default:
          return `${o.__typename}:${o.id}`;
      }
    },
  });

  const tokenJson = localStorage.getItem('token');
  const token: ApiToken | null = tokenJson ? JSON.parse(tokenJson) : null;

  render(
    <App client={client} initialToken={token} />,
    document.getElementById('app-container'),
  );
}
launch();
