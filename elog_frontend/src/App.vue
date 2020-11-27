<template>
  <div id="app">

       <!-- <b-navbar toggleable="md" type="dark" variant="dark">
         <b-container>
           <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
           <b-navbar-brand href="#">NavBar</b-navbar-brand>
           <b-collapse id="nav-collapse" is-nav>
            <b-navbar-nav>
              <b-nav-item :to="{name: 'home'}" exact>Home</b-nav-item>
              <b-nav-item :to="{name: 'about'}">About</b-nav-item>
              <b-nav-item :to="{name: 'grid'}">Grid</b-nav-item>
              <b-nav-item :to="{name: 'cards'}">Cards</b-nav-item>
              <b-nav-item :to="{name: 'form'}">Forms</b-nav-item>
              <b-nav-item :to="{name: 'validaciones'}">Validaciones</b-nav-item>
            </b-navbar-nav>    
           </b-collapse>
         </b-container>
       </b-navbar> -->

       <b-container>
         <router-view/>
       </b-container>
  </div>
</template>

<style>
  body, html, #app {
    background-color: #E1E2E1;
    height: 100%;
    min-height: 803px;
  }
</style>
