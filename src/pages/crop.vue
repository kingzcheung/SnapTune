<script setup lang="ts">
import {Tabs, TabsContent, TabsList, TabsTrigger} from '@/components/ui/tabs'
import {onMounted, ref} from "vue";
import '@/lib/proxyData.ts'
import {ImageUp} from "lucide-vue-next";
import {open} from "@tauri-apps/plugin-dialog";
import {Label} from '@/components/ui/label'
import {readFile} from '@tauri-apps/plugin-fs';
import '@leafer-in/editor'
import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@/components/ui/number-field'
import {App, Rect} from 'leafer-ui'


interface ImageFile {
  raw_path: string;
  savePath: string;
  file_name: string;
  show_url: string;
  raw_width: number;
  raw_height:number;
}

const file = ref<ImageFile | null>(null);
const rect = new Rect({
  height: 400,
  width: 400,
  x: 0,
  y: 0,
  locked: true,
})

async function selectFileHandle() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "Images",
        extensions: ["png", "jpg", "jpeg", "webp"],
      },
    ],
  });

  if (selected != null) {
    let imageData = await readFile(selected)
    // 获取图片的高和宽

    const image_url = URL.createObjectURL(new Blob([imageData]));
    const img = new Image();
    img.src = image_url
    img.onload = () => {
      file.value = {
        raw_path: selected,
        savePath: "",
        file_name: selected.split('/').pop() || "",
        show_url: image_url,
        raw_height: img.height,
        raw_width: img.width,
      }
    }



    const proxyRect = rect.proxyData
    if (proxyRect) {
      proxyRect.fill = {
        type: 'image',
        url: image_url,
      }
    }
  }
}


onMounted(() => {
  const app = new App({
    view: 'leafer',
    editor: {
      rotateable: false
    }, // 会自动创建 editor实例、tree层、sky层
    width: 400, // 不能设置为 0， 否则会变成自动布局
    height: 400,
    wheel: {
      disabled: true,
    },
  })

  app.tree.add(rect)
  const r1 = Rect.one({
    editable: true,
    fill: '#FFE04B44',
    dragBounds: 'parent',
    cornerRadius: [0, 0, 0, 0],
  }, 300, 100)
  app.tree.add(r1)

  r1.on("scale",()=>{
    console.log(r1.scale)
  })

})


</script>

<template>
  <main class="container h-full flex flex-col justify-start items-center">
    <div class="flex items-center flex-row justify-between mb-4 px-6 w-full">
      <h1 class="text-4xl text-left font-bold flex-1 w-full">Crop and Resize</h1>
    </div>
    <div class="bg-white py-4 h-full w-full px-4 rounded-2xl flex flex-col">
      <Tabs default-value="crop" class="w-[400px] mb-4">
        <TabsList>
          <TabsTrigger value="crop">
            Crop
          </TabsTrigger>
          <TabsTrigger value="resize">
            Resize
          </TabsTrigger>
        </TabsList>
        <TabsContent value="crop">
          <div class="flex items-center gap-4">
            <NumberField :min="20" :default-value="100" class="flex items-center">
              <Label>Width</Label>
              <NumberFieldContent>
                <NumberFieldDecrement/>
                <NumberFieldInput/>
                <NumberFieldIncrement/>
              </NumberFieldContent>
            </NumberField>
            <NumberField :min="20" :default-value="100" class="flex items-center">
              <Label>Height</Label>
              <NumberFieldContent>
                <NumberFieldDecrement/>
                <NumberFieldInput/>
                <NumberFieldIncrement/>
              </NumberFieldContent>
            </NumberField>
          </div>
        </TabsContent>
        <TabsContent value="resize">
          here is resize
        </TabsContent>
      </Tabs>
      <div
          v-show="!file?.show_url"
          class="flex flex-col gap-2 w-full items-center justify-center py-12 flex-1  rounded-2xl border-dashed border-2 cursor-pointer"
          @click="selectFileHandle">
        <div>
          <ImageUp class="w-12 h-12 text-zinc-600"/>
        </div>
        <div class="text-xl text-zinc-700">Click, or Drag and drop image here.</div>
        <div class="text-base text-gray-400 mb-8">Supports the following formats: PNG, JPEG, WEBP, GIF.</div>
      </div>
      <div class="flex items-center justify-center bg-center  relative w-[400px] h-[400px]" v-show="file?.show_url">
        <div id="leafer" class="w-full h-full"></div>
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>