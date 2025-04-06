"use client";
import { Button, Link } from "@heroui/react";
import { ArrowLeftRight, PackageMinus, Settings, } from "lucide-react";
import { usePathname } from "next/navigation";

export const Navbar = () => {
  const pathname = usePathname();
  return (
    <div className="p-2 flex flex-col gap-2 max-w-xs">
      <Button
        as={Link}
        color={`default`}
        variant={pathname == '/'?`solid`:`light`}
        href="/"
        className="flex gap-2 justify-start w-full text-zinc-300"
      >
        <PackageMinus className="w-5 h-5 text-violet-500" />
        <span>Compressor</span>
      </Button>
      <Button
        as={Link}
        color="default"
        variant={pathname == '/converter'?`solid`:`light`}
        href="/converter"
        className="flex gap-2 justify-start w-full text-zinc-300"
      >
        <ArrowLeftRight className="w-5 h-5 text-purple-500" />
        <span>File Converter</span>
      </Button>

      <Button
        as={Link}
        color="default"
        variant={pathname == '/settings'?`solid`:`light`}
        href="/settings"
        className="flex gap-2  justify-start w-full text-zinc-300"
      >
        <Settings className="w-5 h-5 text-slate-500" />
        <span>Settings</span>
      </Button>
    </div>
  );
};
