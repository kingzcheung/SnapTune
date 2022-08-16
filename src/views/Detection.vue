<script setup>
import { ref, onMounted, computed } from 'vue';
import { open, message } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { readDir, BaseDirectory } from '@tauri-apps/api/fs';
import { convertFileSrc } from '@tauri-apps/api/tauri';

const detection = ref({
    onnx_file: '',
    from: '',
    from_files: [],
    output: '',
});

String.prototype.extension = function () {
    var ext = null;
    var name = this.toLowerCase();
    var i = name.lastIndexOf(".");
    if (i > -1) {
        var ext = name.substring(i);
    }
    return ext;
}
const imgExt = new Array(".png", ".jpg", ".jpeg");


const currentProgress = ref(-1);

const percentageProgress = computed(()=>{
    if (currentProgress.value < 0) {
        return 0
    }
    const len = detection.value.from_files.length;
    console.log(currentProgress.value,len)
    if (len == 0) {
        return 0
    }
    let res =  currentProgress.value / len * 100;

    return res.toFixed(2)
})

const showImage = ref([])
const showImageUrls = computed(()=>{
    return showImage.value.map(v=>{
        return convertFileSrc(v);
    })
})

const chooseOnnxFile = async () => {
    const selected = await open({
        directory: false,
        multiple: false,
        defaultPath: await downloadDir(),
    });
    if (selected) {
        let ext = selected.extension()
        if (ext == '.onnx') {
            detection.value.onnx_file = selected;
        } else {
            await message('请选择正确的 ONNX 文件', { title: '选择文件错误', type: 'error' });
        }
    }
}
const chooseImportDir = async () => {
    const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: await downloadDir(),
    });
    if (selected) {
        detection.value.from = selected;
        const entries = await readDir(selected);

        detection.value.from_files = entries.map(v => v.path).filter(v => {
            let ext = v.extension()
            console.log(ext)
            return [".png", ".jpg", ".jpeg"].indexOf(ext) > -1
        });

    }
}
const chooseOutputDir = async () => {
    const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: await downloadDir(),
    });
    if (selected) {
        detection.value.output = selected;
    }
}

const logs = ref([])

const startHandle = () => {
    currentProgress.value = 0
    detection.value.from_files.forEach(v => {
        invoke("detection_command", {
            onnxFile: detection.value.onnx_file,
            from: v,
            to: detection.value.output,
        }).then((res) => {
            logs.value.push(res)
            if (res.extra.length > 0) {
                showImage.value = res.extra
            }
        }).catch((error) => {
            logs.value.push({message:error,extra:[],msgType:"Error"})
        }).finally(() => {
            currentProgress.value++
        });
    })
}

</script>
<template>
    <div class="text-sm">
        <div class="border-l-4 pl-2 leading-8 font-bold border-indigo-500 mb-2 text-lg">商品裁剪(YOLOV5)</div>
        <div class="overflow-y-auto bg-white rounded-lg p-4">
            <div class="flex items-center mb-2">
                <div class="mr-2 w-24">ONNX 模型: </div>
                <div class="mx-2" v-if="detection.onnx_file">{{ detection.onnx_file }}</div>
                <button class="btn btn-sm" @click="chooseOnnxFile">选择yolov5模型</button>
            </div>

            <div class="flex items-center mb-2">
                <div class="mr-2 w-24">输入目录: </div>
                <div class="mx-2" v-if="detection.from">{{ detection.from }}</div>
                <button class="btn btn-sm mr-2" @click="chooseImportDir">选择目录</button>

            </div>
            <div class="flex items-center mb-2">
                <div class="mr-2 w-24">输出目录: </div>
                <div class="mx-2" v-if="detection.output">{{ detection.output }}</div>
                <button class="btn btn-sm" @click="chooseOutputDir">请选择</button>
            </div>

            <div class="flex items-center mb-4">
                <button class="btn btn-sm" v-if="currentProgress == -1">
                    <svg t="1660221713126" class="icon" viewBox="0 0 1024 1024" version="1.1"
                        xmlns="http://www.w3.org/2000/svg" p-id="3612" width="16" height="16">
                        <path
                            d="M924.8 337.6c-22.6-53.4-54.9-101.3-96-142.4-41.1-41.1-89-73.4-142.4-96C631.1 75.8 572.5 64 512 64S392.9 75.8 337.6 99.2c-53.4 22.6-101.3 54.9-142.4 96s-73.4 89-96 142.4C75.8 392.9 64 451.5 64 512s11.8 119.1 35.2 174.4c22.6 53.4 54.9 101.3 96 142.4 41.1 41.1 89 73.4 142.4 96C392.9 948.2 451.5 960 512 960s119.1-11.8 174.4-35.2c53.4-22.6 101.3-54.9 142.4-96 41.1-41.1 73.4-89 96-142.4C948.2 631.1 960 572.5 960 512s-11.8-119.1-35.2-174.4z m-55 325.5c-19.6 46.2-47.6 87.8-83.2 123.4s-77.2 63.7-123.4 83.2c-47.8 20.2-98.7 30.5-151.1 30.5-52.4 0-103.3-10.3-151.1-30.5-46.2-19.6-87.8-47.6-123.4-83.2s-63.7-77.2-83.2-123.4c-20.2-47.8-30.5-98.7-30.5-151.1 0-52.4 10.3-103.3 30.5-151.1 19.6-46.2 47.6-87.8 83.2-123.4s77.2-63.7 123.4-83.2c47.8-20.2 98.7-30.5 151.1-30.5 52.4 0 103.3 10.3 151.1 30.5 46.2 19.6 87.8 47.6 123.4 83.2s63.7 77.2 83.2 123.4c20.2 47.8 30.5 98.7 30.5 151.1 0 52.4-10.3 103.3-30.5 151.1z"
                            fill="#e6e6e6" p-id="3613"></path>
                        <path d="M381 704l302-192-302-192z" fill="#e6e6e6" p-id="3614"></path>
                    </svg>
                    <span class="ml-1" @click="startHandle">开始</span>
                </button>
                <div class="mr-2 w-24" v-if="currentProgress >= 0">进度: </div>
                <div class="mx-2 flex-1 flex items-center" v-if="currentProgress >= 0">
                    <progress class="progress progress-primary w-full" :value="currentProgress"
                        :max="detection.from_files.length"></progress>
                        <span class="ml-2">{{percentageProgress}}%</span>
                </div>
            </div>

            <div class="p-4 bg-gray-600 text-gray-300 rounded-lg overflow-x-hidden max-h-64 mb-4">
                <p class="overflow-ellipsis truncate" v-if="logs.length > 0" v-for="(log, i) of logs" :key="i">
                    <span class="mr-1"> > </span>
                    <span class="{'text-yellow-400':log.msg_type=='Warning','text-green-600':log.msg_type=='Success'}">{{ log.message }}</span>
                </p>
                <p v-else>暂无日志</p>
            </div>
            <div class="flex items-center justify-center">
                <div v-for="(img,i) of showImageUrls" :key="i" class="max-w-xs">
                    <img :src="img" />
                </div>
               
            </div>
        </div>
    </div>
</template>