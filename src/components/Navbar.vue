<script setup lang="ts">
import {PackageMinus, Repeat, Settings,Crop} from 'lucide-vue-next'
import {ref} from "vue";
import {useRoute} from "vue-router";

const route = useRoute()

console.log(route.path)

interface NavItem {
  title: string
  icon: any
  href: string
}

const navs = ref<NavItem[]>([
  {
    title: 'Dashboard',
    icon: PackageMinus,
    href: '/'
  },
  {
    title: 'Converter',
    icon: Repeat,
    href: '/converter'
  },{
    title: 'Crop',
    icon: Crop,
    href: '/crop'
  },
  {
    title: 'Settings',
    icon: Settings,
    href: '/settings'
  }
])

</script>

<template>
  <div class="fixed inset-y-0 left-0 h-full flex flex-col items-center justify-center p-4 gap-4">
    <div class="bg-white flex flex-col items-center justify-center p-2 gap-4 rounded-full shadow-2xl">
      <div v-for="nav in navs" :key="nav.href" class="p-2 rounded-full transition-all ease-in-out"
           :class="{'bg-zinc-200':nav.href==route.path,'bg-transparent':nav.href!=route.path}">
        <router-link :to="nav.href" class="block text-zinc-600">
          <component :is="nav.icon"></component>
        </router-link>
      </div>
    </div>

  </div>
</template>

<style scoped>

</style>