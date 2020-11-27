<template>
  <div class="hello text-center">
    <h1>{{ msg }}</h1>
    <p>
      Welcome to eLog, a system designed to help you managing your expenses<br>Please visit our website
      <a href="https://blog.nano-bytes.com" target="_blank" rel="noopener"> nano-bytes.com</a>.
    </p>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
