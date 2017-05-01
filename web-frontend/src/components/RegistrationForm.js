import React, { Component } from 'react';
import PropTypes from 'prop-types';
import classNames from 'classnames';

import axios from 'axios';

const api = axios.create({
  baseURL: process.env.REACT_APP_KAIZEN_API_ROOT || '',
  timeout: 1000,
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  }
});

export default class RegistrationForm extends Component {
  static propTypes = {};

  constructor (props) {
    super(props);
    this.state = {
      username: '',
      display_name: '',
      email: '',
      password: '',
      confirm_password: ''
    };

    this.handleInput = this.handleInput.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }
  handleSubmit(event) {
    event.preventDefault();
    const payload = {
      username: this.state.username,
      password: this.state.password,
      display_name: this.state.display_name,
      email: this.state.email
    };
    console.debug(payload);
    api.post('/auth/register', payload).then(
        ok => console.info(ok),
        err => console.error(err)
    );
    return false;
  }
  handleInput(event) {
    const field = event.target.name;
    const updated = {};
    updated[field] = event.target.value;
    this.setState(updated);
  }
  render() {
    return (
        <form onSubmit={this.handleSubmit}>
          <div>
            <label htmlFor="reg-form-email">Email</label>
            <input type="text" value={this.state.email} onChange={this.handleInput} name="email" id="reg-form-email"/>
          </div>
          <div>
            <label htmlFor="reg-form-username">Handle</label>
            <input type="text" value={this.state.username} onChange={this.handleInput} name="username" id="reg-form-username"/>
          </div>
          <div>
            <label htmlFor="reg-form-display_name">Display Name</label>
            <input type="text" value={this.state.display_name} onChange={this.handleInput} name="display_name" id="reg-form-display_name"/>
          </div>
          <div>
            <label htmlFor="reg-form-password">Password</label>
            <input type="password" value={this.state.password} onChange={this.handleInput} name="password" id="reg-form-password"/>
          </div>
          <div>
            <label htmlFor="reg-form-confirm_password">Repeat Password</label>
            <input type="password" value={this.state.confirm_password} onChange={this.handleInput} name="confirm_password" id="reg-form-confirm_password"/>
          </div>
          <input type="submit" value="Sign Up"/>
        </form>
    );
  }
};
