<script setup lang="ts">
import {onMounted, onUnmounted, ref, watchEffect} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button";
import {CircleCheckBig, Loader} from 'lucide-vue-next';
import {getCurrentWebview} from "@tauri-apps/api/webview";

import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {open} from '@tauri-apps/plugin-dialog';
import {readFile} from '@tauri-apps/plugin-fs';
import {convertBytes} from "./lib/utils.ts";
import {UnlistenFn} from "@tauri-apps/api/event";

// 压缩文件对象
interface ImageFile {
  raw_path: string;
  savePath: string;
  file_name: string;
  size: number;
  compressedSize: number;
}

interface CompressedFile {
  saved_path: string;
  size: number;
}


const files = ref<ImageFile[]>([]);

async function selectFileHandle() {
  files.value = []
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "Images",
        extensions: ["png", "jpg", "jpeg"],
      },
    ],
  });
  if (Array.isArray(selected)) {
    // user selected multiple files
    for (let file of selected) {
      const fileData = await readFile(file);
      files.value.push({
        file_name: file.split('/').pop() || "",
        raw_path: file,
        size: fileData.length,
        compressedSize: 0,
        savePath: "",
      })
    }

    console.log(selected)
  } else if (selected === null) {
    // user
  }
}

watchEffect(async () => {
  console.log("加班了")
  for (let file of files.value) {
    let compress_file: CompressedFile = await invoke('compress_image', {filePath: file.raw_path})
    file.compressedSize = compress_file.size;
    file.savePath = compress_file.saved_path;
  }
})

async function openHandle(p: String) {
  if (p === "") {
    console.log("not path")
    return
  }
  await invoke('open_folder', {path: p})
}

let unlisten: UnlistenFn | null = null;
onMounted(async () => {
  unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'drop') {
      console.log('User dropped', event.payload.paths);
      for (let file of event.payload.paths) {
        readFile(file).then((fileData) => {
          files.value.push({
            file_name: file.split('/').pop() || "",
            raw_path: file,
            size: fileData.length,
            compressedSize: 0,
            savePath: "",
          })
        });
      }
    }
  });
})

onUnmounted(()=>{
  if (unlisten !== null) unlisten()
})

</script>

<template>
  <main class="container">
    <h1 class="text-2xl text-center mb-4">Image to Tiny</h1>
    <div class="flex flex-col gap-2 items-center justify-center py-12 rounded-3xl border-dashed border-2"
         v-if="files.length === 0">
      <div class="text-xl">Drag and drop image files</div>
      <div class="text-base text-gray-400 mb-8">Supports the following formats:png, jpg, jpeg</div>
      <Button variant="outline" class="w-24" @click="selectFileHandle">Select files</Button>
    </div>
    <div v-else>
      <div class="flex flex-wrap items-center justify-between gap-4 mt-4">
        <h1 class="flex-1">All files</h1>
        <Button variant="outline" class="w-24" @click="selectFileHandle">Select files</Button>
      </div>
      <Table>
        <TableCaption>
          The output image is in the output directory at the original image location.
        </TableCaption>
        <TableHeader>
          <TableRow>
            <TableHead class="w-[200px]">
              File
            </TableHead>
            <TableHead>Before Size</TableHead>
            <TableHead>After Size</TableHead>
            <TableHead>Status</TableHead>
            <TableHead class="text-right">
              Saved up
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="(file,i) in files" :key="i">
            <TableCell class="font-medium">
              <a class="underline" href="javascript:;" @click="openHandle(file.savePath)">{{ file.file_name }}</a>
            </TableCell>
            <TableCell>
              <span>{{ convertBytes(file.size) }} </span>
            </TableCell>
            <TableCell>
              <span class="text-green-600" v-if="file.compressedSize > 0">{{ convertBytes(file.compressedSize) }}</span>
            </TableCell>
            <TableCell class="text-right">
              <CircleCheckBig class="text-green-400 w-4 h-4" v-if="file.compressedSize > 0"/>
              <Loader class="animate-spin w-4 h-4 " v-else/>
            </TableCell>
            <TableCell class="text-right">
              {{ file.compressedSize ? `${((file.size - file.compressedSize) / file.size * 100).toFixed(2)}%` : "0%" }}
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </main>
</template>
