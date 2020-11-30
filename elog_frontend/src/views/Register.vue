<template>
  <div class="container home text-center">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="eLog Expenses System"/>
    <Signup/>
    <Footer/>
  </div>
</template>

<script>
// @ is an alias to /src
import HelloWorld from '@/components/HelloWorld.vue'
import Signup from '@/components/Signup.vue'
import Footer from '@/components/Footer.vue'

export default {
  name: 'home',
  components: {
    HelloWorld,
    Signup,
    Footer
  }
}
</script>