<script>
  import App from './App'

  export default {
    name: "wrapper",
    data: function () {
      return {
        incomingInvoiceData: {}
      }
    },
    computed: {
      isGeneratorActive: function () {
        return JSON.parse((this.$route.query['isGeneratorActive'] || false))
      }
    },
    components: {
      App
    }
  }
</script>
<template>
  <div>
    <router-view 
      v-if="isGeneratorActive"
      :incomingInvoiceData="incomingInvoiceData"></router-view>
    <App v-else/>
  </div>
</template>
