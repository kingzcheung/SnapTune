<script setup lang="ts">
import Navbar from "@/components/Navbar.vue";
</script>
<template>
  <div class="h-full relative">
    <Navbar></Navbar>
    <div class="pl-24 p-12  h-full">
      <router-view />
    </div>
  </div>
</template>