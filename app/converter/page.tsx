"use client";

import {
  addToast,
  Button,
  Chip,
  getKeyValue,
  Select,
  SelectItem,
  Table,
  TableBody,
  TableCell,
  TableColumn,
  TableHeader,
  TableRow,
  Tooltip,
} from "@heroui/react";
import { readFile } from "@tauri-apps/plugin-fs";
import { useEffect, useReducer, useState } from "react";
import {
  converterReducer,
  ConverterImageFile,
  ImageFormatKey,
} from "@/app/converter/reducer";
import { open } from "@tauri-apps/plugin-dialog";
import React from "react";
import {
  DeleteIcon,
  LoaderPinwheel,
  SendHorizontal,
  TicketX,
  Trash2,
} from "lucide-react";
import { convertBytes } from "../utils";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { ChooseFile } from "@/components/chooseFile";

interface ConverterFileResponse {
  saved_path: string;
  size: number;
}

const statusColorMap = {
  ready: "default",
  converting: "primary",
  error: "danger",
  converted: "success",
};

const formats = [
  { key: "jpg", label: "JPEG" },
  { key: "png", label: "PNG" },
  { key: "webp", label: "WEBP" },
  {key: "avif", label: "AVIF"},
  {key: "gif", label: "GIF"},
  {key: "bmp", label: "BMP"},
  {key: "tiff", label: "TIFF"},
  {key: "exr", label: "EXR"},
  {key: "hdr", label: "HDR"},
];

export default function Converter() {
  const [files, dispatch] = useReducer(converterReducer, []);
  const [converterLoading, setConverterLoading] = useState(false);
  const [savePath, setSavePath] = useState("");
  const [allToFormat, setAllToFormat] = useState<ImageFormatKey>("jpg");

  async function selectFileHandle() {
    const selected = await open({
      multiple: true,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "jpg", "bmp"],
        },
      ],
    });
    if (Array.isArray(selected)) {
      // user selected multiple files
      for (let file of selected) {
        // 获取文件的类型
        const ext: string = await invoke("get_file_type", {
          filePath: file,
        });

        dispatch({
          type: "added",
          payload: {
            file_name: file.split("/").pop() || "",
            raw_path: file,
            from_format: ext as ImageFormatKey,
            to_format: "jpg",
            save_path: "",
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
            from_format: "jpg",
            to_format: "jpg",
            save_path: "",
            status: "ready",
          },
        });
      }
      if (event.payload.type === "drop") {
        for (let file of event.payload.paths) {
          readFile(file).then((fileData) => {
            // 获取文件的类型
            invoke("get_file_type", {
              filePath: file,
            }).then((ext) => {
              dispatch({
                type: "added",
                payload: {
                  file_name: file.split("/").pop() || "",
                  raw_path: file,
                  save_path: "",
                  from_format: ext as ImageFormatKey,
                  to_format: "jpg",
                  status: "ready",
                },
              });
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
  async function convertFileHandle() {
    setConverterLoading(true);

    for (let file of files) {
      if (file.status === "converted") {
        continue;
      }
      try {
        let resp: ConverterFileResponse = await invoke("convert", {
          filePath: file.raw_path,
          toFormat: file.to_format,
          saveDir: savePath,
        });
        file.save_path = resp.saved_path;
        file.status = "converted";
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
        addToast({
          color: "danger",
          title: "Error",
          description: `Cant convert ${file.file_name} 
                        Error: ${e}`,
        });
      }
    }
    setConverterLoading(false);
  }

  const renderCell = React.useCallback(
    (file: ConverterImageFile, columnKey: any) => {
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
        case "actions": {
          return (
            <div className="relative flex items-center gap-2">
              <Tooltip color="danger" content="Delete user">
                <span className="text-lg text-danger cursor-pointer active:opacity-50">
                  <Trash2 className="w-5 h-5" />
                </span>
              </Tooltip>
            </div>
          );
        }
        case "from_format": {
          return <span className="uppercase line-through">{cellValue}</span>;
        }
        case "to_format": {
          return (
            <Chip color="warning" variant="flat" className="uppercase">
              {cellValue}
            </Chip>
          );
        }
        default:
          return cellValue;
      }
    },
    []
  );

  return (
    <div className="h-full relative w-full">
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
            <Select
              size="sm"
              className="max-w-40"
              selectedKeys={[allToFormat]}
              onChange={(e) => {
                setAllToFormat(e.target.value as ImageFormatKey);
                dispatch({
                  type: "set_all_to_format",
                  payload: {
                    to_format: e.target.value as ImageFormatKey,
                    raw_path: "",
                    save_path: "",
                    file_name: "",
                    from_format: "jpg",
                    status: "ready",
                  },
                });
              }}
            >
              {formats.map((format) => (
                <SelectItem key={format.key}>{format.label}</SelectItem>
              ))}
            </Select>
            <Button
              size="sm"
              color="primary"
              className="shadow-2xl "
              onPress={convertFileHandle}
              disabled={converterLoading}
            >
              {converterLoading ? (
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
                    from_format: "jpg",
                    to_format: "jpg",
                    save_path: "",
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
              <TableColumn width={"100"} key={"from_format"}>
                Before Format
              </TableColumn>
              <TableColumn width={"120"} key={"to_format"}>
                Final Format
              </TableColumn>
              <TableColumn width={"120"} key={"status"}>
                Status
              </TableColumn>
              <TableColumn width={"80"} key={"actions"}>
                Actions
              </TableColumn>
            </TableHeader>
            <TableBody>
              {files.map((file: ConverterImageFile) => (
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
