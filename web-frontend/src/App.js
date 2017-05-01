import React, { Component } from 'react';
import { Provider } from 'react-redux';
import initialState from './store/initialState';
import configureStore from './store/configureStore';
import logo from './logo.svg';
import './App.css';

import RegistrationForm from './components/RegistrationForm';


const store = configureStore(initialState);

class App extends Component {
  render() {
    return (
    <Provider store={store}>
      <div className="App">
        <div className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h2>Welcome to React</h2>
        </div>
        <RegistrationForm/>
      </div>
    </Provider>);
  }
}

export default App;
