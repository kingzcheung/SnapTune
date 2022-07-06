<script setup>
import { onMounted, ref, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { get_file_name,ext } from "../utils";
import {
  UploadIcon
} from "@heroicons/vue/solid";
const files = ref([]);

onMounted(() => {
  listen("tauri://file-drop", (event) => {
    console.log(event.payload);
    files.value = event.payload.map((v) => {
      return {
        filename: get_file_name(v),
        file: v,
        status: 0,
        type: ext(v) + " 图像",
      };
    });
  });
});
</script>
<template>
  <div class="compression ">
    <h1 class="text-2xl divide-white divide-y-2 mb-2">JPEG/PNG 压缩</h1>
    <div class="h-px bg-base-300 dark:bg-gray-600	mb-2"></div>
    <div class="overflow-y-auto -m-3 text-sm	" v-if="files.length > 0">
      <div class="flex item-center justify-between p-3 align-middle" :class="{ 'bg-base-100': i % 2 == 0 }"
        v-for="(file,i) of files" :key="i">
        <div class="truncate max-w-xs	">{{file.filename}}</div>
        <div class="truncate w-20 pl-2">{{file.type}}</div>
        <div class="truncate px-2 w-20 text-sm">33.20KB</div>
        <div class="truncate pl-3 w-40 text-green-400 text-sm	">16.55KB (-50.45%)</div>
      </div>
    </div>
      <div class="flex justify-center content-center pt-32" else>
        <div class="w-40 h-40 border-dashed	border-2 flex justify-center items-center">
          <upload-icon class="h-24 w-24 text-gray-500"></upload-icon>
        </div>
      </div>

  </div>
</template>