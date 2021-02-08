import {authHeader, unauthHeader, serverUrl} from '../helpers';

export const appUserServices = {
    register,
    login,
    logout
}

async function register (appUserDTO){
    const requestOptions = {
        method: 'POST',
        headers: unauthHeader(),
        body: JSON.stringify(appUserDTO)
    };
    return fetch(`${serverUrl()}/register`, requestOptions)
    .then(handleResponse)
    .then(response => {
        return response;
    });

}

async function login(username, password) {
    const requestOptions = {
        method: 'POST',
        headers: unauthHeader(),
        body: JSON.stringify({ username, password })
    };
    return fetch(`${serverUrl()}/login`, requestOptions)
    .then(handleResponse)
    .then(response => {
        let response_server = JSON.parse(response);
        let user = null;
        if (response_server['access_token']) {
            user = [];
            user = {'username': username, 'token_type': response_server['token_type'], 'access_token': response_server['access_token']};
            localStorage.setItem('user', JSON.stringify(user));
            return user;
        } else {
            return response;
        }
    });
}

async function logout() {
    const requestOptions = {
        method: 'GET',
        headers: authHeader()
    };
    return fetch(`${serverUrl()}/logout`, requestOptions)
    .then(async response => {
        localStorage.removeItem('user');
        return response.text().then(text => {
            if (!response.ok) {
                const error = text || response.statusText;
                return Promise.reject(error);
            }
            return text;
        });
    });
}

async function handleResponse(response) {
    return response.text().then(text => {
        if (!response.ok) {
            if (response.status === 401) {
                // auto logout if 401 response returned from api
                logout();
            }
            const error = text || response.statusText;
            return Promise.reject(error);
        }
        return text;
    });
}