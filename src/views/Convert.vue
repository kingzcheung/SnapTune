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
import { get_file_name, ext } from "../utils";
import { open, message } from '@tauri-apps/api/dialog';
import { homeDir } from '@tauri-apps/api/path';


import {
  CheckCircleIcon,
  XCircleIcon,
  ExclamationCircleIcon,
} from "@heroicons/vue/24/solid";;
import UploadEmpty from "../components/UploadEmpty.vue";

const files = ref([]);
const formats = ref([
  "PNG",
  "JPEG",
  "GIF",
  "BMP",
  "TIFF",
  "WebP",
]);

// 选择输出的格式
const outputFormat = ref("PNG");

const output_dir = ref("");
const is_custom_dir = ref(false)



// 转换的加载动画状态
const convertLoading = ref(false);

const selectFormat = (format) => {
  outputFormat.value = format;
};

const onClick = () => {
  if (files.value.length == 0) {
    message('直接将图片拖拽到窗口即可。', { title: '请选择压缩的图片', type: 'error' });
    return;
  }

  if (!outputFormat.value) {
    message('支持常用格式，如: JPEG/PNG/WEBP/GIF/BMP', { title: '请选择压缩格式', type: 'error' });
    return;
  }

  if (convertLoading.value) {
    return;
  }

  convertLoading.value = true;

  files.value.forEach((file, index) => {
    invoke("image2x_command", {
      index: index,
      outputFormat: outputFormat.value,
      source: file.file,
      sourceFormat: ext(file.file)
    })
      .then((index) => {
        files.value[index].status = 1;
      })
      .catch((error) => console.error(error))
      .finally(() => {
        convertLoading.value = false;
      });
  });
};

const chooseDir = async (event) => {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await homeDir(),
  });
  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single directory
    output_dir.value = selected;
  }
}

const deleteItem = (index) => {
  files.value.splice(index, 1);
};

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
  <div class="convert text-sm">
    <div class="border-l-4 pl-2 leading-8 font-bold border-indigo-500 mb-2 text-lg">通用格式转换</div>
    <div class="
        w-full
        flex
        items-stretch
      ">
      <div class="flex-1 pl-1 text-sm my-2">
        <div class="flex items-start justify-start">
          <span class="my-2.5 mr-2">选择格式: </span>
          <div class="">
            <div class="flex items-center dropdown dropdown-right dropdown-start">
              <label tabindex="0" class="btn btn-sm m-1">{{
                  outputFormat ? `${outputFormat}` : "选择格式"
              }}</label>
              <div tabindex="0" class="
            dropdown-content
            card card-compact
            w-96
            p-2
            shadow
            bg-black
            text-primary-content
          ">
                <div class="card-body">
                  <div class="grid grid-cols-3 gap-4">
                    <button @click="selectFormat(format)" v-for="format in formats" :key="format"
                      class="btn text-md btn-sm" :class="{ 'btn-active btn-accent': format === outputFormat }">
                      {{ format }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="flex items-center justify-start">
          <span class="my-2.5 mr-2">输出目录: </span>
          <div class="flex items-center">
            <label class="cursor-pointer flex items-center" @click="is_custom_dir = false">
              <input type="radio" name="radio-6" class="radio checked:bg-gray-500 scale-75" :checked="!is_custom_dir" />
              <span class="label-text  justify-self-start">原文件夹</span>
            </label>
            <div class="flex flex-1">
              <label class="cursor-pointer flex items-center" @click="is_custom_dir = true">
                <input type="radio" name="radio-6" class="radio checked:bg-gray-500 scale-75"
                  :checked="is_custom_dir" />
                <span class="label-text mr-1">自定义</span>
              </label>
              <input v-if="is_custom_dir" type="text" placeholder="请输入目录" v-model="output_dir"
                class="input input-bordered input-sm w-full max-w-xs mr-1 flex-1" />
              <button v-if="is_custom_dir" class="btn btn-sm" @click="chooseDir">选择目录</button>
            </div>
          </div>
          <button class="btn btn-sm btn-primary ml-4" @click="onClick">
            <span>转 换</span>
            <!-- SwitchHorizontalIcon -->
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24"
              stroke="currentColor" stroke-width="2">
              <path stroke-linecap="round" stroke-linejoin="round"
                d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
          </button>

        </div>

      </div>
    </div>
    <div class="overflow-y-auto bg-white  rounded-lg" v-if="files.length > 0">
      <div class="flex item-center p-3 align-middle" :class="{ 'bg-base-100': i % 2 == 0 }" v-for="(file, i) of files"
        :key="file.file">
        <div class="truncate w-80 flex-1">{{ file.filename }}</div>
        <div class="px-2">{{ file.type }}</div>
        <div class="px-4 self-center">
          <check-circle-icon v-if="file.status == 1" class="h-5 w-5 text-green-500"></check-circle-icon>
          <x-circle-icon v-if="file.status == -1" class="h-5 w-5 text-red-500"></x-circle-icon>
          <exclamation-circle-icon v-if="file.status == 0" class="h-5 w-5 text-gray-500"></exclamation-circle-icon>
        </div>
        <div class="ml-2">
          <button class="btn btn-xs btn-link" @click="deleteItem(i)">
            删除
          </button>
        </div>
      </div>
    </div>
    <div class="flex items-center justify-center py-20 bg-white rounded-lg" v-else>
      <upload-empty></upload-empty>
    </div>
  </div>
</template>

