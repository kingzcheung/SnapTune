<script setup>
import { onMounted, ref, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import {
  CheckCircleIcon,
  XCircleIcon,
  ExclamationCircleIcon,
} from "@heroicons/vue/solid";

const files = ref([]);
const formats = ref([
  "PNG",
  "JPEG",
  "GIF",
  "BMP",
  "ICO",
  "TIFF",
  "WebP",
  "AVIF",
  "PNM",
  "DDS",
  "TGA",
  "OpenEXR",
  "Farbfeld",
]);
const currentFormat = ref("PNG");

const errors = ref({
  msg: "",
  error: false,
});

const get_file_name = (path) => {
  var pos1 = path.lastIndexOf("/");
  var pos2 = path.lastIndexOf("\\");
  var pos = Math.max(pos1, pos2);
  if (pos < 0) return path;
  else return path.substring(pos + 1);
};

// 转换的加载动画状态
const convertLoading = ref(false);

watch(currentFormat, (newValue, oldValue) => {
  if (newValue) {
    errors.value.error = false;
  }
});

watch(files, (newValue, oldValue) => {
  if (newValue.length > 0) {
    errors.value.error = false;
  }
});

const selectFormat = (format) => {
  currentFormat.value = format;
};

const onClick = () => {
  if (files.value.length == 0) {
    errors.value = {
      error: true,
      msg: "请选择转换文件",
    };
    return;
  }

  if (!currentFormat.value) {
    errors.value = {
      error: true,
      msg: "请选择转换格式",
    };
    return;
  }

  if (convertLoading.value) {
    return;
  }

  convertLoading.value = true;

  files.value.forEach((file, index) => {
    invoke("image2x_command", {
      x: currentFormat.value,
      index: index,
      source: file.file,
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

const ext = (file_path) => {
  const h = file_path.substr(file_path.lastIndexOf(".") + 1);
  return h.toUpperCase();
};

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
  <div class="convert">
    <h1 class="text-2xl mb-5">图片转换</h1>
    <div class="overflow-y-auto -m-3">
      <div
        class="flex item-center p-3 align-middle"
        :class="{ 'bg-base-100': i % 2 == 0 }"
        v-for="(file, i) of files"
        :key="file.file"
      >
        <div class="truncate w-80 flex-1">{{ file.filename }}</div>
        <div class="px-2">{{ file.type }}</div>
        <div class="px-4 self-center">
          <check-circle-icon
            v-if="file.status == 1"
            class="h-5 w-5 text-green-500"
          ></check-circle-icon>
          <x-circle-icon
            v-if="file.status == -1"
            class="h-5 w-5 text-red-500"
          ></x-circle-icon>
          <exclamation-circle-icon
            v-if="file.status == 0"
            class="h-5 w-5 text-gray-500"
          ></exclamation-circle-icon>
        </div>
        <div class="ml-2">
          <button class="btn btn-xs btn-outline" @click="deleteItem(i)">
            删除
          </button>
        </div>
      </div>
    </div>
    <div
      class="
        absolute
        z-50
        left-0
        right-0
        bottom-0
        bg-gray-800
        w-full
        flex
        items-stretch
      "
    >
      <div class="flex-1 flex items-center">
        <!-- <button class="btn btn-wide m-2">添加更多文件</button> -->
        <div v-if="!errors.error" class="pl-4 flex items-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            class="stroke-info flex-shrink-0 w-6 h-6"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            ></path>
          </svg>
          <span class="text-gray-300 text-sm pl-1"
            >提示: 可以直接把文件拖拽进来</span
          >
        </div>
        <div v-if="errors.error" class="pl-4 flex items-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-current flex-shrink-0 h-6 w-6"
            fill="#ff0000"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
            />
          </svg>
          <span class="text-gray-300 pl-1">错误: {{ errors.msg }}</span>
        </div>
      </div>
      <div class="flex items-center dropdown dropdown-left dropdown-end">
        <label tabindex="0" class="btn m-1">{{
          currentFormat ? `格式: ${currentFormat}` : "选择格式"
        }}</label>
        <div
          tabindex="0"
          class="
            dropdown-content
            card card-compact
            w-96
            p-2
            shadow
            bg-black
            text-primary-content
          "
        >
          <div class="card-body">
            <div class="grid grid-cols-3 gap-4">
              <button
                @click="selectFormat(format)"
                v-for="format in formats"
                :key="format"
                class="btn text-md btn-sm"
                :class="{ 'btn-active btn-accent': format === currentFormat }"
              >
                {{ format }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <div
        class="flex items-center px-1 bg-primary ml-2 cursor-pointer"
        @click="onClick"
      >
        <span class="text-primary-content font-bold px-8">转换</span>
        <svg
          v-if="convertLoading"
          t="1656508933337"
          class="icon animate-spin"
          viewBox="0 0 1024 1024"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          p-id="1306"
          width="24"
          height="24"
        >
          <path
            d="M834.7648 736.3584a5.632 5.632 0 1 0 11.264 0 5.632 5.632 0 0 0-11.264 0z m-124.16 128.1024a11.1616 11.1616 0 1 0 22.3744 0 11.1616 11.1616 0 0 0-22.3744 0z m-167.3216 65.8944a16.7936 16.7936 0 1 0 33.6384 0 16.7936 16.7936 0 0 0-33.6384 0zM363.1616 921.6a22.3744 22.3744 0 1 0 44.7488 0 22.3744 22.3744 0 0 0-44.7488 0z m-159.744-82.0224a28.0064 28.0064 0 1 0 55.9616 0 28.0064 28.0064 0 0 0-56.0128 0zM92.672 700.16a33.6384 33.6384 0 1 0 67.2256 0 33.6384 33.6384 0 0 0-67.2256 0zM51.2 528.9984a39.168 39.168 0 1 0 78.336 0 39.168 39.168 0 0 0-78.336 0z m34.1504-170.0864a44.8 44.8 0 1 0 89.6 0 44.8 44.8 0 0 0-89.6 0zM187.904 221.7984a50.432 50.432 0 1 0 100.864 0 50.432 50.432 0 0 0-100.864 0zM338.432 143.36a55.9616 55.9616 0 1 0 111.9232 0 55.9616 55.9616 0 0 0-111.9744 0z m169.0112-4.9152a61.5936 61.5936 0 1 0 123.2384 0 61.5936 61.5936 0 0 0-123.2384 0z m154.7776 69.632a67.1744 67.1744 0 1 0 134.3488 0 67.1744 67.1744 0 0 0-134.3488 0z m110.0288 130.816a72.8064 72.8064 0 1 0 145.5616 0 72.8064 72.8064 0 0 0-145.5616 0z m43.7248 169.472a78.3872 78.3872 0 1 0 156.8256 0 78.3872 78.3872 0 0 0-156.8256 0z"
            fill="#cccccc"
            p-id="1307"
          ></path>
        </svg>
        <svg
          v-else
          t="1656476104531"
          class="icon"
          viewBox="0 0 1024 1024"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          p-id="2198"
          width="32"
          height="18"
        >
          <path
            d="M760.768 576L384 931.84 469.76 1024 1024 515.392 474.048 0 384 98.88 753.6 448H0v128h760.768z"
            fill="#cccccc"
            p-id="2199"
          ></path>
        </svg>
      </div>
    </div>
  </div>
</template>

