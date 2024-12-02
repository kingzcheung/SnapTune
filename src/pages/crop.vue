<script setup lang="ts">
import {Tabs, TabsContent, TabsList, TabsTrigger} from '@/components/ui/tabs'
import {onMounted, ref} from "vue";
import '@/lib/proxyData.ts'
import {ImageUp,Link2} from "lucide-vue-next";
import {open} from "@tauri-apps/plugin-dialog";
import {readFile} from '@tauri-apps/plugin-fs';
import '@leafer-in/editor'
import {Button} from '@/components/ui/button'
import {App, IUI,  LeaferEvent, PropertyEvent, Rect} from 'leafer-ui'
import { save } from '@tauri-apps/plugin-dialog';
import {invoke} from "@tauri-apps/api/core";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';

interface ImageFile {
  raw_path: string;
  savePath: string;
  file_name: string;
  show_url: string;
  raw_width: number;
  raw_height: number;
  crop_width: number;
  crop_height: number;
  scale: number;
  crop_x:number;
  crop_y:number;
}

interface ResizeImage {
  width: number;
  height: number;
  view_width: number;
  view_height: number;
}
// 是否关联
const isAssociate = ref(false)
const permissionGranted = ref(false)
const file = ref<ImageFile | null>(null);
const resize = ref<ResizeImage>({
  width:0,
  height:0,
  view_width:0,
  view_height:400,
})
const rect = new Rect({
  height: 400,
  width: 400,
  x: 0,
  y: 0,
  locked: true,
})
const activeTab = ref('crop')
let app: App;
let r1: IUI;

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
    const proxyRect = rect.proxyData
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
        scale: 400 / img.height,
        crop_width: 192,
        crop_height: 192,
        crop_x: 0,
        crop_y: 0,
      }

      resize.value.width = img.width
      resize.value.height = img.height
      resize.value.view_width = img.width * 400 / img.height

      if (proxyRect) {
        proxyRect.width = img.width * 400 / img.height
      }
      if (app.proxyData) {
        app.proxyData.width = img.width * 400 / img.height
      }
      if (r1.proxyData) {
        if (r1.proxyData.widthRange) {
          r1.proxyData.widthRange.max = img.width * 400 / img.height
        }
        if (r1.x && proxyRect) {
          r1.proxyData.x =(proxyRect.width || 0) / 2 - 96
        }
        if (r1.y && proxyRect) {
          r1.proxyData.y =(proxyRect.height || 0) / 2 - 96
        }
        if (r1.width) {
          r1.proxyData.width = 192 * file.value?.scale||1
        }
        if (r1.height) {
          r1.proxyData.height = 192 * file.value?.scale||1
        }
      }
      console.log(app)
    }


    if (proxyRect) {
      proxyRect.fill = {
        type: 'image',
        url: image_url,
      }
    }
  }
}


onMounted(async () => {
  let isp = await isPermissionGranted();
  if (!isp) {
    const permission = await requestPermission();
    permissionGranted.value = permission === 'granted';

  }

  app = new App({
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
  r1 = Rect.one({
    editable: true,
    fill: 'rgba(138,250,171,0.47)',
    dragBounds: 'parent',
    width:192,
    height:192,
    cornerRadius: [0, 0, 0, 0],
    widthRange: {
      min: 100,
      max: 400
    },
    heightRange: {
      min: 100,
      max: 400
    }
  }, 300, 100)

  app.tree.add(r1)


  app.on(LeaferEvent.READY, () => {
    console.log("READY")
    r1.on(PropertyEvent.CHANGE, function (e: PropertyEvent) {
      // console.log('leafer', e.target, e.attrName, e.newValue, e.oldValue)
      if (e.attrName ==='width') {
        if (file.value) {
          file.value.crop_width = Math.floor((e.newValue as number) / file.value.scale)
        }
      }
      if (e.attrName==='height') {
        if (file.value) {
          file.value.crop_height = Math.floor((e.newValue as number) / file.value.scale)
        }
      }
      if (e.attrName==='x') {
        if (file.value) {
          file.value.crop_x = Math.floor((e.newValue as number) / file.value.scale)
        }
      }
      if (e.attrName==='y') {
        if (file.value) {
          file.value.crop_y = Math.floor((e.newValue as number) / file.value.scale)
        }
      }
    })
  })
})

async function cropHandle(){

  const path = await save({
    filters: [
      {
        name: 'crop',
        extensions: ['png', 'jpeg'],
      },
    ],
  });
  if (path) {
    if (r1.x && r1.y && file.value) {
      await invoke("crop_image",{
        imagePath: file.value?.raw_path,
        savePath: path,
        cropWidth: file.value?.crop_width,
        cropHeight: file.value?.crop_height,
        x: file.value.crop_x,
        y: file.value.crop_y,
      })
    }

  }

}
function changeWHandle(){
  resize.value.view_width = Math.floor(resize.value.width * 400 / resize.value.height)
  if (isAssociate) {
    resize.value.height = Math.floor(resize.value.width * resize.value.view_height / resize.value.height)
    resize.value.view_height = Math.floor(resize.value.height * resize.value.view_width / resize.value.width)

  }
}
function changeHHandle(){
  resize.value.view_height = Math.floor(resize.value.height * 400 / resize.value.width)
  if (isAssociate) {
    resize.value.width = Math.floor(resize.value.height * resize.value.view_width / resize.value.width)
    resize.value.view_width = Math.floor(resize.value.width * resize.value.view_height / resize.value.height)
  }
}
async function exportHandle(){
  const path = await save({
    filters: [
      {
        name: 'crop',
        extensions: ['png', 'jpeg'],
      },
    ],
  });
  if (path) {
    await invoke("resize_image",{
      imagePath: file.value?.raw_path,
      savePath: path,
      width: resize.value.width,
      height: resize.value.height,
    })
    if (permissionGranted.value) {
      sendNotification({ title: 'Resize Result', body: 'Success!' });
    }
  }
}

</script>

<template>
  <main class="container h-full flex flex-col justify-start items-center">
    <div class="flex items-center flex-row justify-between mb-4 px-6 w-full">
      <h1 class="text-4xl text-left font-bold flex-1 w-full">Crop and Resize</h1>
    </div>
    <div class="bg-white py-4 h-full w-full px-4 rounded-2xl flex flex-col">
      <div class="flex items-center justify-between">
        <Tabs default-value="crop" v-model="activeTab" class="mb-4 flex items-center w-full gap-4 flex-1">
          <TabsList>
            <TabsTrigger value="crop">
              Crop
            </TabsTrigger>
            <TabsTrigger value="resize">
              Resize
            </TabsTrigger>
          </TabsList>
          <TabsContent value="crop" class="w-full mt-0">
            <div class="flex items-center w-full gap-4" v-if="file">
              <div class="flex items-center  flex-1" >
                <span>Crop size:</span>
                <span>{{file?.crop_width}} X {{file?.crop_height}}</span>
              </div>
              <Button class="rounded-full" @click="cropHandle">Crop</Button>
            </div>
          </TabsContent>
          <TabsContent value="resize"  class="w-full mt-0">
            <div class="flex items-center gap-6">
              <div class="relative w-24">
                <label for="w" class="absolute inset-y-0 px-1 text-xs align-middle leading-loose	 bg-zinc-200 rounded-sm inline-block left-0 uppercase">w</label>
                <input id="w" @change="changeWHandle" v-model="resize.width" class="py-0.5 pl-6 w-full outline outline-offset-2 outline-2 rounded-md outline-zinc-300 focus:outline-zinc-500 " type="number">
              </div>

              <div class="relative w-24">
                <label for="h" class="absolute inset-y-0 px-1 text-xs align-middle leading-loose	 bg-zinc-200 rounded-sm inline-block left-0 uppercase">h</label>
                <input id="h" @change="changeHHandle" v-model="resize.height" class="py-0.5 pl-6 w-full outline outline-offset-2 outline-2 rounded-md outline-zinc-300 focus:outline-zinc-500 " type="number">
              </div>
              <button type="button" @click="isAssociate = !isAssociate" class="bg-zinc-200 p-2 rounded-md" :class="{'bg-zinc-700 text-white':isAssociate}">
                <Link2 class="w-5 h-5" />
              </button>
              <span class="flex-1"></span>
              <Button class="" :disabled="!file?.show_url" @click="exportHandle">Export</Button>
            </div>
          </TabsContent>
        </Tabs>

      </div>
      <div
          v-show="!file?.show_url"
          class="flex flex-col  gap-2 w-full items-center justify-center py-12 flex-1  rounded-2xl border-dashed border-2 cursor-pointer"
          @click="selectFileHandle">
        <div>
          <ImageUp class="w-12 h-12 text-zinc-600"/>
        </div>
        <div class="text-xl text-zinc-700">Click, or Drag and drop image here.</div>
        <div class="text-base text-gray-400 mb-8">Supports the following formats: PNG, JPEG, WEBP, GIF.</div>
      </div>
      <div class="flex items-center justify-center bg-center overflow-hidden relative " v-show="file?.show_url && activeTab=='crop'">
        <div id="leafer" class=" h-[400px]"></div>
      </div>
      <div class="flex items-center justify-center bg-center overflow-hidden relative "  v-show="file?.show_url && activeTab=='resize'">
        <div class="h-[400px] w-full bg-no-repeat bg-center" :style="{
          backgroundImage:`url(${file?.show_url})`,
          backgroundSize:resize.view_width+'px '+resize.view_height+'px'
        }">
<!--          <img :src="file?.show_url" alt="" class="object-cover" :style="{-->
<!--            width:resize.view_width+'px',-->
<!--            height:resize.view_height+'px'-->
<!--          }" />-->
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>

</style>