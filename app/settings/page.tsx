"use client";

import { NumberInput } from "@heroui/react";
import { Store } from "@tauri-apps/plugin-store";
import { useEffect, useLayoutEffect, useRef, useState } from "react";

interface SettingsInterface {
  quality: number;
}

export default function DocsPage() {
  const [settings, setSettings] = useState<SettingsInterface>({
    quality: 100,
  });
  const storeRef = useRef<Store | undefined>(undefined);

  useEffect(() => {
    let isMounted = true;
    let fn = async () => {
      const store = await Store.load("settings.json");
      storeRef.current = store;
      const storedSettings = await store.get<SettingsInterface>("settings");
      if (isMounted && storedSettings) {
        console.log("storedSettings", storedSettings);
        setSettings({
          ...storedSettings,
        });
      }
    };
    fn();
    return () => {
      isMounted = false;
    };
  }, []);

  useLayoutEffect(() => {
    const store = storeRef.current;
    if (store) {
      console.log("保存设置:", settings);
      store.set("settings", settings);
      store.save();
    }
  }, [settings]);

  // set quality
  async function handleQualityChange(value: number) {
    if (storeRef.current) {
      setSettings({
        ...settings,
        quality: value,
      });
      console.log("settings", settings);
    }
  }

  return (
    <div className="h-full w-full mt-2 p-4 bg-zinc-300/20 rounded-xl">
      <h2 className="text-3xl mb-4">settings</h2>
      <div className="mb-4 flex items-start justify-between gap-2">
        <div className="text-zinc-600/70 dark:text-zinc-200/70">
          <h2 className="text-lg">Compressed image quality</h2>
          <span className="text-sm text-zinc-600/40 dark:text-zinc-200/40">The smaller the value, the smaller the image capacity, but the lower the image quality.</span>
        </div>
        <NumberInput
          className="max-w-3xs"
          label="Quality"
          defaultValue={100}
          value={settings.quality}
          step={5}
          max={100}
          min={70}
          onValueChange={(v) => handleQualityChange(v)}
        />
      </div>
    </div>
  );
}
