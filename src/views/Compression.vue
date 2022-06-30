<script setup></script>
<template>
  <div class="compression">
    <h1 class="text-2xl">JPEG/PNG 压缩</h1>

    <div
      class="
        absolute
        z-50
        left-0
        right-0
        bottom-0
        bg-neutral
        w-full
        flex
        items-stretch
        py-2
        pl-2
      "
    >
      <div class="flex items-center">
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
        <span class="text-gray-500 text-sm pl-1"
          >提示: 直接把文件拖拽进来即可</span
        >
      </div>
    </div>
  </div>
</template>