"use client";

import { Button, PressEvent } from "@heroui/button";

export const ChooseFile = ({
    onSelectFile,
    description,
}:{
    description?: string,
    onSelectFile: (e: PressEvent) => void,

}) => {

  return (
    <Button
                  className="w-full h-2/3 group transition-all ease-in-out flex flex-col border-2 p-8 border-dashed border-zinc-400 hover:border-blue-600"
                  onPress={onSelectFile}
                  disableRipple
                  disableAnimation
                >
                  <div className="opacity-50">
                    <svg
                      className="icon w-full"
                      viewBox="0 0 1154 1024"
                      version="1.1"
                      xmlns="http://www.w3.org/2000/svg"
                      width="128"
                      height="128"
                    >
                      <path
                        d="M628.443429 101.400381a130.194286 130.194286 0 0 1 99.905015 13.417651 132.209778 132.209778 0 0 1 61.383112 80.530285l124.960507 471.12127a133.501968 133.501968 0 0 1-13.255111 100.636445 131.494603 131.494603 0 0 1-79.798857 61.992635l-465.034158 125.87073a130.21054 130.21054 0 0 1-99.896889-13.417651 132.185397 132.185397 0 0 1-61.358731-80.522159l-124.993015-471.12127a133.477587 133.477587 0 0 1 13.279492-100.644571 131.478349 131.478349 0 0 1 79.806984-62.008889l465.001651-125.854476z"
                        fill="#DFE8FF"
                      ></path>
                      <path
                        d="M975.579429 37.912381c39.196444 0 74.873905 16.205206 100.742095 42.300952a144.075175 144.075175 0 0 1 41.788952 101.562921V714.930794c0 39.537778-16.002032 75.483429-41.82146 101.554793a141.409524 141.409524 0 0 1-100.733968 42.276572H449.29219a141.442032 141.442032 0 0 1-100.742095-42.276572 144.067048 144.067048 0 0 1-41.788952-101.554793V181.776254c0-39.545905 16.002032-75.491556 41.813333-101.562921A141.531429 141.531429 0 0 1 449.316571 37.953016l526.262858-0.032508z"
                        fill="#F2F3FF"
                      ></path>
                      <path
                        d="M442.920635 264.167619c0 39.822222 21.008254 76.63746 55.165968 96.548571a109.437968 109.437968 0 0 0 110.364445 0 111.624127 111.624127 0 0 0 55.198476-96.548571c0-61.570032-49.42019-111.485968-110.364445-111.485968-60.968635 0-110.364444 49.915937-110.364444 111.485968z"
                        fill="#B5C7FF"
                      ></path>
                      <path
                        d="M997.953016 454.810413a122.725587 122.725587 0 0 0-134.395937-26.900318 122.725587 122.725587 0 0 0-39.952254 26.900318l-379.091301 382.927238-6.395937 6.460952h535.860826c70.363429 0 127.943111-58.189206 127.943111-129.259682V561.436444l-103.968508-106.626031z"
                        fill="#8EABFF"
                      ></path>
                      <path
                        d="M854.007873 844.198603L582.06273 567.897397a122.766222 122.766222 0 0 0-87.178159-36.352 122.676825 122.676825 0 0 0-87.178158 36.352l-84.764445 87.251301v59.790223c0 71.070476 57.58781 129.259683 127.951238 129.259682h403.114667z"
                        fill="#618DFF"
                        opacity=".247"
                      ></path>
                    </svg>
                  </div>
                  <div className="text-xl text-zinc-200/60 group-hover:text-zinc-200 transition-all ease-in-out">
                   {description ? description :"Drag & drop or click to choose image files"}
                  </div>
                </Button>
  );
};
