import React from 'react';

import {render} from 'react-dom';
import App from "./components/App";

import 'styles/app.scss';

function launch() {
  render(
    <App/>,
    document.getElementById('app-container'),
  );
}
launch();
