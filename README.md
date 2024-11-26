# image2x
[简体中文](./README_CN.md) | [English](./README.md)

## Introduction

`image2x` is a multi-platform image processing tool designed to provide convenient image processing features. Currently, it supports lossy compression of JPG and PNG images, with plans to support more features in the future, such as format conversion.

## Main Features

- [x] Lossy compression of JPG and PNG images
- [ ] Format conversion

## Installation

### From Source Code

1. **Clone the repository**

   ```sh
   git clone https://github.com/kingzcheung/image2x.git
   cd image2x
   ```

2. **Install dependencies**

   Ensure you have the Rust toolchain installed. If not, you can install it using the following command:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Build and run**

   ```sh
   pnpm tauri build
   ```

### From Binary Package

> TODO


## Preview

![Preview](data/screenshot-20241126-174559.png)


## Other

If you want to use the lossy compression feature of the online version, visit [https://tinypng.ximei.me](https://tinypng.ximei.me).
This is a pure front-end website, does not rely on back-end services. Its principle is based on the image compression algorithm implemented by WASM.

## Contribution

Contributions and suggestions are welcome! 

## License

This project is licensed under the GPL3 License.


