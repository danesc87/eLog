<template>
    <!-- <small class="footer text-center text-muted">
      <footer class="page-footer font-small green pt-5">
            <div class="footer-copyright text-center py-3"><span class="copyleft">&copy;</span> 2021 GPL v3: 
                <a href="https://nano-bytes.com/"> Nano-Bytes</a>
            </div>
        </footer>
    </small> -->
    <v-footer color="transparent" absolute class="aling-self-end">
        <v-col
        class="text-center"
        cols="12"
        >
            <a href="https://nano-bytes.com/" color="success">
                <span class="copyleft">&copy;</span> {{ new Date().getFullYear() }} — <strong>Nano-Bytes</strong>
            </a>
        </v-col>
    </v-footer>
</template>

<script>

</script>

<style scoped>
    a {
        color: #28a745 !important;
        text-decoration: none;
        font-size: small;
    }
    a:hover {
        color: gray !important;
    }
    .copyleft {
        display: inline-block;
        transform: rotate(180deg);
    }
</style>