<script setup lang="ts">
import Navbar from "@/components/Navbar.vue";
import {Store} from '@tauri-apps/plugin-store'
import {onMounted} from "vue";
import config from '@/config/index.json'

onMounted(async ()=>{
  const store = await Store.load('settings.json')
  // 如果是空对象
  if (Object.keys(store).length === 0) {
    await store.set("settings",config)
    await store.save()
    console.log(config)
  }

})

</script>
<template>
  <div class="h-full relative">
    <Navbar></Navbar>
    <div class="pl-24 p-12 bg-[#f3f3f3] h-full">
      <router-view />
    </div>
  </div>
</template>