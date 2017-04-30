import { combineReducers } from 'redux';
import initialState from './initialState';
import { ActionTypes } from './actions';

function registration(state=initialState.registration, action) {
  switch (action.type) {
    case ActionTypes.AUTH_REGISTRATION_ATTEMPT:
    case ActionTypes.AUTH_REGISTRATION_SUCCESS:
    case ActionTypes.AUTH_REGISTRATION_FAILURE:
    default:
      return state;
  }
}

function user(state=initialState.user, action) {
  switch (action.type) {
    case ActionTypes.AUTH_LOGIN_ATTEMPT:
    case ActionTypes.AUTH_LOGIN_SUCCESS:
    case ActionTypes.AUTH_LOGIN_FAILURE:
    default:
      return state;
  }
}

export default combineReducers({
  user,
  registration
});
