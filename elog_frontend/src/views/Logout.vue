<template>
  <div class="container home text-center pt-5">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="eLog Expenses System" class="pt-5"/>
    <Logout/>
    <Footer/>
  </div>
</template>

<script>
// @ is an alias to /src
import HelloWorld from '@/components/HelloWorld.vue'
import Logout from '@/components/Logout.vue'
import Footer from '@/components/Footer.vue'

export default {
  name: 'home',
  components: {
    HelloWorld,
    Logout,
    Footer
  }
}
</script>