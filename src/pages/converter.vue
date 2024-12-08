<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref,watchEffect} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button";
import {CircleX, ImageUp, Send, LoaderCircle} from 'lucide-vue-next';
import {getCurrentWebview} from "@tauri-apps/api/webview";
import {open} from '@tauri-apps/plugin-dialog';
import {UnlistenFn} from "@tauri-apps/api/event";
import { toast } from 'vue-sonner'
import { Toaster } from '@/components/ui/sonner'
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'

// 压缩文件对象
interface ImageFile {
  raw_path: string;
  savePath: string;
  file_name: string;
  to_format: string;
  status: string;
}

interface ConverterResponse {
  status: string;
  save_path: string;
}

const all_to_format = ref("")


const files = ref<ImageFile[]>([]);

async function selectFileHandle() {
  files.value = []
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "Images",
        extensions: ["png", "jpg", "jpeg", "webp"],
      },
    ],
  });
  if (Array.isArray(selected)) {
    // user selected multiple files
    for (let file of selected) {
      files.value.push({
        file_name: file.split('/').pop() || "",
        raw_path: file,
        to_format: "",
        savePath: "",
        status: "",
      })
    }

    console.log(selected)
  } else if (selected === null) {
    // user
  }
}

watchEffect(() => {
  console.log("console")
  for (let file of files.value) {
    file.to_format = all_to_format.value
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
        files.value.push({
          file_name: file.split('/').pop() || "",
          raw_path: file,
          savePath: "",
          to_format: "",
          status: "",
        })
      }
    }
  });
})

onUnmounted(() => {
  if (unlisten !== null) unlisten()
})

// 查看是否所有的文件都设置了转换格式
const allFilesSet = computed(() => {
  return files.value.every(value => value.to_format.length > 0)
});

async function handleRemoveItem(i: number) {
  files.value.splice(i, 1)
}

const loading = ref(false)

async function handleConvert() {
  loading.value = true
  for (let file of files.value) {
    try {
      if (!file.to_format) {
        throw new Error("Please select a format")
      }
      file.status = "In Progress"
      let resp: ConverterResponse = await invoke('convert', {filePath: file.raw_path, toFormat: file.to_format})
      file.status = resp.status;
      file.savePath = resp.save_path;
    } catch (e) {
      file.status = "Error"
    } finally {
      loading.value = false
    }
  }
  toast.info('Finish!')
}

</script>

<template>
  <main class="container h-full flex flex-col justify-start items-center">
    <div class="flex items-center flex-row justify-between mb-4 px-6 w-full">
      <h1 class="text-4xl text-left font-bold flex-1 w-full">Image Converter</h1>
      <div class="flex items-center justify-center mr-4" v-if="files.length>0">
        <span class="capitalize mr-2 text-sm">All Convert to</span>
        <Select v-model="all_to_format">
          <SelectTrigger class="w-[80px] py-0">
            <SelectValue placeholder="..."/>
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem value="jpeg">
                JPEG
              </SelectItem>
              <SelectItem value="png">
                PNG
              </SelectItem>
              <SelectItem value="webp">
                WEBP
              </SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
      <Button v-if="files.length>0" variant="outline" class="w-24 rounded-full" @click="selectFileHandle">Select files</Button>
    </div>
    <Toaster position="bottom-center" />
    <div class="bg-white rounded-2xl p-6 w-full flex-1">
      <div
          class="flex flex-col gap-2  items-center justify-center py-12 h-full rounded-2xl border-dashed border-2 cursor-pointer"
          v-if="files.length === 0" @click="selectFileHandle">
        <div>
          <ImageUp class="w-12 h-12 text-zinc-600"/>
        </div>
        <div class="text-xl text-zinc-700">Click, or Drag and drop image here.</div>
        <div class="text-base text-gray-400 mb-8">Supports the following formats: PNG, JPEG, WEBP, GIF.</div>
      </div>
      <div v-else>
        <div class="flex items-center gap-4 mb-1 bg-zinc-50 py-2 px-2 rounded-md" v-for="(file,i) in files">
          <a class="flex-1 " href="javascript:" @click="openHandle(file.file_name)">{{ file.file_name }}</a>
          <div class="flex items-center justify-center gap-2 shrink-0 w-[200px]" v-if="file.status.length > 0">
            <span class="capitalize" :class="{
              'text-green-500':file.status=='done',
              'text-red-500': file.status=='error',
              }">{{ file.status }}</span>
          </div>
          <div class="flex items-center justify-center gap-2 shrink-0 w-[200px]"
               v-if="file.status.length===0 && loading === false">
            <span class="">Convert to</span>
            <Select v-model="file.to_format">
              <SelectTrigger class="w-[80px]">
                <SelectValue placeholder="..."/>
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectItem value="jpeg">
                    JPEG
                  </SelectItem>
                  <SelectItem value="png">
                    PNG
                  </SelectItem>
                  <SelectItem value="webp">
                    WEBP
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
          <div class="w-[64px]">
            <Button type="button" variant="link" @click="handleRemoveItem(i)">
              <CircleX/>
            </Button>
          </div>
        </div>
        <div class="fixed bottom-12 right-14 flex items-center shadow-2xl rounded-full ">
          <button type="button" variant="link" :disabled="loading || !allFilesSet" @click="handleConvert"
                  :class="{
                       'bg-zinc-300 hover:bg-zinc-300':!allFilesSet
                  }"
                  class="flex rounded-full text-white transition-all ease-in-out bg-zinc-700 hover:bg-zinc-900 p-4">
            <LoaderCircle class="w-8 h-8 animate-spin" v-if="loading"/>
            <Send v-else class="w-8 h-8"/>
          </button>
        </div>
      </div>
    </div>
  </main>
</template>
