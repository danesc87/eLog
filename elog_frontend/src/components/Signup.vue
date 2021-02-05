<template>
  <div class="form-container">
    <b-form @submit.prevent="register(form.AppUser)" @reset="onReset">
      <b-form-group id="input-group-1" label="Email:" label-for="email">
        <b-form-input
          id="email"
          v-model="form.AppUser.email"
          required
          placeholder="Enter email"
        ></b-form-input>
      </b-form-group>
      <b-form-group id="input-group-1" label="Full Name:" label-for="name">
        <b-form-input
          id="firstname"
          v-model="form.AppUser.first_name"
          required
          placeholder="First Name"
        ></b-form-input>

        <b-form-input
          id="lastname"
          v-model="form.AppUser.last_name"
          required
          placeholder="Last Name"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-2" label="Username:" label-for="username">
        <b-form-input
          id="username"
          v-model="form.AppUser.username"
          required
          placeholder="Enter username"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-3" label="Password:" label-for="text-password">
        <b-form-input
          id="input-5"
          type="password"
          v-model="form.AppUser.password"
          required
          placeholder="Enter password"
        ></b-form-input>
      </b-form-group>
      <div>
          <b-button type="submit" variant="outline-success">Submit</b-button>
      </div>
    </b-form>
    <div class="text-center pt-2" v-if="loading">
        <b-button variant="success" disabled>
        <b-spinner small type="grow"></b-spinner>
          Loading...
        </b-button>
      </div>
        <p class="text-center text-danger pt-2" v-if="error">{{errorMsg}}</p>
        <b-modal ref="register-modal" hide-footer v-model="processState">
          <template class="text-center" #modal-title>
            {{successMsg}}
          </template>
          <b-button class="btn-success mt-3" block @click="onReset" to="/">OK</b-button>
        </b-modal>
  </div>
</template>

<script>
  import { mapActions, mapState } from 'vuex'
  import { AppUser } from '../models'

  export default {
    data() {
      return {
        form: {
            AppUser
        }
      }
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error'])
    },
    methods: {
      ...mapActions('userStore',['register', 'cleanAll']),
      onReset(evt) {
        evt.preventDefault();
        this.form.AppUser.first_name = '';
        this.form.AppUser.last_name = '';
        this.form.AppUser.username = '';
        this.form.AppUser.email = '';
        this.form.AppUser.password = '';
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