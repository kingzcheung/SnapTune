<script setup>
import { onMounted, ref, watch } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
const currentTab = ref('Convert');
const tabData = ref([
  {key:"Convert",name:"图片转换器"},
  {key:"Compression",name:"图片压缩"},
  {key:"DPI",name:"修改图片尺寸(DPI)"},
  {key:"SuperResolution",name:"超分辨率(AI)"},
]);

const clickTab = (key) => {
  currentTab.value = key;
  router.push({
    name: key,
  })
};
</script>
<template>
  <div class="relative h-full overscroll-none">
    <div class="tabs tabs-boxed">
      <a
        class="tab"
        @click="clickTab(t.key)"
        :class="{ 'tab-active': t.key == currentTab }"
        v-for="t of tabData"
        :key="t.key"
      >
        {{ t.name }}
      </a>
    </div>
    <router-view></router-view>
  </div>
</template>