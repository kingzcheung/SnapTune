<script setup>
import { onMounted, ref, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

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

onMounted(() => {

  listen("tauri://file-drop", (event) => {
    console.log(event.payload)
    files.value = event.payload.map((v) => {
      return {
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
    <div class="overflow-y-auto">
      <table class="table table-compact w-full mb-20">
        <!-- head -->
        <thead>
          <tr>
            <th></th>
            <th>名称</th>
            <th>类型</th>
            <th>状态</th>
          </tr>
        </thead>
        <tbody>
          <!-- row 1 -->
          <tr
            :class="{ active: i % 2 == 0 }"
            v-for="(file, i) of files"
            :key="file.file"
          >
            <th>{{ i + 1 }}</th>
            <td class="truncate">{{ file.file }}</td>
            <td>{{ file.type }}</td>
            <td>
              <svg
                v-if="file.status === 1"
                t="1656489464034"
                class="icon"
                viewBox="0 0 1024 1024"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                p-id="1248"
                width="16"
                height="16"
              >
                <path
                  d="M512 512m-448 0a448 448 0 1 0 896 0 448 448 0 1 0-896 0Z"
                  fill="#07C160"
                  p-id="1249"
                ></path>
                <path
                  d="M466.7 679.8c-8.5 0-16.6-3.4-22.6-9.4l-181-181.1c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l158.4 158.5 249-249c12.5-12.5 32.8-12.5 45.3 0s12.5 32.8 0 45.3L489.3 670.4c-6 6-14.1 9.4-22.6 9.4z"
                  fill="#FFFFFF"
                  p-id="1250"
                ></path>
              </svg>
              <svg
                v-if="file.status === -1"
                t="1656489517929"
                class="icon"
                viewBox="0 0 1024 1024"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                p-id="1523"
                width="16"
                height="16"
              >
                <path
                  d="M512 512m-448 0a448 448 0 1 0 896 0 448 448 0 1 0-896 0Z"
                  fill="#FA5151"
                  p-id="1524"
                ></path>
                <path
                  d="M557.3 512l113.1-113.1c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L512 466.7 398.9 353.6c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L466.7 512 353.6 625.1c-12.5 12.5-12.5 32.8 0 45.3 6.2 6.2 14.4 9.4 22.6 9.4s16.4-3.1 22.6-9.4L512 557.3l113.1 113.1c6.2 6.2 14.4 9.4 22.6 9.4s16.4-3.1 22.6-9.4c12.5-12.5 12.5-32.8 0-45.3L557.3 512z"
                  fill="#FFFFFF"
                  p-id="1525"
                ></path>
              </svg>
              <svg
                v-if="file.status === 0"
                t="1656489552631"
                class="icon"
                viewBox="0 0 1024 1024"
                version="1.1"
                xmlns="http://www.w3.org/2000/svg"
                p-id="1798"
                width="16"
                height="16"
              >
                <path
                  d="M512 512m-448 0a448 448 0 1 0 896 0 448 448 0 1 0-896 0Z"
                  fill="#CCCCCC"
                  p-id="1799"
                ></path>
                <path
                  d="M676.6 347.4H347.4c-15.1 0-27.4 12.3-27.4 27.4v274.3c0 15.1 12.3 27.4 27.4 27.4h329.1c15.1 0 27.4-12.3 27.4-27.4V374.9c0.1-15.2-12.2-27.5-27.3-27.5zM361 374.9h302c7.5 0 13.6 6.1 13.6 13.6v167.3l-90.4-94.7c-2.7-2.7-6.2-4-9.7-4-3.5 0-7 1.3-9.7 4l-98.7 102.8-44.5-22.1c-2.3-1.6-5-2.3-7.6-2.3-3.5 0-7 1.3-9.6 4l-58.9 58.9V388.5c-0.1-7.6 6-13.6 13.5-13.6z m315.6 260.6c0 7.5-6.1 13.6-13.6 13.6H361c-5.9 0-10.8-3.7-12.8-8.9l70.5-70.5 44 21.9c2.4 1.8 5.3 2.7 8.2 2.7 4.2 0 8.4-1.9 11-5.5l95.7-99.7 98.8 101.3v45.1z"
                  fill="#FFFFFF"
                  p-id="1800"
                ></path>
                <path
                  d="M416 484.6c22.7 0 41.1-18.4 41.1-41.1 0-22.7-18.4-41.1-41.1-41.1-22.7 0-41.1 18.4-41.1 41.1 0 22.7 18.4 41.1 41.1 41.1z m0-54.9c7.6 0 13.7 6.1 13.7 13.7s-6.1 13.7-13.7 13.7-13.7-6.1-13.7-13.7 6.1-13.7 13.7-13.7z"
                  fill="#FFFFFF"
                  p-id="1801"
                ></path>
              </svg>
            </td>
          </tr>
        </tbody>
      </table>
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
          <span class="text-gray-300 text-sm pl-1">提示: 可以直接把文件拖拽进来</span>
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

