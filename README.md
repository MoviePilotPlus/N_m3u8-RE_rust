# N_m3u8-RE_rust

[See English version here](README.en.md)

跨平台的DASH/HLS/MSS下载工具，使用 Rust 重新实现。支持点播、直播(DASH/HLS)。

[![img](https://img.shields.io/github/stars/MoviePilotPlus/N_m3u8-RE_rust?label=星星)](https://github.com/MoviePilotPlus/N_m3u8-RE_rust)  [![img](https://img.shields.io/github/last-commit/MoviePilotPlus/N_m3u8-RE_rust?label=最后提交)](https://github.com/MoviePilotPlus/N_m3u8-RE_rust)  [![img](https://img.shields.io/github/license/MoviePilotPlus/N_m3u8-RE_rust?label=许可证)](https://github.com/MoviePilotPlus/N_m3u8-RE_rust)

---

## 特性

- 完全兼容原 N_m3u8DL-RE 项目的命令行参数
- 高性能的 Rust 实现
- 支持 DASH、HLS 和 MSS 协议
- 点播和直播下载
- 跨平台支持：Windows、macOS、Linux

## 编译与安装

### 前置条件

- Rust 1.70 或更高版本
- Cargo 包管理器

### 编译

```bash
git clone https://github.com/MoviePilotPlus/N_m3u8-RE_rust.git
cd N_m3u8-RE_rust
cargo build --release
```

编译完成后，可执行文件位于 `target/release/N_m3u8DL-RE`。

## 使用

基本用法与原项目一致：

```bash
# 下载点播视频
./N_m3u8DL-RE <input_url>

# 查看帮助
./N_m3u8DL-RE --help
```

## 项目结构

```
N_m3u8-RE_rust/
├── src/
│   ├── commandline/    # 命令行参数解析
│   ├── crypto/         # 加密解密
│   ├── downloader/     # 下载管理
│   ├── entity/         # 数据结构
│   ├── i18n/           # 国际化
│   ├── muxer/          # 媒体混流
│   ├── parser/         # 媒体流解析
│   ├── utils/          # 工具函数
│   └── main.rs         # 主入口
├── tests/              # 测试用例
├── Cargo.toml          # 项目配置
└── README.md           # 项目说明
```

## 致谢

- 原项目：[nilaoda/N_m3u8DL-RE](https://github.com/nilaoda/N_m3u8DL-RE)
