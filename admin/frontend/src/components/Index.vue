<style scoped>
</style>
<template>
    <div class="medium push">
        Centrale admin!
        <br><br>
        <lastrequests>
        </lastrequests>
    </div>
</template>
<script>

import lastrequests from './lastrequests.vue'

export default {
    name: 'Index',
    data() {
        return {
            productKey: ''
        }
    },
    components: {
      lastrequests,
    },
    mounted() {

    },
    methods: {
        next() {

        }
    }
}
</script>
