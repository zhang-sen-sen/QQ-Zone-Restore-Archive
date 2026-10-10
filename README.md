<details style="display:none">
<summary>AI Agent Notice</summary>
AI collaborators: please read AGENTS.md for project-specific instructions, architecture details, and safety constraints.
</details>

# QQ 空间恢复归档 · 便携版 v2.0.0

[![Rust](https://img.shields.io/badge/Rust-1.77%2B-orange)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://v2.tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

将 QQ 空间动态、照片、视频与互动记录安全归档到本地的桌面 / 移动端工具。
本仓库为 **QzoneArchive 的本地定制分支（v2.0.0）**，在原版基础上修复了 18 项媒体模块缺陷并新增多项功能，发布 **Windows（安装版 / 便携版）、macOS、Linux、Android 与 iOS（未签名 IPA）多端版本**。

> [!IMPORTANT]
> 本项目基于 [Gaoshu705/QzoneArchive](https://github.com/Gaoshu705/QzoneArchive) 与 [xiaosu19/QQ-Zone-Restore-Archive](https://github.com/xiaosu19/QQ-Zone-Restore-Archive)（v1.1.0，commit `e9dcb73`）二次开发，并参考了 [LibraHp/GetQzonehistory](https://github.com/LibraHp/GetQzonehistory)、[ShunCai/QZoneExport](https://github.com/ShunCai/QZoneExport)、[salt-fishes/qzone-archiver](https://github.com/salt-fishes/qzone-archiver)、[11273/QzonePhoto](https://github.com/11273/QzonePhoto) 与 [Gu-Heping/onebot-qzone](https://github.com/Gu-Heping/onebot-qzone) 的历史取数、空间资料接口、评论正文和昵称解析思路。QZoneExport 参考实现遵循 Apache-2.0；详见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。原项目作者、参考项目作者和腾讯公司均不对本分支提供背书或担保。

> [!WARNING]
> 本项目不是腾讯、QQ 或 QQ 空间官方产品。所谓“恢复已删除说说”仅指：当已删除内容仍残留在点赞、评论、回复等互动记录中时，尝试还原其中可取得的正文和媒体信息；没有互动痕迹、已被服务端彻底清除、无权访问或接口不再返回的内容无法恢复，也不保证归档结果完整。请仅处理本人账号或已获得充分授权的内容，并自行承担账号限制、第三方接口变化、数据遗漏和本地数据保管风险。

## 下载与安装

请从本仓库 [Releases](https://github.com/zhang-sen-sen/QQ-Zone-Restore-Archive/releases) 下载：

| 资产 | 说明 |
| --- | --- |
| `QQ-Zone-Archive-2.0.0-setup.exe` | **Windows 安装版**（NSIS，默认安装到 `D:\QQ空间恢复归档`，中文开始菜单） |
| `QQ-Zone-Archive-v2.0.0-portable.exe` | **Windows 便携版**：单文件 EXE，无需安装，解压即用（**必须解压后运行**） |
| `QQ-Zone-Archive-v2.0.0-macos-x64.dmg` | **macOS Intel**（x64）安装包 |
| `QQ-Zone-Archive-v2.0.0-macos-aarch64.dmg` | **macOS Apple Silicon**（arm64）安装包 |
| `QQ-Zone-Archive-v2.0.0-linux-amd64.AppImage` | **Linux x86_64** 免安装版 |
| `QQ-Zone-Archive-v2.0.0-linux-amd64.deb` | **Linux x86_64**（Debian / Ubuntu 系） |
| `QQ-Zone-Archive-v2.0.0-linux-x86_64.rpm` | **Linux x86_64**（Fedora / RHEL 系） |
| `QQ-Zone-Archive-v2.0.0-android.apk` | **Android 版**（arm64-v8a，已签名） |
| `QQ-Zone-Archive-v2.0.0-ios-unsigned.ipa` | **iOS 未签名 IPA**（iPhone + iPad 通用，需自行签名后安装） |
| `QQ-Zone-Archive-v2.0.0-source.zip` | 完整源代码（含修改记录、对比文档与许可证） |

### 使用说明

- **Windows 便携版**：请先把压缩包解压到本地文件夹再运行 EXE；在压缩包内直接双击会被检测并提示「请先解压后再运行」，不会进入主界面
- **Windows 安装版**：NSIS 向导默认安装到 `D:\QQ空间恢复归档`（软件需要写入安装目录，不建议装到 Program Files）；开始菜单文件夹为「QQ空间恢复归档」
- **macOS**：按芯片选择 x64 或 aarch64 的 dmg；首次启动如提示「无法验证开发者」，请在系统设置「隐私与安全性」中允许打开
- **Linux**：AppImage 执行 `chmod +x` 后运行；deb / rpm 使用系统包管理器安装；NixOS 用户请使用源码构建
- **Android**：下载 apk 后允许「安装未知来源应用」即可安装
- **iOS**：未签名 IPA 无法直接安装到设备，需使用 Apple 开发者账号（或自签名工具）签名后安装；也可在 Mac 上用于模拟器调试
- 运行环境（Windows）：Windows 10 / 11（需 Microsoft Edge WebView2，Win10+ 通常自带）
- 左侧「空间资料 → 相册 / 视频」点击「从 QQ 空间读取」同步；支持扫码登录与网页登录
- 归档任务在「归档工作台 → 归档任务」查看进度，支持断点续传
- 归档完成（或从「说说归档」归档动态）后，可浏览、搜索、导出 HTML，媒体可在应用内直接查看 / 播放

### 数据存放位置

- **所有数据都保存在 EXE 所在目录**（数据库、登录会话、媒体文件），随软件整体迁移与备份
- 数据按登录的 QQ 账号分目录存放，每个账号一个独立文件夹（`qq_QQ号`），互不干扰：

  ```
  软件目录/
  └── qq_QQ号/
      ├── qzone-archive.sqlite3   # 该账号的归档数据库（按账号分开）
      ├── images/                 # 图片（归档原图 + 相册下载图片）
      └── videos/                 # 视频（归档视频 + 相册下载视频）
  ```

- 切换登录账号后自动读写对应账号目录；首次使用新版登录时，旧版存放在软件根目录的数据库与 images / videos 会自动迁移到当前账号目录
- 不再生成「媒体缓存」文件夹：下载的媒体按类型自动归入 images / videos
- 缓存媒体文件按「上传时间」命名（如 `2019-02-08_10-50.jpg`，同一时间多个媒体带短哈希后缀区分），与界面卡片时间一致
- 删除软件不会自动清除归档数据；彻底清除请在「工具与设置 → 设置 → 清除全部应用数据」（仅清除当前登录账号目录）

### 登录与安全

- 登录凭据保存在操作系统凭据保管库（Windows 凭据管理器），不写入数据库、日志或前端存储
- 归档过程中请勿切换 QQ 客户端账号，以免触发风控；频繁限流时建议稍后再继续

## 功能

- **多来源恢复**：合并当前可见说说、旧历史消息残留和移动端互动通知，尽量找回仍被 QQ 接口保留的本人说说、好友动态与留言线索
- **结构化互动**：还原点赞用户、评论、回复人与被回复人，补全可取得的昵称，并提供联系人排行和评论往来视图
- **深度扫描与续传**：顺序探测历史记录，在最后命中后继续验证空尾；被限流或中断时保留断点和已经写入的数据
- **资料独立归档**：相册、相册照片、独立视频、留言板与 QQ 空间网页端旧收藏分别分页同步，不与说说混在一起
- **原图 / 原视频优先**：说说图片按质量分稳定排序下载原图，psc 地址优先原图变体 `/o`；相册视频解析真实播放直链并在应用内播放，403 签名过期自动刷新
- **媒体整理**：按年份浏览说说照片和视频，缓存文件按上传时间命名，相册 / 说说同图内容级去重、不重复下载
- **多账号隔离**：数据库与媒体按 QQ 账号分目录（`qq_QQ号`），旧数据自动迁移
- **本地优先**：SQLite 数据库、媒体缓存和导出文件都保存在软件目录（便携版为 EXE 旁）；登录会话只进入操作系统安全凭据库
- **检索与导出**：支持全文搜索、年份筛选、时间排序、批量管理与离线 HTML 导出
- **桌面体验**：面向大数据量重新设计紧凑双列卡片、资料导航、暗色模式和窄屏布局
- **跨平台发行**：提供 Windows 便携版（单 EXE）、Android APK、macOS / Linux 构建与 iOS 工作流

完整版本历史请查看 [CHANGELOG.md](CHANGELOG.md)。

## 修改记录（相对上游 v1.1.0，共 18 项）

> 详细对比与代码级说明见仓库根目录 **[《与原作者的代码对比及全部更改记录.md》](与原作者的代码对比及全部更改记录.md)**。

| # | 功能 / 修复 |
| --- | --- |
| 1 | 修复「相册打开任何照片都显示第一张缓存图」（URL 哈希唯一命名缓存） |
| 2 | 新增图片查看器（ImageViewer）：放大 / 缩小 / 旋转 90° / 复位 / 保存 |
| 3 | 修复「相册里实际是视频的条目被当成图片」（is_video 识别 + 应用内播放） |
| 4 | 修复「视频模块点击播放视频无反应」（下载白名单加入 gtimg.com / myqcloud.com） |
| 5 | 修复「部分视频获取不到地址」（服务下线视频回退封面并提示） |
| 6 | 数据目录便携化：所有数据（库 / 会话 / 媒体）保存在软件目录 |
| 7 | 取消「媒体缓存」文件夹：按类型自动归入 images / videos |
| 8 | 缓存媒体按上传时间命名（年-月-日 时:分，短哈希防重名） |
| 9 | 修复「2018-2019 年相册老视频播放地址失效」（顺序映射解析直链，实测 42 条全通） |
| 10 | 新增视频播放地址过期自动刷新（vkey 403 自动重同步后继续播放） |
| 11 | 数据按 QQ 账号分目录（qq_QQ号，数据库按账号分开，旧数据自动迁移） |
| 12 | 修复「年份 / 排序下拉框浮层跑到左侧」（全部 Select append-to="self"） |
| 13 | 说说归档图片按上传时间命名 |
| 14 | 说说归档图片缓存原图（原图优先排序 + 原图地址哈希命名） |
| 15 | 说说归档图片真正原图（psc `/o` 变体恒返原图）+ 相册 / 说说同图识别不重复下载 |
| 16 | 同图去重升级为「内容级合并」（media_dedup 表，token 不同也合并） |
| 17 | 版本号更新至 2.0.0 + 清理 public/runtime 冗余截图（体积减约 3.9MB） |
| 18 | 必须解压后才能运行（压缩包内运行检测，防止数据写入挂载路径） |

## 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Vite |
| UI 组件 | PrimeVue 4 |
| 状态管理 | Pinia |
| 后端数据库 | SQLite (rusqlite) |
| HTTP 客户端 | reqwest (rustls-tls) |
| 打包 | NSIS / DMG / AppImage / deb / rpm / APK / unsigned IPA / Nix closure |

## 开发

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install) 1.77+
- [Node.js](https://nodejs.org/) 20+
- Windows: [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)（Windows 10+ 自带）
- Android: [Android Studio](https://developer.android.com/studio) + Android SDK + NDK

### 启动开发环境

```bash
# 安装前端依赖
npm install

# 启动开发服务器（桌面端）
npm run tauri dev

# Android 构建
npm run tauri android dev
```

### 构建

```bash
# Windows NSIS 安装包
npm run tauri:build:windows

# Windows NSIS + MSI
npm run tauri:build:windows:all

# Windows 便携版（单 EXE）
npm run tauri build --no-bundle
#（将 target/release/QQ 空间恢复归档.exe 与资源一并放入便携版目录）

# Android APK
npm run tauri android build
```

### 项目结构

```
├── src/                    # Vue 前端
│   ├── views/              # 页面组件
│   │   ├── DashboardView   # 概览（统计 + 互动排行）
│   │   ├── ArchivesView    # 归档内容（分类浏览、搜索、导出）
│   │   ├── MediaView       # 媒体时光轴
│   │   ├── LibraryView     # 相册（应用内播放 / 查看大图）
│   │   ├── TasksView       # 归档任务
│   │   └── SettingsView    # 设置
│   ├── components/         # 通用组件（含 ImageViewer.vue 图片查看器）
│   ├── stores/             # Pinia 状态管理
│   ├── utils/              # 工具函数与类型
│   └── layouts/            # 布局组件
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── main.rs         # 入口
│       ├── lib.rs          # Tauri 命令注册
│       ├── qlogin.rs       # QQ 登录（二维码 + 网页，含账号分目录）
│       ├── qzone.rs        # QQ 空间接口
│       └── archive.rs      # 归档引擎 + 数据库（核心改动）
└── src-tauri/capabilities/ # Tauri 权限配置
```

## 原理

### 数据来源与完整度

归档会合并三类来源：QQ 空间当前可见说说接口、旧历史消息接口，以及移动端互动列表接口 (`mobile.qzone.qq.com/get_feeds`)。可见说说会使用接口返回的 `total` 逐页对账；历史残留采用顺序扫描并验证最后一次命中后的 6,000 个记录位置；其他动态与留言主要来自互动通知和旧历史卡片。多来源记录按说说 ID、用户 QQ 号和事件时间合并，搜索在整个 SQLite 归档中执行，不受当前分页限制。

旧历史卡片显示的是点赞或评论发生时间，不是说说发布时间。对于仍保留标准 QQ 说说 ID 的记录，程序会从 ID 中校验账号并解码原始发布时间；无法验证的记录才保留接口时间。本地现有样本已经验证到 2018 年，但是否存在 2017 年记录必须以该账号在本次深度扫描中实际返回的数据为准。

图片和视频会保留接口返回的多个清晰度候选地址，并过滤头像、点赞图标和空间装饰图。部分旧资源的原图或视频签名已经被 QQ 服务端清理或过期时，只能保存仍可访问的低清地址或封面。

界面中的最早年份只代表本次请求在已验证范围内最早命中的记录，不能据此断言账号在更早年份没有发布内容。**没有被点赞或评论过、已被服务端彻底清除、超出接口保留范围或接口当前拒绝返回的动态无法恢复**。其他动态与留言也没有权威的服务端总数，因此只能报告本次接口返回量，不能保证穷尽。

### 登录方式

- **二维码登录**：调用 QQ 空间移动端扫码登录流程，全程不接触密码
- **网页登录**（桌面端）：打开独立窗口加载 QQ 登录页，通过 WebView Cookie API 提取登录凭证

登录凭证（Cookie）不会写入 SQLite、浏览器本地存储或日志。桌面端在用户登录成功后将必要会话加密保存到操作系统安全凭据库，用于下次启动恢复登录；退出登录或「删除所有数据」会清除该凭据。QQ 会话自身过期后仍需重新登录。

## 注意事项

- 请只归档本人或已获得授权的账号内容
- 归档过程中不要切换 QQ 客户端账号，否则可能有冻结风险
- 出现频繁提示时建议换个时间段继续，程序支持断点续传
- QQ 的视频签名有时效性，过期后需要重新归档以更新视频地址
- 数据默认保存在软件目录下，随程序整体迁移；建议定期将重要资料额外备份

## 已知限制

- 从未被点赞或评论过的说说无法恢复（互动列表中没有记录）
- QQ 视频地址带时效签名，过期后需重新同步刷新地址
- 部分 2019 年 QQ 小视频（v.qqstory.qq.com）因腾讯侧服务下线、域名无法解析，仅可查看封面
- 相册老视频依赖「视频」模块的同步数据解析播放直链：请先完成「视频」模块同步再播放相册视频

## 免责声明

本软件是用于整理和备份个人 QQ 空间资料的本地工具，与腾讯公司、QQ、QQ 空间及其关联主体不存在隶属、授权、合作关系。使用者应在合法授权范围内使用，并自行承担使用风险。详见应用内《免责声明与使用须知》。

## 许可证

本项目采用 [GNU GPLv3](LICENSE)（GNU General Public License v3.0）开源许可证：

- 您可以自由使用、修改与分发本软件，但任何修改或衍生作品必须以相同许可证（GPLv3）开源发布并提供源代码
- 本软件按“现状”提供，不附带任何明示或默示的担保
- 上游原作者：[xiaosu19/QQ-Zone-Restore-Archive](https://github.com/xiaosu19/QQ-Zone-Restore-Archive)（GPLv3）
- 上游主仓库：[Gaoshu705/QzoneArchive](https://github.com/Gaoshu705/QzoneArchive)（GPLv3）

## 友情链接

* [LINUX DO](https://linux.do/) - 新的理想型社区
