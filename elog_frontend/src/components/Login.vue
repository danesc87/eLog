<template>
  <div class="form-container">
    <v-form
      @keyup.enter="submit"
      @submit.prevent="submit"
      @reset="onReset" v-if="show"
    >
        
        <div class="">
          <v-text-field 
            label="Username" 
            :error-messages="usernameErrors" 
            type="text" 
            required 
            v-model="username" 
            @input="$v.username.$touch()"
            @blur="$v.username.$touch()" 
          />
          
          <v-text-field 
            label="Password" 
            :error-messages="passwordErrors" 
            type="password" 
            v-model="password" 
            @input="$v.password.$touch()" 
            @blur="$v.password.$touch()" 
          />
        </div>
        
        <div class="text-center pt-5">
          <v-btn 
            type="submit"
            color="success" 
            outlined
          >
          Login
          </v-btn>
        </div>
        
        <div>
          <a class="text-muted" href="/register">
            <small>Register</small>
          </a>
        </div>
    
    </v-form>
    
    <div class="text-center pt-2" v-if="loading">
      <v-btn 
        color="success" 
        disabled
      >
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
  import { required } from 'vuelidate/lib/validators'
  import { messages_en } from '../localization'

  export default {
    data() {
      return {
        username: '',
        password: '',
        show: true
      }
    },
    validations: {
      username: { required },
      password: { required }
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error', 'logged']),
      usernameErrors () {
        const errors = []
        if (!this.$v.username.$dirty) return errors
        !this.$v.username.required && errors.push(messages_en.username_required)
        return errors
      },
      passwordErrors () {
        const errors = []
        if (!this.$v.password.$dirty) return errors
        !this.$v.password.required && errors.push(messages_en.password_required)
        return errors
      }
    },
    methods: {
      ...mapActions('userStore',['login', 'cleanAll']),
      async onSubmit(user) {
        this.login(user);
      },
      onReset(){
        this.username = '';
        this.password = '';
        this.cleanAll();
      },
      submit() {
        this.$v.$touch();
        if(!this.$v.$invalid){
          this.onSubmit(
            {
              'username': this.username, 
              'password': this.password
            }
          );
        }
      },
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