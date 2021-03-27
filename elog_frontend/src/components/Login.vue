<template>
  <div class="form-container">
    <v-form @submit.prevent="onSubmit({'username': form.username, 'password': form.password})" @reset="onReset" v-if="show">
        <div class="">
          <v-text-field label="Username" :rules="requiredField" type="text" v-model="form.username"/>
          <v-text-field label="Password" :rules="requiredField" type="password" v-model="form.password"/>
        </div>
        <div class="text-center pt-5">
          <v-btn type="submit" color="success" :disabled="!valid" outlined>Login</v-btn>
        </div>
        <div>
          <a class="text-muted" href="/register"><small>Register</small></a>
        </div>
    </v-form>
    <div class="text-center pt-2" v-if="loading">
      <v-btn color="success" disabled>
        Loading...
      </v-btn>
    </div>
    <div class="pt-5">
      <p class="text-center text-danger pt-2" v-if="error">{{errorMsg}}</p>
    </div>
  </div>
</template>

<script>
  import { mapActions, mapState } from 'vuex'

  export default {
    data() {
      return {
        valid: true,
        form: {
          username: '',
          password: '',
        },
        show: true,
        requiredField: [
          v => !!v || 'This field is required'
        ]
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