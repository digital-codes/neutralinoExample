<script setup lang="ts">


import HelloWorld from './components/HelloWorld.vue'
import Calendar from "./components/Calendar.vue"

// Import filesystem namespace
import { filesystem } from "@neutralinojs/lib"
import { onMounted, ref } from 'vue';  

const msg = ref("123")

onMounted(() => {
  filesystem.readDirectory('./').then((data) => {
      console.log("Files:",data)
      msg.value = "Files read successfully"
    }).catch((err) => {
      console.log("Error:",err)
      msg.value = "Error reading files"
    })
});


</script>

<template>
  <div>
    <Calendar></Calendar>
    </div>
  <p>Message {{ msg }}</p>
  <HelloWorld msg="Vite + Vue" />

</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
