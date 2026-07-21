<template>
  <div style="width: 100%; margin-left: auto; margin-right: auto;">

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
    <table style="border: 1px solid #999; display: block;">
      <tr>
        <td>
          time
        </td>
        <td>
            method
        </td>
        <td>
            subdomain
        </td>
        <td>
          url
        </td>
        <td>
          user
        </td>
        <td>
          <span>status</span>
        </td>
        <td>
          <span>timer</span>
        </td>
        <td>
            secs
        </td>
        <td>
         bot
        </td>
        <td>
          lead
        </td>
        <td>
          campaign
        </td>
        <td>
          Browser
        </td>
        <td>
          OS
        </td>
        <td>
            query
        </td>
        <td>
            host
        </td>
        <td>
          Referrer
        </td>
        <td>
            error
        </td>
        <td>
            forwarded
        </td>
        <td>
            x_forwarded_for
        </td>
        <td>
            x_real_ip
        </td>
        <td>
            client_addr
        </td>
        <td>
            client_ip
        </td>
        <td>
            client_port
        </td>

      </tr>

      <tr v-for="touch in filtered" v-if="touch && touch.url" @click="showJson(touch)" style="cursor: pointer;">
        <td>
          <span v-if="touch.id">
              {{ touch.id }}
          </span>
          <span v-else>
            {{ touch.id }}
          </span>
        </td>
        <td style="">
            {{ touch.method }}
        </td>
        <td>
            <span>
              {{ touch.subdomain }}
            </span>
        </td>
        <td style="width: 200px; max-width: 200px; ">
          <a v-if="touch.status === 404" v-bind:href="touch.path" target="_blank" style="color: #900; text-decoration: none;" @click.stop>
            {{ touch.url }}
          </a>
          <a v-else v-bind:href="touch.path" target="_blank" style="color: #000; text-decoration: none;" @click.stop>
            {{ touch.url }}
          </a>
        </td>
        <td style="">
          <span v-if="touch.anon_name">
            {{ touch.anon_name }}
          </span>
          <span v-else-if="touch.isAdmin" style="color: #000;">
            <div >
              admin
            </div>
          </span>
          <span v-else style="color: #999;">
              {{touch.ip}}
          </span>
        </td>
        <td>
          <a v-if="touch.status === 404" v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #900; text-decoration: none;" @click.stop>
            {{ touch.status }}
          </a>
          <a v-else v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #999; text-decoration: none;" @click.stop>
            {{ touch.status }}
          </a>
        </td>
        <td>
          <a v-bind:href="'/admin/touch/' + touch._id " target="_blank" style="color: #999; text-decoration: none;" @click.stop>
            {{ touch.timer }}
          </a>
        </td>

        <td style="">
          <span v-if="touch.counter">
            {{ touch.counter }}
          </span>
        </td>
        <td>
            <span v-if="touch.is_bot">
                🤖
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
            <span>
              {{ touch.query }}
            </span>
        </td>
        <td style="">
            {{ touch.host }}
        </td>
        <td style="width: 40px;">
          <span>
            {{ touch.referrer }}
          </span>
        </td>
        <td>
            <span>
              {{ touch.error }}
            </span>
        </td>
        <td>
            <span>
              {{ touch.forwarded }}
            </span>
        </td>
        <td>
            <span>
              {{ touch.x_forwarded_for }}
            </span>
        </td>
        <td>
            <span>
              {{ touch.x_real_ip }}
            </span>
        </td>
        <td>
            <span>
              {{ touch.client_addr }}
            </span>
        </td>
        <td>
            <span>
              {{ touch.client_ip }}
            </span>
        </td>
        <td>
            <span>
              {{ touch.client_port }}
            </span>
        </td>

      </tr>
    </table>
    <button v-if="limit" style="margin-left: auto; margin-right: auto; margin-top: 10px; width: 100%;" @click="showMore()">Show more</button>

    <!-- JSON overlay -->
    <div v-if="selected" class="json-overlay" @click.self="selected = null">
      <div class="json-box">
        <button class="json-close" @click="selected = null">×</button>
        <pre>{{ selectedJson }}</pre>
      </div>
    </div>

  </div>
</template>
<script>

  const _ = require('lodash')

  module.exports = {
    data: function () {
      return {
        e404: [],
        limit: true,
        filterName: 'actual',
        selected: null
      }
    },
    components: {
//      'footerelement': footer,
    },
    mounted: function () {
      const t = this
      document.addEventListener('keydown', this.onKeydown)
    },
    beforeDestroy: function () {
      document.removeEventListener('keydown', this.onKeydown)
    },
   // props: ['tracker'],
    computed: {
      sorted: function () {
        let t = this
        const sorted = _.orderBy(this.$store.inputFeed, ['id'], ['desc'])
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
              return one.counter && one.counter > -1
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
      selectedJson: function () {
        if (!this.selected) {
          return ''
        }
        return JSON.stringify(this.selected, null, 2)
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
      showJson: function (touch) {
        this.selected = touch
      },
      onKeydown: function (e) {
        if (e.key === 'Escape') {
          this.selected = null
        }
      }
    }
  }
</script>
<style scoped="">
  table td:hover {
     background-color: #eee;
  }
  .json-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 1000;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .json-box {
    position: relative;
    background: #fff;
    border: 1px solid #999;
    max-width: 80vw;
    max-height: 80vh;
    overflow: auto;
    padding: 20px;
  }
  .json-box pre {
    margin: 0;
    font-size: 12px;
    white-space: pre-wrap;
    word-break: break-all;
  }
  .json-close {
    position: absolute;
    top: 5px;
    right: 5px;
    border: none;
    background: none;
    font-size: 20px;
    cursor: pointer;
    color: #999;
  }
  .json-close:hover {
    color: #000;
  }
</style>
