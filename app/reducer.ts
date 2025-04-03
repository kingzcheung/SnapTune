// 压缩文件对象
export interface ImageFile {
    raw_path: string;
    savePath: string;
    file_name: string;
    size: number;
    compressedSize: number;
    // 未准备好|压缩中|压缩完成|错误
    status: 'ready' | 'compressing' | 'compressed'| 'error';
}
export interface CompressedAction {
    type: 'added' | 'changed'|'clear'; //
    payload: ImageFile;
}

export function compressedReducer(state: ImageFile[], action: CompressedAction) {
    switch (action.type) {
        case 'added': {
            return [
                ...state,
                action.payload
            ];
        }
        case 'changed': {
            return state.map((item) => {
                if (item.raw_path === action.payload.raw_path) {
                    return action.payload;
                } else {
                    return item;
                }
            });
        }
        case 'clear': {
            return [];
        }

        default:
            return state;
    }
}