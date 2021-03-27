<template>
  <div class="form-container">

    <v-form @submit.prevent="onSubmit()" @reset="onReset">
        <div class="">
          <v-text-field label="Email" :rules="requiredField" type="email" v-model="form.AppUser.email"/>
          <v-text-field label="First Name" :rules="requiredField" type="text" v-model="form.AppUser.first_name"/>
          <v-text-field label="Last Name" :rules="requiredField" type="text" v-model="form.AppUser.last_name"/>
          <v-text-field label="Username" :rules="requiredField" type="text" v-model="form.AppUser.username"/>
          <v-text-field label="Password" :rules="requiredField" type="password" v-model="form.AppUser.password"/>
        </div>
        <div class="text-center pt-5">
          <v-btn type="submit" color="success" :disabled="!valid" outlined>Register</v-btn>
        </div>
    </v-form>

    <div class="text-center pt-2" v-if="loading">
        <v-progress-circular
        indeterminate
        color="success"
      ></v-progress-circular>
    </div>
      <p class="text-center text-danger pt-2" v-if="error">{{errorMsg}}</p>
      <v-alert
        type="success"
        v-if="processState"
      >{{successMsg}}</v-alert>
      <div class="pt-2">
        <v-btn :to="{name: 'home'}" color="success" outlined>Back to home</v-btn>
      </div>
  </div>
</template>

<script>
  import { mapActions, mapState } from 'vuex'
  import { AppUser } from '../models'

  export default {
    data() {
      return {
        valid: true,
        form: {
            AppUser
        },
        requiredField: [
          v => !!v || 'This field is required'
        ]
      }
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error'])
    },
    methods: {
      ...mapActions('userStore',['register', 'cleanAll']),
      onSubmit(){
        this.register(this.form.AppUser);
        this.onReset();
      },
      onReset() {
        console.log(this.processState);
        if(this.processState){
          this.form.AppUser.first_name = '';
          this.form.AppUser.last_name = '';
          this.form.AppUser.username = '';
          this.form.AppUser.email = '';
          this.form.AppUser.password = '';
          this.cleanAll();
          this.$router.push({name: 'home'});
        }
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