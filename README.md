# Xiaozhi AI Slint GUI

本项目是为 [xiaozhi_linux_rs](https://github.com/Hyrsoft/xiaozhi_linux_rs) 提供的一个示例 GUI 客户端实现。它基于高性能的 Rust UI 框架 [Slint](https://github.com/slint-ui/slint) 构建。

这是一个纯净的显示与交互层，它不处理复杂的音频录制或唤醒逻辑，而是通过 **UDP Socket** 与 `xiaozhi_linux_rs` 的核心进程 (Core) 进行通讯，实现状 态的实时可视化。

## 功能特性

- 界面风格明亮可爱，支持流畅的状态切换动画。
- 动态显示设备不同工作状态：
  - 待机空闲 (Online)
  - 离线/断开 (Disconnected)
  - 正在倾听 (Listening)
  - 正在说话 (Speaking)
- 实时显示 TTS 云端播报的对话框文本。
- 接收并展示 Toast 通知消息和配网激活码。
- 支持触摸/点击事件，主动向 Core 发送打断 (`abort`) 等控制指令。

## 运行前配置选项

为了让 GUI 正确地和 Core 进程通信以及在系统中正常渲染，在编译/运行之前可能需要修改以下配置：

### 1. 通信 IP 与端口配置 (`src/main.rs`)
GUI 和 Core 通过预定的端口进行双向 UDP 通信。
在 `src/main.rs` 的顶部找到以下常量进行修改：

```rust
// GUI 监听的地址和端口（接收来自 Core 的状态、TTS文本等信息）
const GUI_LISTEN_ADDR: &str = "0.0.0.0:5679";

// Core 进程监听的地址和端口（向 Core 发送点击打断等控制指令）
const CORE_TARGET_ADDR: &str = "192.168.8.235:5678";
```
*请确保 `CORE_TARGET_ADDR` 的 IP 地址与运行 Core 进程的物理设备 IP 相匹配。*

### 2. 系统字体配置 (`ui/main.slint`)
为了确保中文字符不会显示为"方块"乱码，需要在 Slint UI 中指定系统可用的中文字体。
在 `ui/main.slint` 的开头：

```slint
export component MainWindow inherits Window {
    // ...
    default-font-family: "Hiragino Sans GB"; // 根据你的操作系统环境修改
}
```
* macOS 推荐使用 `"PingFang SC"` 或 `"Hiragino Sans GB"` 或 `"STHeiti"` 
* Windows 推荐使用 `"Microsoft YaHei"` 或 `"SimHei"`
* Linux 会根据你的系统环境而定，如 `"Noto Sans CJK SC"` 或 `"WenQuanYi Micro Hei"`

## 编译与运行

本项目基于 Rust 编写，首先请确保你已经安装了 [Rust 编译环境](https://www.rust-lang.org/tools/install)。

```bash
# 检查并运行 (开发模式)
cargo run

# 编译为发布版本 (Release)
cargo build --release
```

## 嵌入式交叉编译 (ARM Linux + DRM/KMS)

本项目支持使用 [cross](https://github.com/cross-rs/cross) 交叉编译到嵌入式 ARM Linux 设备，生成 musl 静态链接的二进制文件，使用 DRM/KMS 直接渲染到屏幕（无需 X11/Wayland）。

### 1. 安装前置依赖

```bash
# 安装 Docker（cross 依赖 Docker 容器进行交叉编译）
# 参考: https://docs.docker.com/get-docker/

# 安装 cross
cargo install cross --git https://github.com/cross-rs/cross

# 添加 Rust 目标
rustup target add armv7-unknown-linux-musleabihf
```

### 2. 编译

```bash
# 使用一键脚本
./cross_build.sh          # 默认 release 模式
./cross_build.sh debug    # debug 模式

# 或手动执行
cross build --target armv7-unknown-linux-musleabihf \
    --features embedded --no-default-features --release
```

### 3. 部署与运行

```bash
# 将编译产物复制到设备
scp target/armv7-unknown-linux-musleabihf/release/slint_xiaozhi_gui user@device:/path/to/

# 在设备上运行（全屏 DRM/KMS 渲染）
SLINT_FULLSCREEN=1 ./slint_xiaozhi_gui

# 指定 DRM 设备（默认 /dev/dri/card0）
SLINT_FULLSCREEN=1 DRM_DEVICE=/dev/dri/card0 ./slint_xiaozhi_gui
```

> **提示**：运行时需要对 `/dev/dri/card0` 有读写权限，通常需要将用户加入 `video` 组或使用 `sudo`。

## 协议

本项目基于 **GPL-3.0** 许可证开源。请参阅项目根目录下的 [LICENSE](LICENSE) 文件了解更多详情。
