import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { configureStore } from './store';
import { createBrowserHistory } from 'history';
import { Provider } from 'react-redux';
import { Router } from 'react-router';

import Home from './containers/home';

const history = createBrowserHistory();
const store = configureStore();

const socket = new WebSocket('ws://' + window.location.host + '/ws');
socket.onmessage = event => {
    console.log(`Received: ${event.data}`)
};

ReactDOM.render(
    <Provider store={store}>
        <Router history={history}>
            <Home />
        </Router>
    </Provider>,
    document.getElementById('root')
);
