<script setup>
import { onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { themeChange } from "theme-change";
import { CollectionIcon,CashIcon,ArchiveIcon,CubeTransparentIcon } from "@heroicons/vue/solid";
const route = useRoute();
const router = useRouter();
const currentTab = ref("Convert");
const tabData = ref([
  { key: "Convert", icon: CashIcon, name: "图片转换" },
  { key: "Compression", icon: ArchiveIcon, name: "图片压缩" },
  // { key: "DPI", icon: CollectionIcon, name: "修改图片尺寸(DPI)" },
  // { key: "SuperResolution", icon: CubeTransparentIcon, name: "超分辨率" },
  { key: "Rename", icon: CollectionIcon, name: "批量重命名" },
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
  themeChange(false);
});
</script>
<template>
  <div
    class="relative h-full overflow-x-hidden overscroll-none flex bg-base-200"
  >
    <ul
      class="
        menu
        overflow-y-auto
        w-60
        text-base-content
        bg-base-100
        border-gray-400
        flex-none
        h-screen
        fixed
        left-0
        top-0
        bottom-0
        z-50
        mt-10
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

    <div class="ml-60 relative flex-1 p-3">
      <label class="swap swap-rotate">
        <!-- this hidden checkbox controls the state -->
        <input
          type="checkbox"
          data-act-class="active"
          data-toggle-theme="dark,light"
          checked
        />

        <!-- sun icon -->
        <svg
          class="swap-on fill-current w-6 h-6"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
        >
          <path
            d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"
          />
        </svg>

        <!-- moon icon -->
        <svg
          class="swap-off fill-current w-6 h-6"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
        >
          <path
            d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"
          />
        </svg>
      </label>
      <router-view></router-view>
    </div>
  </div>
</template>