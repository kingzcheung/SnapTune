<script setup>
import { ref } from 'vue';


const inBox = ref(false)

const dragoverHandle = ()=>{
    console.log("拖动中")
    inBox.value = true
}
const dragleaveHandle = ()=>{
    inBox.value = false
}

</script>
<template>
    <div class="px-40 py-10 rounded-2xl border-2 border-gray-300 border-dashed text-center bg-indigo-50"
        :class="{'bg-indigo-200 border-indigo-200 animate-pulse':inBox}"
     @dragover="dragoverHandle" @dragleave="dragleaveHandle">
        <div class="p-5 text-center flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" width="192" height="128" fill="none" viewBox="0 0 192 128"><g clip-path="url(#clip0)"><rect width="192" height="128" fill="#EEF2FF" rx="12"/><circle cx="78" cy="28" r="12" fill="#C7D2FE"/><path fill="#6366F1" d="M52 48L84 80H100L140 112H-12L52 48Z"/><path fill="#818CF8" d="M116 32L132 48H148L212 104H44L116 32Z"/><path fill="#A5B4FC" d="M116 38V52L100 68L144 104H50L116 38Z"/><rect width="192" height="40" y="96" fill="#C7D2FE"/><rect width="184" height="120" x="4" y="4" stroke="#6366F1" stroke-opacity=".25" stroke-width="8" rx="8"/><rect width="188" height="124" x="2" y="2" stroke="#EEF2FF" stroke-opacity=".5" stroke-width="4" rx="10"/></g><defs><clipPath id="clip0"><rect width="192" height="128" fill="#fff" rx="12"/></clipPath></defs></svg>
        </div>
        <div class="text-base mb-2">将图片拖拽到此处</div>
        <div class="text-gray-400 text-sm">支持常用格式，如: JPEG/PNG/WEBP/GIF/BMP</div>
    </div>
</template>