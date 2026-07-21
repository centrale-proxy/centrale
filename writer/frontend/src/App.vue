<template>
  <div id="main">
    <router-view></router-view>
  </div>
</template>
<script>
  import axios from 'axios'

  export default {
    name: "AppContainer",
    data () {
      return {
        admin: false
      }
    },
    mounted() {
//      this.getAndSetUser()
//      this.getAndSetCompany()
 //     this.getProducts()
    },
    components: {
    },
    methods: {
      getAndSetUser: function () {
        let t = this
        axios
          .get('/api/user')
          .then(({data}) => {
            this.$set(this.$store, 'user', data.user || {})
            if (data.user && data.user.roles && data.user.roles.indexOf('admin') > -1) {
              t.admin = true
            }
          })
          .catch((error) => {
            t.$router.push('/login')
            console.log(error)
          })
      },
    }
  }
</script>

<style>

html {
  height: 100%;
}

body {
  font-family: Roboto, 'Segoe UI', Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  height: 100%;
  margin: 0;
  padding: 0;
  font-size: 18px;
}

pre {
  text-align: left;
  white-space: pre-line;
  font-family: Consolas, Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace, serif;
  margin-bottom: 10px;
  overflow: auto;
  width: auto;
  padding: 5px;
  background-color: #eee;
  width: 650px!ie7;
  padding-bottom: 20px!ie7;
  max-height: 600px;
  font-size: 15px;
}

pre.small {
  white-space: pre;
  font-size: 12px;
}

.green {
  color: #080;
}

.medium {
  width: 800px;
  max-width: 100%;
  margin-left: auto;
  margin-right: auto;
  padding: 15px;
  margin-bottom: 200px;
}
.push {
  margin-bottom: 800px;
  padding: 20px;
}

.red {
  background: rgb(220, 20, 60);
  color: rgb(255, 255, 255);
  padding: 10px 15px;
}

.gray {
  padding: 10px 15px;
}

.cancel {
  float: right;margin-right: 35px;text-decoration: none;color: #999;
}

.back {
  text-decoration: none;
  color: #000;
  font-weight: bold;
}

.button {
  cursor: pointer; border: 1px solid #eee; padding: 15px; width: 80%;
}

.hundred {
  width: 80%;
  font-size: 12pt;
  height: 20pt;
  padding: 3px 10px 3px 10px;
}

.right {
  float: right; margin-right: 35px;
}

.linkbutton {
  width: 80%; display: block; text-decoration: none; height: 50px; color: #000; text-align: center;
}

#main {
  overflow: hidden;
}

</style>
