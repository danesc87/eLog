<template>
  <v-app id="app">
    <router-view/>
  </v-app>
</template>

<style lang="scss">
@import url('https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap'); // font import
@import url('https://cdn.jsdelivr.net/npm/@mdi/font@5.9.55/css/materialdesignicons.min.css'); // Material Design

html, #app {
  background-color: var(--v-primary-base);
}
</style>
