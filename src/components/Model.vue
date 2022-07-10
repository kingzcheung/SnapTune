<!--
 Copyright 2022 kingzcheung
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

<script setup>
import { onMounted, ref } from 'vue';


let props = defineProps({
    modelValue: Boolean
})


let emits = defineEmits(['update:modelValue','click'])

const handleClick = (event)=>{
    emits('update:modelValue',false);
    emits('click');
}

onMounted(()=>{
    
})

</script>
<template>
  <div class="fixed inset-0 flex justify-center items-center" v-if="props.modelValue">
    <div class="modal-box">
      <h3 class="font-bold text-lg mb-2"><slot name="header">Header</slot></h3>
      <slot></slot>
      <div class="modal-action">
        <button class="btn btn-sm" @click="handleClick">确认</button>
      </div>
    </div>
  </div>
</template>