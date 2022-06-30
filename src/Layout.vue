<script setup>
import { onMounted, ref, watch } from "vue";
import { useRouter } from "vue-router";
const router = useRouter();
const currentTab = ref('Convert');
const tabData = ref([
  {key:"Convert",name:"图片转换"},
  {key:"Compression",name:"JPEG/PNG 压缩"},
  {key:"DPI",name:"修改图片尺寸(DPI)"},
  {key:"SuperResolution",name:"超分辨率(AI)"},
  {key:"Rename",name:"批量重命名"},
]);

const clickTab = (key) => {
  currentTab.value = key;
  router.push({
    name: key,
  })
};
</script>
<template>
  <div class="relative h-full overflow-x-hidden overscroll-none flex">
  <ul class="menu p-4 overflow-y-auto w-60 text-base-content bg-base-300 border-gray-400 border-r-2	flex-none">
      <!-- Sidebar content here -->
      <li v-for="t of tabData" :key="t.key">
       <a class="block mb-1 text-sm	"
        href="javascript:;"
        @click="clickTab(t.key)"
        :class="{ 'bg-primary text-gray-50': t.key == currentTab }" >
        {{ t.name }}
      </a>
      </li>
      
    </ul>

    <div class="relative flex-1 p-2">
      <router-view></router-view>
    </div>
  </div>
</template>