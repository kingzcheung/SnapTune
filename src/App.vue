<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { ScrollArea } from '@/components/ui/scroll-area'


const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}

</script>

<template>
  <main class="container">
    <h1 class="text-2xl text-center mb-4">Image to Tiny</h1>
    <div class="flex flex-col gap-2 items-center justify-center py-12 rounded-3xl border-dashed border-2">
      <div class="text-xl">Drag and drop image files</div>
      <div class="text-base text-gray-400 mb-8">支持以下格式：png, jpg, jpeg</div>
      <Button variant="outline" class="w-24">Select files</Button>
    </div>
    <div>
      <div class="flex flex-wrap gap-4 mt-4">
        All files
      </div>
      <ScrollArea class="h-[200px] w-full rounded-md border p-4">
        <Table>
          <TableCaption>A list of your recent invoices.</TableCaption>
          <TableHeader>
            <TableRow>
              <TableHead class="w-[100px]">
                文件名
              </TableHead>
              <TableHead>大小</TableHead>
              <TableHead class="text-right">
                已节省
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="i in 10" :key="i">
              <TableCell class="font-medium">
                INV001.jpg
              </TableCell>
              <TableCell>18mb</TableCell>
              <TableCell class="text-right">
                89%
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  </main>
</template>
