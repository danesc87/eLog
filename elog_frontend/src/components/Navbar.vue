<template>
  <v-app>
    <v-navigation-drawer
      v-model="drawer"
      :mini-variant.sync="mini"
      permanent
    >
      <v-list-item class="px-2 pt-2" position="center center">

        <v-img v-bind:max-width="mini?40:100" src="https://smartmulticlipboard.com/images/ss/icons.png"></v-img>

        <v-btn
          icon
          class="ml-auto"
          @click.stop="mini = !mini"
        >
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
      </v-list-item>

      <v-list-item>
        <v-list-item-title v-if="!mini && user">{{user.username}}</v-list-item-title>
      </v-list-item>

      <v-divider></v-divider>

      <v-list dense>

        <v-list-item @click.prevent="">
          <v-list-item-icon>
            <v-icon>mdi-home-city</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>Home</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item @click.prevent="">
          <v-list-item-icon>
            <v-icon>mdi-account</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>My Account</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item @click.prevent="">
          <v-list-item-icon>
            <v-icon>mdi-account-multiple</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>Users</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

        <v-list-item @click.prevent="logout()">
          <v-list-item-icon>
            <v-icon>mdi-logout</v-icon>
          </v-list-item-icon>
          <v-list-item-content>
            <v-list-item-title>Logout</v-list-item-title>
          </v-list-item-content>
        </v-list-item>

      </v-list>
    </v-navigation-drawer>
  </v-app>
</template>

<script>
  import { mapActions, mapState } from 'vuex';

  export default {
    data() {
      return {
        drawer: true,
        mini: true,
      };
    },
    computed: {
      ...mapState('userStore',['loading', 'successMsg', 'errorMsg', 'processState', 'error', 'logged', 'user'])
    },
    methods: {
      ...mapActions('userStore',['logout'])
    }
  }
</script>

<style>
</style>