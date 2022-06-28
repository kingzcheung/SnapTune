<script setup>
import { ref } from "vue";

  const currentTab = ref(0);
  const tabData = ref([
    '图片转换器',
    'Tab2',
    'Tab3',
  ])

  const clickTab = (i)=>{
    currentTab.value = i;
  }

</script>

<template>
  <div class="tabs tabs-boxed">
    <a class="tab" @click="clickTab(i)" :class="{'tab-active': i == currentTab}" v-for="(t,i) of tabData" :key="t">{{t}}</a> 
  </div>
</template>

