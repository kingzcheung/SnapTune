<script setup>
import { onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { CollectionIcon,CashIcon,ArchiveIcon,CubeTransparentIcon } from "@heroicons/vue/solid";
const route = useRoute();
const router = useRouter();
const currentTab = ref("Convert");
const tabData = ref([
  { key: "Convert", icon: CashIcon, name: "通用格式转换" },
  { key: "Compression", icon: ArchiveIcon, name: "图片压缩" },
  { key: "Rename", icon: CollectionIcon, name: "批量重命名" },
  { key: "Detection", icon: CollectionIcon, name: "商品裁剪" },
]);

const clickTab = (key) => {
  currentTab.value = key;
  router.push({
    name: key,
  });
};

onMounted(() => {
  for (let item of tabData.value) {
    if (route.matched[1]?.name == item.key) {
      currentTab.value = route.matched[1]?.name;
    }
  }
});
</script>
<template>
  <div
    class="relative h-full overflow-x-hidden overscroll-none flex border-gray-500"
  >
    <ul
      class="
        menu
        overflow-y-auto
        w-60
        text-base-content
        bg-base-100
        border-gray-500
        flex-none
        h-screen
        fixed
        left-0
        top-0
        bottom-0
        z-50
      "
    >
      <!-- Sidebar content here -->
      <li class="" v-for="t of tabData" :key="t.key">
        <a
          class="px-4 flex mb-1 text-sm item-center"
          href="javascript:;"
          @click="clickTab(t.key)"
          :class="{ 'bg-primary text-gray-50': t.key == currentTab }"
        >
          <component class="h-5 w-5 text-base-content" :is="t.icon"></component>
          <span class="ml-1">{{ t.name }}</span>
        </a>
      </li>
    </ul>

    <div class="ml-60 relative flex-1 p-4">

      <router-view></router-view>
    </div>
  </div>
</template>