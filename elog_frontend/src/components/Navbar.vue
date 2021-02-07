<template>
       <b-navbar toggleable="xl" type="dark" variant="dark">
         <b-container>
           <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
           <b-navbar-brand v-if="user">{{user.username}}</b-navbar-brand>
           <b-collapse id="nav-collapse" is-nav>
            <b-navbar-nav>
              <b-nav-item  exact>Home</b-nav-item>
              <b-nav-item :to="{name: 'dashboard'}">About</b-nav-item>
              <b-nav-item :to="{name: 'dashboard'}">Grid</b-nav-item>
              <b-nav-item :to="{name: 'dashboard'}">Cards</b-nav-item>
              <b-nav-item :to="{name: 'dashboard'}">Forms</b-nav-item>
              <b-nav-item :to="{name: 'dashboard'}">Validaciones</b-nav-item>
              <b-nav-item @click="logout">Logout</b-nav-item>
            </b-navbar-nav>    
           </b-collapse>
         </b-container>
       </b-navbar>

</template>

<script>
  import { mapActions, mapState } from 'vuex';

  export default {
    data() {
      return {

      }
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error', 'logged', 'user'])
    },
    methods: {
      ...mapActions('userStore',['logout'])
    }
  }
</script>

<style>

</style>