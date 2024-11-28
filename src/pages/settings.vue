<script setup lang="ts">
import {Input} from '@/components/ui/input'
import {Button} from "@/components/ui/button";
import {Separator} from '@/components/ui/separator'
import {Slider} from '@/components/ui/slider'
import {Store} from '@tauri-apps/plugin-store';

import {ref, onMounted} from "vue";

interface SettingsInterface {
  quality: number[];
  imageCompressSavePath: string;
  imageConvertSavePath: string;
}

const form = ref<SettingsInterface>({
  quality: [80],
  imageCompressSavePath: "",
  imageConvertSavePath: "",
})

let store: Store;

onMounted(async () => {
  store = await Store.load('settings.json')
  const store_form = await store.get<SettingsInterface>('settings');
  if (store_form) {
    form.value = store_form
  }
})


async function handleVolumeChange(val: number[]) {
  console.log("set quality")
  await store.set('quality', {value: val[0]})
  await store.save()
}

</script>

<template>
  <main class="container h-full flex flex-col justify-start items-center">
    <div class="flex items-center flex-row justify-between mb-4 px-6 w-full">
      <h1 class="text-4xl text-left font-bold flex-1 w-full">Settings</h1>
    </div>
    <div class="bg-white py-4 h-full w-full px-4 rounded-2xl">
      <div class="my-4">
        <h2 class="text-zinc-700 text-xl mb-4 capitalize">image compression quality</h2>
        <div class="flex w-full flex-col  items-center gap-1.5">
          <Slider
              v-model="form.quality"
              :max="100"
              :min="20"
              :step="1"
              @valueCommit="handleVolumeChange"
              class="w-full"
          />
          <div class="flex text-left w-full items-center gap-2 justify-between text-zinc-500 text-base">
            <span class="text-xs">Note: Too low quality will lead to image distortion.</span>
            <span>{{ form.quality[0] }}%</span>
          </div>
        </div>
      </div>
      <div class="my-4">
        <h2 class="text-zinc-700 text-xl mb-4">Image compression output directory</h2>
        <div class="text-zinc-400 text-xs mb-2">
          <p>If it is empty, it will be output to the output folder in the directory where the selected picture is located by default.</p>
          <p>For example, for the image compression function, when the image path you select is `/Users/xxx/Downloads/xxx.jpg`, after compression, it is output by default to `/Users/xxx/Downloads/output/xxx.jpg`.</p>
        </div>
        <div class="flex w-full max-w-sm items-center gap-1.5">
          <Input id="imageCompressSavePath" v-model="form.imageCompressSavePath" type="text"/>
          <Button type="submit">
            Select Folder
          </Button>
        </div>
      </div>

      <Separator/>
      <div class="my-4">
        <h2 class="text-zinc-700 text-xl mb-4">Image conversion output directory</h2>
        <div class="text-zinc-400 text-xs mb-2">
          <p>If it is empty, it will be output to the output folder in the directory where the selected picture is located by default.</p>
          <p>For example, if the picture path you select is `/Users/xxx/Downloads/xxx.jpg`, after conversion to PNG, it is output by default to `/Users/xxx/Downloads/output/xxx.png`.</p>
        </div>
        <div class="flex w-full max-w-sm items-center gap-1.5">
          <Input id="imageConvertSavePath" type="text"/>
          <Button type="submit">
            Select Folder
          </Button>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>