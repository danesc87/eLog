var token_type = 'Bearer'

export function authHeader() {
    let user = JSON.parse(localStorage.getItem('user'));

    if (user && user.access_token) {
        let user_token_type = user.token_type ? user.token_type : token_type;
        return { 
            'Authorization': user_token_type + ' ' + user.access_token,
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        };
    } else {
        return {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        };
    }
}



export function unauthHeader(){
    return { 
        'Accept': 'application/json',
        'Content-Type': 'application/json'
    }
}