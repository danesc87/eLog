<template>
  <div class="form-container">
    <b-form @submit="onSubmit" @reset="onReset" v-if="show">
      <b-form-group id="input-group-1" label="Email:" label-for="email">
        <b-form-input
          id="email"
          v-model="form.email"
          required
          placeholder="Enter email"
        ></b-form-input>
      </b-form-group>
      <b-form-group id="input-group-1" label="Full Name:" label-for="name">
        <b-form-input
          id="firstname"
          v-model="form.firstname"
          required
          placeholder="First Name"
        ></b-form-input>

        <b-form-input
          id="lastname"
          v-model="form.lastname"
          required
          placeholder="Last Name"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-2" label="Username:" label-for="username">
        <b-form-input
          id="username"
          v-model="form.username"
          required
          placeholder="Enter username"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-3" label="Password:" label-for="text-password">
        <b-form-input
          id="input-5"
          type="password"
          v-model="form.password"
          required
          placeholder="Enter password"
        ></b-form-input>
      </b-form-group>
      <div>
          <b-button type="submit" variant="outline-success">Submit</b-button>
      </div>
    </b-form>
  </div>
</template>

<script>
  export default {
    data() {
      return {
        form: {
            email: '',
            firstname: '',
            lastname: '',
            username: '',
            password: '',
        },
        show: true
      }
    },
    methods: {
      onSubmit(evt) {
        evt.preventDefault()
        alert(JSON.stringify(this.form))
      },
      onReset(evt) {
        evt.preventDefault()
        // Reset our form values
        this.form.username = ''
        this.form.password = ''
        // Trick to reset/clear native browser form validation state
        this.show = false
        this.$nextTick(() => {
          this.show = true
        })
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