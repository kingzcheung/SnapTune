<!--
 Copyright 2022 kingzcheung
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

<script setup>
import { onMounted, ref, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { get_file_name, ext, get_file_size } from "../utils";
// import { DocumentArrowUpIcon } from "@heroicons/vue/24/solid";
import UploadEmpty from "../components/UploadEmpty.vue";
const files = ref([]);


const proportion = (v1, v2) => {
  let p = (v2 - v1) / v1 * 100

  return p.toFixed(2)
}

onMounted(() => {
  listen("tauri://file-drop", (event) => {
    console.log(event.payload);
    files.value = event.payload.map((v) => {
      return {
        filename: get_file_name(v),
        file: v,
        status: 0,
        type: ext(v) + " 图像",
        file_size: 0,
        opt_file_size: 0,
        file_size_text: '',
        opt_file_size_text: '',
      };
    });
    invoke("file_metadata_command", { files: event.payload })
      .then((meta) => {
        if (meta && meta.length > 0) {
          files.value = files.value.map((file) => {
            let fsize = "";
            let file_size = 0;
            for (let m of meta) {
              if (m.file == file.file) {
                fsize = get_file_size(m.file_size);
                file_size = m.file_size;
                break;
              }
            }
            file.file_size_text = fsize;
            file.file_size = file_size;
            return file;
          });
        }
      })
      .catch((error) => console.error(error));

    files.value.forEach(v => {
      invoke('image_optimize_command', { filename: v.file, level: 7 })
        .then((size) => {
          v.opt_file_size = size
          v.opt_file_size_text = get_file_size(size)
        })
    })


  });
});
</script>
<template>
  <div class="compression">
    <div class="border-l-4 pl-2 leading-8 font-bold border-indigo-500 mb-2 text-lg">JPEG/PNG 压缩</div>
    <div class="overflow-y-auto text-sm bg-white rounded-lg p-4" v-if="files.length > 0">
      <div class="flex item-center justify-between p-3 align-middle">
        <div class="truncate max-w-xs w-40">文件</div>
        <div class="truncate w-20 pl-2">类型</div>
        <div class="truncate px-2 w-20 text-sm">优化前</div>
        <div class="truncate pl-3 w-40 text-sm">
          优化后
        </div>
      </div>
      <div class="flex item-center justify-between p-3 align-middle" :class="{ 'bg-base-100': i % 2 == 0 }"
        v-for="(file, i) of files" :key="i">
        <div class="truncate max-w-xs w-40">{{ file.filename }}</div>
        <div class="truncate w-20 pl-2">{{ file.type }}</div>
        <div class="truncate px-2 w-20 text-sm">{{ file.file_size_text }}</div>
        <div class="truncate pl-3 w-40 text-sm" :class="{
          'text-green-600': file.file_size > file.opt_file_size,
          'text-gray-600': file.file_size == file.opt_file_size,
          'text-red-600': file.file_size < file.opt_file_size,
        }">
          {{ file.opt_file_size_text }} ({{ proportion(file.file_size, file.opt_file_size) }}%)
        </div>
      </div>
    </div>
    <div class="flex items-center justify-center py-20 bg-white rounded-lg" v-else>
      <upload-empty>
        <template #content>
          支持 JPEG / PNG
        </template>
      </upload-empty>
    </div>
  </div>
</template>