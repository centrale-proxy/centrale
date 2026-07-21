import "@babel/polyfill";

import Vue from "vue"
import VueRouter from "vue-router"
import VueStash from 'vue-stash'
import axios from "axios"

import Wrapper from "./Wrapper"
import routes from "./routes"

import "billboard.js/dist/billboard.css";

Vue.use(VueStash)
Vue.use(VueRouter)

const router = new VueRouter({
  routes,
  scrollBehavior(to, from) {
    return { x: 0, y: 0 } // SCROLL TO TOP
  }
})

// MAKE AXIOS AVAILABLE IN VUE
Vue.prototype.$axios = axios


//axios.defaults.headers.common['Authorization'] = 'Bearer ' + token

new Vue({
  el: "#app",
  router,
  data: {
    store: {
      inputFeed: [],
    }
  },
  render: h => h(Wrapper)
})
