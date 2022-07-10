<script setup>
import { ref, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { get_file_name, ext } from "../utils";
import Model from '../components/Model.vue';

const files = ref([]);
const visibleModel = ref(false);

onMounted(() => {
  listen("tauri://file-drop", (event) => {
    files.value = event.payload.map((v) => {
      let filename = get_file_name(v);
      return {
        old_name: filename,
        new_file: filename,
      };
    });

    // 添加弹窗
    visibleModel.value = true;
  });
});
</script>
<template>
  <div class="rename overflow-hidden overflow-x-hidden">
    <div class="overflow-y-auto overflow-x-hidden overscroll-none">
      <table
        class="table table-compact w-full overflow-x-hidden"
        v-if="files.length > 0"
      >
        <thead>
          <tr>
            <th></th>
            <th>原文件名</th>
            <th class="text-right">新文件名</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(file, i) of files" :key="file.old_name">
            <th>{{ i }}</th>
            <th>{{ file.old_name }}</th>
            <th class="text-right">{{ file.new_file }}</th>
          </tr>
        </tbody>
      </table>
      <model v-model="visibleModel">
      <template v-slot:header>
        选项
      </template>
        <div>
            <input type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" />

        </div>
      </model>
    </div>
  </div>
</template>