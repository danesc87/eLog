<template>
  <div class="form-container">

    <v-form 
      @submit.prevent="submit()" 
      @reset="onReset"
    >
        <div>
          <v-text-field 
            label="E-mail"
            :error-messages="emailErrors"
            required
            v-model="AppUser.email" 
            @input="$v.AppUser.email.$touch()"
            @blur="$v.AppUser.email.$touch()"
          />
          <v-text-field 
            label="First Name" 
            :error-messages="firstNameErrors"
            type="text" 
            required
            v-model="AppUser.first_name"
            @input="$v.AppUser.first_name.$touch()"
            @blur="$v.AppUser.first_name.$touch()"
          />
          <v-text-field 
            label="Last Name" 
            :error-messages="lastNameErrors"
            type="text" 
            v-model="AppUser.last_name"
            @input="$v.AppUser.last_name.$touch()"
            @blur="$v.AppUser.last_name.$touch()"
          />
          <v-text-field 
            label="Username"
            :error-messages="usernameErrors"
            type="text" 
            v-model="AppUser.username"
            @input="$v.AppUser.username.$touch()"
            @blur="$v.AppUser.username.$touch()"
          />
          <v-text-field 
            label="Password"
            :error-messages="passwordErrors"
            type="password" 
            v-model="AppUser.password"
            @input="$v.AppUser.password.$touch()"
            @blur="$v.AppUser.password.$touch()"
          />
        </div>
        <div class="text-center pt-5" v-if="!processState">
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
  import { AppUser, AppUserDefinitions } from '../models'
  import { validationMixin } from 'vuelidate'
  import { required, email, maxLength } from 'vuelidate/lib/validators'
  import { messages_en } from '../localization'

  export default {
    data() {
      return {
        valid: true,
        AppUser
      }
    },
    mixins: [validationMixin],
    validations: {
      AppUser : {
        email: { 
          required, 
          email 
        },
        first_name: { 
          required, 
          maxLength: maxLength(AppUserDefinitions.first_name.maxLength)
        },
        last_name: { 
          required, 
          maxLength: maxLength(AppUserDefinitions.last_name.maxLength)
        },
        username: {
          required, 
          maxLength: maxLength(AppUserDefinitions.username.maxLength)
        },
        password: {
          required, 
          maxLength: maxLength(AppUserDefinitions.password.maxLength)
        }
      }
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error']),
      emailErrors () {
        const errors = []
        if (!this.$v.AppUser.email.$dirty) return errors
        !this.$v.AppUser.email.email && errors.push(messages_en.email_valid)
        !this.$v.AppUser.email.required && errors.push(messages_en.email_required)
        return errors
      },
      firstNameErrors () {
        const errors = []
        if (!this.$v.AppUser.first_name.$dirty) return errors
        !this.$v.AppUser.first_name.maxLength && errors.push(messages_en.first_name_max_length_valid)
        !this.$v.AppUser.first_name.required && errors.push(messages_en.first_name_required)
        return errors
      },
      lastNameErrors () {
        const errors = []
        if (!this.$v.AppUser.last_name.$dirty) return errors
        !this.$v.AppUser.last_name.maxLength && errors.push(messages_en.last_name_max_length_valid)
        !this.$v.AppUser.last_name.required && errors.push(messages_en.last_name_required)
        return errors
      },
      usernameErrors () {
        const errors = []
        if (!this.$v.AppUser.username.$dirty) return errors
        !this.$v.AppUser.username.maxLength && errors.push(messages_en.username_max_length_valid)
        !this.$v.AppUser.username.required && errors.push(messages_en.username_required)
        return errors
      },
      passwordErrors () {
        const errors = []
        if (!this.$v.AppUser.password.$dirty) return errors
        !this.$v.AppUser.password.maxLength && errors.push(messages_en.password_max_length_valid)
        !this.$v.AppUser.password.required && errors.push(messages_en.password_required)
        return errors
      },
    },
    methods: {
      ...mapActions('userStore',['register', 'cleanAll']),
      onSubmit(){
        this.register(this.AppUser);
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
          this.$v.$reset();
          this.cleanAll();
          this.$router.push({name: 'home'});
        }
      },
      submit() {
        this.$v.$touch();
        if(!this.$v.$invalid){
          this.onSubmit();
        }
      },
    }
  }
</script>
<style scoped>
    .form-container{
        width: 100%;
        height: 550px;
        max-width: 330px;
        padding: 15px;
        margin: 0 auto;
    }
</style>