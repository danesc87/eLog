import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import Vuelidate from 'vuelidate'
import 'bootstrap/dist/css/bootstrap.css'
import vuetify from './plugins/vuetify'

Vue.use(Vuelidate)

Vue.config.productionTip = false

new Vue({
    vuetify,
    router,
    store,
    render: h => h(App),
}).$mount('#app')
