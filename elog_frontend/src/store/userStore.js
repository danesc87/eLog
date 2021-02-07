import {appUserServices} from '../services'
import {messages_en} from '../localization'
import router from '@/router';

const user = JSON.parse(localStorage.getItem('user'));
const initialState = user 
        ? {processState: false, error: false, loading: false, errorMsg: "", successMsg: "", logged: true, user}
        :{processState: false, error: false, loading: false, errorMsg: "", successMsg: "", logged: false, user: null};

export const userStore = {
    namespaced: true,
    state: initialState,
    mutations: {
        startProcess(state){
            state.loading = true;
            state.error = false;
            state.errorMsg = "";
            state.successMsg = "";
            state.processState = false;
        },
        processSuccess(state, successMsg){
            state.processState = true;
            state.error = false;
            state.loading = false;
            state.successMsg = successMsg;
        },
        processFailed(state, errorMsg){
            state.processState = false;
            state.error = true;
            state.loading = false;
            state.errorMsg = errorMsg;
            state.successMsg = "";
        },
        cleanProcess(state){
            state.processState = false; 
            state.error = false;
            state.loading = false
            state.errorMsg = "";
            state.successMsg = "";
        },
        setLoginState(state, user){
            if(user){
                state.logged = true;
                state.user = user;
            }else{
                state.logged = false;
                state.user = null;
            }
        }
    },
    actions: {
        register({ commit }, appUserDTO ) {
            commit('startProcess');
            appUserServices.register(appUserDTO)
                .then(
                    () => {                      
                        commit('processSuccess', messages_en.user_register_successful);
                    },
                    error => {
                        if(error.toString().indexOf('Failed to fetch') != -1){
                            error = messages_en.server_not_found;
                        }
                        commit('processFailed', error);
                    }
                );
        },
        login({ commit }, { username, password }){
            commit('startProcess');
            appUserServices.login(username, password)
                .then(
                    user => {
                        commit('processSuccess', messages_en.user_login_successful);
                        commit('setLoginState', user);
                        router.push({name: 'dashboard'});
                        commit('cleanProcess');
                    },
                    error => {
                        if(error.toString().indexOf('Failed to fetch') != -1){
                            error = messages_en.server_not_found;
                        }
                        commit('processFailed', error);
                    }
                );
        },
        logout({ commit }){
            commit('startProcess');
            appUserServices.logout()
            .then( () => {
                    commit('processSuccess', messages_en.user_logout_successful);
                    router.push({name: 'logout'});
                    commit('cleanProcess');
                },
                error => {
                    router.push({name: 'home'});
                    if(error.toString().indexOf('Failed to fetch') != -1){
                        error = messages_en.server_not_found;
                    }
                    commit('processFailed', error);
                }
            );
            commit('setLoginState', null);
        },
        cleanAll({ commit }) {
            commit('cleanProcess');
        }
    }
}