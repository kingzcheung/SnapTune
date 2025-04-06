"use client";

import {
  Button,
  Chip,
  getKeyValue,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow,
} from "@heroui/react";
import { readFile } from "@tauri-apps/plugin-fs";
import { useEffect, useReducer, useState } from "react";
import { compressedReducer, ImageFile } from "@/app/reducer";
import { open, save } from "@tauri-apps/plugin-dialog";
import React from "react";
import { LoaderPinwheel, SendHorizontal, TicketX } from "lucide-react";
import { convertBytes } from "./utils";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { ChooseFile } from "@/components/chooseFile";

interface CompressedFile {
  saved_path: string;
  size: number;
}

const statusColorMap = {
  ready: "default",
  compressing: "primary",
  error: "danger",
  compressed: "success",
};

export default function Home() {
  const [files, dispatch] = useReducer(compressedReducer, []);
  const [compressLoading, setCompressLoading] = useState(false);
  const [savePath, setSavePath] = useState("");

  async function selectFileHandle() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg"],
        },
      ],
    });
    if (Array.isArray(selected)) {
      // user selected multiple files
      for (let file of selected) {
        const fileData = await readFile(file);

        dispatch({
          type: "added",
          payload: {
            file_name: file.split("/").pop() || "",
            raw_path: file,
            size: fileData.length,
            compressedSize: 0,
            savePath: "",
            status: "ready",
          },
        });
      }
    } else if (selected === null) {
      // user
    }
  }

  useEffect(() => {
    getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === "enter") {
        // 清空 files
        dispatch({
          type: "clear",
          payload: {
            file_name: "",
            raw_path: "",
            size: 0,
            compressedSize: 0,
            savePath: "",
            status: "ready",
          },
        });
      }
      if (event.payload.type === "drop") {
        for (let file of event.payload.paths) {
          readFile(file).then((fileData) => {
            dispatch({
              type: "added",
              payload: {
                file_name: file.split("/").pop() || "",
                raw_path: file,
                size: fileData.length,
                compressedSize: 0,
                savePath: "",
                status: "ready",
              },
            });
          });
        }
      }
    });
  }, []);

  async function selectSavePathHandle() {
    const path = await open({
      directory: true,
      multiple: false,
    });

    if (path === null) {
      return;
    }

    setSavePath(path);
  }
  async function compressFileHandle() {
    setCompressLoading(true);

    for (let file of files) {
      if (file.status === "compressed") {
        continue;
      }
      try {
        let compress_file: CompressedFile = await invoke("compress_image", {
          filePath: file.raw_path,
          saveDir: savePath,
        });
        file.compressedSize = compress_file.size;
        file.savePath = compress_file.saved_path;
        file.status = "compressed";
        dispatch({
          type: "changed",
          payload: file,
        });
      } catch (e: any) {
        file.status = "error";
        dispatch({
          type: "changed",
          payload: file,
        });
      }
    }
    setCompressLoading(false);
  }

  const renderCell = React.useCallback((file: ImageFile, columnKey: any) => {
    const cellValue = getKeyValue(file, columnKey);
    const statusColor = (statusColorMap[file.status] ?? "default") as
      | "default"
      | "primary"
      | "danger"
      | "success"
      | "secondary"
      | "warning"
      | undefined;
    switch (columnKey) {
      case "status":
        return (
          <Chip
            className="capitalize"
            color={statusColor}
            size="sm"
            variant="flat"
          >
            {cellValue}
          </Chip>
        );
      case "compressedSize":
        return convertBytes(cellValue);
      case "size":
        return convertBytes(cellValue);
      case "saveup":
        return file.compressedSize
          ? `${(((file.size - file.compressedSize) / file.size) * 100).toFixed(2)}%`
          : "-";
      default:
        return cellValue;
    }
  }, []);

  return (
    <div className="h-full relative">
      {files.length > 0 ? (
        <div className="h-full">
          <div className="flex gap-2 items-center mb-2">
            {/* <div className="flex-1"></div> */}
            <Button
              size="sm"
              className="flex-1 text-left flex items-center justify-start"
              onPress={selectSavePathHandle}
            >
              <span>
                {savePath
                  ? `Save path: ${savePath}`
                  : "Please select save path"}
              </span>
            </Button>

            <Button
              size="sm"
              color="primary"
              className="shadow-2xl "
              onPress={compressFileHandle}
              disabled={compressLoading}
            >
              {compressLoading ? (
                <LoaderPinwheel className="w-5 h-5 animate-spin" />
              ) : (
                <SendHorizontal className="w-5 h-5" />
              )}
              <span>Go!</span>
            </Button>
            <Button
              size="sm"
              className="shadow-2xl "
              onPress={() => {
                dispatch({
                  type: "clear",
                  payload: {
                    file_name: "",
                    raw_path: "",
                    size: 0,
                    compressedSize: 0,
                    savePath: "",
                    status: "ready",
                  },
                });
              }}
            >
              <TicketX className="w-5 h-5" />
              <span>Clear</span>
            </Button>
          </div>
          <Table aria-label="Files">
            <TableHeader>
              <TableColumn key={"file_name"}>File({files.length})</TableColumn>
              <TableColumn width={"100"} key={"size"}>
                Before Size
              </TableColumn>
              <TableColumn width={"120"} key={"compressedSize"}>
                Final Size
              </TableColumn>
              <TableColumn width={"120"} key={"status"}>
                Status
              </TableColumn>
              <TableColumn width={"80"} key={"saveup"}>
                Save Up
              </TableColumn>
            </TableHeader>
            <TableBody>
              {files.map((file: ImageFile) => (
                <TableRow key={file.file_name}>
                  {(columnKey) => (
                    <TableCell>{renderCell(file, columnKey)}</TableCell>
                  )}
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      ) : (
        <div className="h-full pb-4 px-4">
          <div className="h-full mt-2 p-14 bg-zinc-300/20 rounded-xl">
            <ChooseFile onSelectFile={selectFileHandle} />
          </div>
        </div>
      )}
    </div>
  );
}
