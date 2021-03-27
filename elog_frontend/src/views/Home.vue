<template>
  <v-main class="text-center pt-5">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="eLog Expenses Management System" class="pt-5"/>
    <Login/>
    <Footer/>
  </v-main>
</template>

<script>
// @ is an alias to /src
import HelloWorld from '@/components/HelloWorld.vue'
import Login from '@/components/Login.vue'
import Footer from '@/components/Footer.vue'

export default {
  name: 'home',
  components: {
    HelloWorld,
    Login,
    Footer
  }
}
</script>
