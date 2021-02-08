<template>
  <div class="form-container">
    <b-form @submit.prevent="onSubmit({'username': form.username, 'password': form.password})" @reset="onReset" v-if="show">
      <b-form-group id="input-group-2" label="Username:" label-for="username">
        <b-form-input
          id="input-2"
          v-model="form.username"
          required
          placeholder="Enter username"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-3" label="Password:" label-for="text-password">
        <b-form-input
          id="input-3"
          type="password"
          v-model="form.password"
          required
          placeholder="Enter password"
        ></b-form-input>
      </b-form-group>
      <div>
          <b-button type="submit" variant="outline-success">Login</b-button>
      </div>
      <div>
        <b-link class="text-muted" :to="{name: 'register'}" ><small>Register</small></b-link>
      </div>
    </b-form>
    <div class="text-center pt-2" v-if="loading">
      <b-button variant="success" disabled>
      <b-spinner small type="grow"></b-spinner>
        Loading...
      </b-button>
    </div>
      <p class="text-center text-danger pt-2" v-if="error">{{errorMsg}}</p><div class="pt-5">
          <b-button :to="{name: 'home'}" variant="outline-success">Back to home</b-button>
      </div>
  </div>
</template>

<script>
  import { mapActions, mapState } from 'vuex'

  export default {
    data() {
      return {
        form: {
          username: '',
          password: '',
        },
        show: true
      }
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error', 'logged'])
    },
    methods: {
      ...mapActions('userStore',['login', 'cleanAll']),
      async onSubmit(user) {
        this.login(user);
      },
      onReset(){
        this.form.username = '';
        this.form.password = '';
        this.cleanAll();
      }
    }
  }
</script>
<style scoped>
    .form-container{
        width: 100%;
        max-width: 330px;
        padding: 15px;
        margin: 0 auto;
    }
</style>