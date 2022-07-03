<script setup></script>
<template>
  <div class="compression ">
    <h1 class="text-2xl divide-white divide-y-2 mb-2">JPEG/PNG 压缩</h1>
    <div class="h-px bg-base-300 dark:bg-gray-600	mb-2"></div>
    <div class="overflow-y-auto -m-3 text-sm	">
      <div class="flex item-center justify-between p-3 align-middle" :class="{'bg-base-100': i % 2 == 0}" v-for="i of 20" :key="i">
        <div class="truncate max-w-xs	">stefen-tan-KYw1eUx1J7Y-unsplashstefen-tan-KYw1eUx1J7Y-unsplash.jpeg</div>
        <div class="truncate w-20 pl-2">JPEG 图像</div>
        <div class="truncate px-2 w-20 text-sm">33.20KB</div>
        <div class="truncate pl-3 w-40 text-green-400 text-sm	">16.55KB (-50.45%)</div>
      </div>
    </div>
    
   
  </div>
</template>