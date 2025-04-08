"use client";

import { NumberInput, Slider } from "@heroui/react";

export default function DocsPage() {
  return (
    <div className="h-full w-full mt-2 p-4 bg-zinc-300/20 rounded-xl">
      <h2 className="text-2xl">settings</h2>
      <div className="mb-4 flex items-center justify-between">
        <div className="text-zinc-600/70 dark:text-zinc-300/70">压缩图片质量</div>
        <NumberInput
          className="max-w-xs"
          label="Quality"
          step={5}
          max={100}
          min={70}
        />
      </div>
    </div>
  );
}
