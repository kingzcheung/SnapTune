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
import { open,message } from '@tauri-apps/api/dialog';
import { homeDir } from '@tauri-apps/api/path';

import {
  CheckCircleIcon,
  XCircleIcon,
  ExclamationCircleIcon,
} from "@heroicons/vue/solid";
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
    <div class="flex items-center justify-center py-40 bg-white rounded-lg" v-else>
      <upload-empty></upload-empty>
    </div>
    <div class="
        absolute
        z-50
        left-0
        right-0
        bottom-0
        bg-gray-200
        w-full
        flex
        items-stretch
        border-t
        border-gray-300	
        border-solid
      ">
      <div class="flex-1 pl-1 text-sm my-2">
        <div class="flex items-start justify-start">
          <span class="my-2.5 mr-2">选择格式: </span>
          <div class="">
            <div class="flex items-center dropdown dropdown-right dropdown-end">
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
          <div class="flex-1 flex items-center">
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
        </div>

      </div>
      <div class="flex items-center px-1 bg-primary ml-2 cursor-pointer" @click="onClick">
        <span class="text-primary-content font-bold px-8 text-2xl">转换</span>
        <svg v-if="convertLoading" t="1656508933337" class="icon animate-spin" viewBox="0 0 1024 1024" version="1.1"
          xmlns="http://www.w3.org/2000/svg" p-id="1306" width="24" height="24">
          <path
            d="M834.7648 736.3584a5.632 5.632 0 1 0 11.264 0 5.632 5.632 0 0 0-11.264 0z m-124.16 128.1024a11.1616 11.1616 0 1 0 22.3744 0 11.1616 11.1616 0 0 0-22.3744 0z m-167.3216 65.8944a16.7936 16.7936 0 1 0 33.6384 0 16.7936 16.7936 0 0 0-33.6384 0zM363.1616 921.6a22.3744 22.3744 0 1 0 44.7488 0 22.3744 22.3744 0 0 0-44.7488 0z m-159.744-82.0224a28.0064 28.0064 0 1 0 55.9616 0 28.0064 28.0064 0 0 0-56.0128 0zM92.672 700.16a33.6384 33.6384 0 1 0 67.2256 0 33.6384 33.6384 0 0 0-67.2256 0zM51.2 528.9984a39.168 39.168 0 1 0 78.336 0 39.168 39.168 0 0 0-78.336 0z m34.1504-170.0864a44.8 44.8 0 1 0 89.6 0 44.8 44.8 0 0 0-89.6 0zM187.904 221.7984a50.432 50.432 0 1 0 100.864 0 50.432 50.432 0 0 0-100.864 0zM338.432 143.36a55.9616 55.9616 0 1 0 111.9232 0 55.9616 55.9616 0 0 0-111.9744 0z m169.0112-4.9152a61.5936 61.5936 0 1 0 123.2384 0 61.5936 61.5936 0 0 0-123.2384 0z m154.7776 69.632a67.1744 67.1744 0 1 0 134.3488 0 67.1744 67.1744 0 0 0-134.3488 0z m110.0288 130.816a72.8064 72.8064 0 1 0 145.5616 0 72.8064 72.8064 0 0 0-145.5616 0z m43.7248 169.472a78.3872 78.3872 0 1 0 156.8256 0 78.3872 78.3872 0 0 0-156.8256 0z"
            fill="#cccccc" p-id="1307"></path>
        </svg>
        <svg v-else t="1656476104531" class="icon" viewBox="0 0 1024 1024" version="1.1"
          xmlns="http://www.w3.org/2000/svg" p-id="2198" width="32" height="18">
          <path d="M760.768 576L384 931.84 469.76 1024 1024 515.392 474.048 0 384 98.88 753.6 448H0v128h760.768z"
            fill="#cccccc" p-id="2199"></path>
        </svg>
      </div>
    </div>
  </div>
</template>

