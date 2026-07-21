<template>
  <div id="main">
    <router-view></router-view>
  </div>
</template>

<script>
const MAX_FEED = 500
const TRIM_THRESHOLD = 600 // trim in batches, not on every message

export default {
  name: 'AppContainer',

  data() {
    return {
      admin: false
    }
  },

  created() {
    // Non-reactive helpers: declared here (not in data) so Vue doesn't
    // waste memory making them reactive.
    this.source = null
    this.feedIndex = new Map() // id -> array index
  },

  mounted() {
    this.getFeed()
  },

  beforeDestroy() {
    this.closeFeed()
  },

  methods: {
    getFeed() {
      if (!window.EventSource) {
        console.warn("Your browser doesn't support SSE")
        return
      }

      this.closeFeed() // guard against duplicate connections

      this.source = new EventSource('/api/feed')
      this.source.onmessage = this.onFeedMessage
      this.source.onerror = (e) => console.error('SSE error:', e)
    },

    closeFeed() {
      if (this.source) {
        this.source.close()
        this.source = null
      }
    },

    onFeedMessage(event) {
      let data
      try {
        data = JSON.parse(event.data)
      } catch (e) {
        console.error('Bad feed payload:', e)
        return
      }

      const feed = this.$store.inputFeed
      const existing = this.feedIndex.get(data.id)

      // Freeze display-only items so Vue skips deep reactivity on them —
      // roughly halves per-item memory. Remove if you mutate items in place.
      const item = Object.freeze(data)

      if (existing !== undefined) {
        this.$set(feed, existing, item)
        return
      }

      feed.push(item)
      this.feedIndex.set(item.id, feed.length - 1)

      if (feed.length > TRIM_THRESHOLD) {
        this.trimFeed(feed)
      }
    },

    trimFeed(feed) {
      // splice mutates in place instead of allocating a new array
      // (slice(-500) copied all 500 items on every message)
      feed.splice(0, feed.length - MAX_FEED)

      // Indices shifted, rebuild the map. Happens once per ~100 messages
      // instead of scanning 500 items on every single message.
      this.feedIndex.clear()
      for (let i = 0; i < feed.length; i++) {
        this.feedIndex.set(feed[i].id, i)
      }
    }
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
}

</style>
