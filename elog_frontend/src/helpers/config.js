var apiUrl = 'localhost';
var apiPort = '8090';
var protocol = 'http'

export function serverUrl() {
    return `${protocol}://${apiUrl}:${apiPort}`;
}