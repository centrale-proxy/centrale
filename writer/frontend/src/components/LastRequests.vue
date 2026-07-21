<template>
  <div style="width: 100%; max-width: 1000px; margin-left: auto; margin-right: auto;">

    <br>
    <button v-if="filterName === 'actual'" style="color: #000;" @click="filterName = 'actual'">
      Actual visitors
    </button>
    <button v-else style="color: #999;" @click="filterName = 'actual'">
      Actual visitors
    </button>
    &nbsp;&nbsp;
    &nbsp;&nbsp;
    <button v-if="filterName === '404'" style="color: #000;" @click="filterName = '404'">
      404
    </button>
    <button v-else style="color: #999;" @click="filterName = '404'">
      404
    </button>
    &nbsp;&nbsp;
    &nbsp;&nbsp;
    <button v-if="filterName === 'all'" style="color: #000;" @click="filterName = 'all'">
      All visitors
    </button>
    <button v-else style="color: #999;" @click="filterName = 'all'">
      All visitors
    </button>

    <br><br>
    <table style=" overflow: hidden; border: 1px solid #999; display: block;">
      <tr>
        <td>
          time
        </td>
        <td>
          url
        </td>
        <td>
          user
        </td>
        <td>
          seconds
        </td>
        <td>
          event
        </td>
        <td>
          lead
        </td>
        <td>
          Campaign
        </td>
        <td>
          Browser
        </td>
        <td>
          OS
        </td>
        <td>
          <span>status</span>
        </td>
        <td>
          <span>timer</span>
        </td>
        <td>
          Referrer
        </td>
      </tr>

       <tr v-for="touch in filtered" v-if="touch && touch.url">
        <td>
          <span v-if="touch.id">
            <a v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #000; text-decoration: none;">
              {{ touch.checkin }}
            </a>
          </span>
          <span v-else>
            {{ touch.checkin }}
          </span>
        </td>
        <td style="width: 200px; max-width: 200px; overflow: hidden;">
          <a v-if="touch.status === 404" v-bind:href="touch.path" target="_blank" style="color: #900; text-decoration: none;">
            {{ touch.url }}
          </a>
          <a v-else v-bind:href="touch.path" target="_blank" style="color: #000; text-decoration: none;">
            {{ touch.url }}
          </a>
        </td>
        <td style="">
          <span v-if="touch.user">
            {{ touch.user }}
          </span>
          <span v-else-if="touch.isAdmin" style="color: #000;">
            <div >
              admin
            </div>
          </span>
          <span v-else style="color: #999;">
            <div v-if="touch.ip && userNames[touch.ip]">
              {{ userNames[touch.ip] }}
            </div>
          </span>
        </td>
        <td style="">
          <span v-if="touch.ping">
            {{ touch.ping }}
          </span>
        </td>
        <td style="width: 40px;">
          <span v-if="touch.body && touch.body.event">
            <span :title="touch.body.event">!!!</span>
          </span>
        </td>
        <td style="width: 40px;">
          <span v-if="touch.lead">
            <span>{{ touch.lead }}</span>
          </span>
        </td>
        <td>
          <span>
            {{ touch.campaign }}
          </span>
        </td>
        <td>
          <span>
            {{ touch.browser }}
          </span>
        </td>
        <td>
          <span>
            {{ touch.os }}
          </span>
        </td>
        <td>
          <a v-if="touch.status === 404" v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #900; text-decoration: none;">
            {{ touch.status }}
          </a>
          <a v-else v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #999; text-decoration: none;">
            {{ touch.status }}
          </a>
        </td>
        <td>
          <a v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #999; text-decoration: none;">
            {{ touch.timer }}
          </a>
        </td>
        <td style="width: 40px;">
          <span>
            {{ touch.referrer }}
          </span>
        </td>
      </tr>
    </table>
    <button v-if="limit" style="margin-left: auto; margin-right: auto; margin-top: 10px; width: 100%;" @click="showMore()">Show more</button>
  </div>
</template>
<script>

  const _ = require('lodash')

  module.exports = {
    data: function () {
      return {
        e404: [],
        limit: true,
        filterName: 'actual'
      }
    },
    components: {
//      'footerelement': footer,
    },
    mounted: function () {

    },
   // props: ['tracker'],
    computed: {
      sorted: function () {
        let t = this
        //const sorted = _.orderBy(this.tracker, ['checkin'], ['desc'])
        const sorted = t.$store.inputFeed;
        /*
        let all = []
        let i = 0
        _.each(sorted, function (one) {
          if (t.limit) {
            if (i < 10) {
              all.push(one)
              i++
            }
          } else {
            all.push(one)
          }
        })
        */
        return sorted
      },
      filtered: function () {

        const t = this

        let filtered = []
        if (this.filterName === 'all') {
          filtered = this.sorted
        } else if (this.filterName === '404') {
          filtered = this.sorted.filter(function (one) {
            return one.status && one.status === 404
          })
        } else if (this.filterName === 'actual') {
          filtered = this.sorted.filter(function (one) {
            if (one.url === '/api/visitor') {
              return one
            } else {
              return one.ping && one.ping > -1
            }
          })
        } else {
          console.log('no filter')
        }


        let all = []
        let i = 0
        _.each(filtered, function (one) {
          if (t.limit) {
            if (i < 10) {
              all.push(one)
              i++
            }
          } else {
            all.push(one)
          }
        })

        return all
      },
      userNames: function () {

        const a = [
          'Allan',
          'Bob',
          'Cameron',
          'Dietrich',
          'Eevald',
          'Fjodor',
          'Gunita',
          'Harry',
          'Ivar',
          'Janis',
          'Kalle',
          'Larry',
          'Mary',
          'Nina',
          'Oskar',
          'Peeter',
          'Raissa',
          'Stephen',
          'Thomas',
          'Uugatsagga',
          'Välek'
        ]
        const grouped = _.groupBy(this.tracker, 'ip')
        let names = {}
        let i = 0
        _.each(grouped, function (one, key) {
          if (!a[i]) {
            i = 0
          }
          names[key] = a[i]
          i++
        })
        return names
      }
    },
    methods: {
      showMore: function () {
        this.limit = false
      },
    }
  }
</script>
<style scoped="">
  table td:hover {
     background-color: #eee;
  }
</style>
