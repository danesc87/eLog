import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import store from '../store/index.js'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'home',
    component: Home
  },
  {
    path: '/register',
    name: 'register',
    component: () => import(/* webpackChunkName: "register" */ '../views/Register.vue')
  },
  {
    path: '/logout',
    name: 'logout',
    component: () => import(/* webpackChunkName: "logout" */ '../views/Logout.vue')
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: () => import(/* webpackChunkName: "logout" */ '../views/Dashboard.vue'),
    meta: {requiresAuth: true}
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

router.beforeEach((to, from, next) => {
  const user = store.state.userStore.logged;
 
  if(to.matched.some(record => record.meta.requiresAuth)){
    if(user){
      next()
    }else{
      next({name: 'home'})
    }
  }else{
    if(user){
      next({name: 'dashboard'})
    }else{
      next()
    }
  }
})

export default router
