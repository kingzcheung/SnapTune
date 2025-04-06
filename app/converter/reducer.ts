
export const ImageFormat = {
  jpg: "jpg",
  png: "png",
  webp: "webp",
  avif: "avif",
  tiff: "tiff",
  bmp: "bmp",
  gif: "gif",
  jpeg: "jpeg",
  heic: "heic",
  heif: "heif",
  ico: "ico",
  cur: "cur",
  svg: "svg",
}

export type ImageFormatKey = keyof typeof ImageFormat;


// 压缩文件对象
export interface ConverterImageFile {
  raw_path: string;
  save_path: string;
  file_name: string;
  from_format: ImageFormatKey;
  to_format: ImageFormatKey;
  status: 'ready' | "converting" | "converted" | "error";
}

export interface ConverterImageAction {
  type: "added" | "changed" | "clear"; //
  payload: ConverterImageFile;
}

export function converterReducer(
  state: ConverterImageFile[],
  action: ConverterImageAction
) {
  switch (action.type) {
    case "added": {
      return [...state, action.payload];
    }
    case "changed": {
      return state.map((item) => {
        if (item.raw_path === action.payload.raw_path) {
          return { ...item, ...action.payload };
        }
        return item;
      });
    }

    case "clear":
      return [];
    default:
      return state;
  }
}
