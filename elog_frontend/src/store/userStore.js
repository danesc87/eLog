import {appUserServices} from '../services'
import {messages_en} from '../localization'

const initialState = {processState: false, error: false, loading: false, errorMsg: "", successMsg: ""};

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
                        console.log(error);
                    }
                );
        },
        cleanAll({ commit }) {
            commit('cleanProcess');
        }
    }
}