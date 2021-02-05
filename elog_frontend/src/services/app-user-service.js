import {unauthHeader, serverUrl} from '../helpers';

export const appUserServices = {
    register
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

async function handleResponse(response) {
    return response.text().then(text => {
        let data = null;
        try{
            data = text; //&& JSON.parse(text);
        }catch{
            data = text;
        }
        if (!response.ok) {
            if (response.status === 401) {
                // auto logout if 401 response returned from api
                //logout();
            }
            const error = data || response.statusText;
            return Promise.reject(error);
        }
        return data;
    });
}