<script setup lang="ts">


import HelloWorld from './components/HelloWorld.vue'
import Calendar from "./components/Calendar.vue"

// Import filesystem namespace
import { filesystem, os, app } from "@neutralinojs/lib"
import { onMounted, ref } from 'vue';  

const msg = ref("123")

onMounted(async () => {
  msg.value = "Mounted. Globals: " + window.NL_OS + " " + window.NL_ARCH + " " + window.NL_ARGS + " " + window.NL_CWD
  try {
    const config = await app.getConfig()
    console.log("App config:",config)
    msg.value += " //Config: " + JSON.stringify(config)
  } catch (error) {
    msg.value += "Error reading config"
    console.error("Error:", error);
  }
  try {
    const files = await filesystem.readDirectory('./')
    console.log("Files:",files)
    msg.value += " //Files read successfully" + JSON.stringify(files)
  } catch (error) {
    msg.value += "Error reading files"
    console.error("Error:", error);
  }
  try {
    const envs = await os.getEnvs();
    console.log(envs);
    msg.value += " //ENV: " + JSON.stringify(envs)
  } catch (error) {
    console.error("Error:", error);
    msg.value += error
  }

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
