# Rust 开发 WebAssembly 指南

## 前言

该项目是一个 wasm-pack + Rust + WebAssembly 的开发模板，用于使用Rust开发WebAssembly的包，该项目以开发一个wasm版本的 md5 算法为例，我们捋一下使用Rust开发WebAssembly包的整个流程。


> 温馨提示：md5算法 并不安全，不推荐使用。

## 项目结构

```
packages/md5/
├── .cargo/              # Rust cargo配置目录
│   └── config.toml      # cargo配置文件
├── src/                 # 源代码目录
│   └── lib.rs          # Rust实现代码
├── pkg/                # 构建产物目录
│   └── node/         # nodejs构建产物
│   └── bundler/        # bundler构建产物
│   └── web/            # web构建产物
│   └── no-modules/     # no-modules构建产物
├── tests/              # 测试目录
├── scripts/            # 脚本目录
├── .gitignore          # Git忽略文件
├── Cargo.toml          # Rust项目配置
├── rust-toolchain.toml  # Rust编译工具链配置
├── package.json        # npm包配置
└── README.md           # 包说明文档
└── README_GUIDE.md     # 项目说明文档
```

## 开发环境准备

1. 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装 wasm-pack

```bash
cargo install wasm-pack
# 或
npm install -g wasm-pack
```

wasm-pack 说明文档：

<https://rustwasm.github.io/wasm-pack/book/quickstart.html>

3. 安装 Node.js (如果还没安装)

## 项目初始化

1. 创建项目目录
```bash
mkdir md5-wasm
cd md5-wasm
```

2. 初始化 Rust 项目
```bash
cargo init --lib
```

或者使用 wasm-pack 初始化项目

```bash
wasm-pack new md5-wasm

```

3. 配置 Cargo.toml

该部分根据需要改动

```toml
# Cargo.toml
[package]
name = "md5"
version = "0.1.0"
edition = "2021"
description = "MD5 WebAssembly implementation"
keywords = ["md5", "md5-wasm"]
license = "MIT"
authors = ["Pursue-LLL <yululiu2018@gmail.com>"]
repository = "git+https://github.com/Pursue-LLL/md5-wasm.git"

[lib]
# C ABI： 定义了程序在二进制层面进行交互的规则，包括数据类型表示、函数调用约定等。只要不同的编程语言生成的代码都符合 C ABI，它们就可以互相调用，而无需关心彼此使用的具体编程语言或编译器。
# wasm-bindgen 通过cabi的方式去调用rust按照cabi的规则编译好的二进制.wasm 文件，然后生成js胶水代码以在js中使用
crate-type = ["cdylib"]

[dependencies]
# wasm-bindgen 是 WebAssembly 绑定工具，用于将 Rust 代码编译为 WebAssembly 并生成 JavaScript 胶水代码。
wasm-bindgen = "0.2"
# serde 是序列化和反序列化框架，可以轻松地将 Rust 数据结构转换为各种格式（如 JSON、YAML、TOML 等）。features = ["derive"] 启用了 serde 的派生宏，可以通过简单的注解自动为你的 Rust 结构体和枚举实现序列化和反序列化功能。
serde = { version = "1.0", features = ["derive"] }
# serde-wasm-bindgen库提供了 serde 和 wasm-bindgen 之间的集成。可以使用 serde 来序列化和反序列化 Rust 数据结构，并将其与 JavaScript 的对象进行相互转换。
serde-wasm-bindgen = "0.6.5"
# js全局对象，如window、document等
js-sys = "0.3"

[profile.release]
# 进行最高级别的优化。编译器会尽可能地优化代码，以生成运行速度最快的二进制文件。这可能会显著增加编译时间，并可能稍微增加二进制文件的大小。通常用于 release 构建
opt-level = 3
# true 表示启用链接时优化。LTO 是一种编译器优化技术，它允许编译器在链接阶段对整个程序进行优化，而不仅仅是单个编译单元（例如单个 .rs 文件）。
# LTO 通常可以生成更小、更快的二进制文件，但会增加编译时间。
lto = true

# 添加 wasm-pack 的配置
[package.metadata.wasm-pack.profile.release]
# 打包优化配置，无需改动，-O4 表示最高级别优化，--enable-bulk-memory 表示启用大内存优化
wasm-opt = ["-O4", "--enable-bulk-memory"]

# 条件编译配置，MD5不使用web api，该包不依赖DOM，所以Node和Browser通用
[features]
default = ["node"]
browser = []
node = []

# 配置编译器lint规则
[lints.rust]
# 忽略wasm_bindgen_unstable_test_coverage的警告，因为该条件不在标准条件列表中，是一个自定义的编译条件
unexpected_cfgs = { level = "allow", check-cfg = ['cfg(wasm_bindgen_unstable_test_coverage)'] }
unsafe_code = "forbid"        # 禁止不安全代码
unused = "warn"               # 未使用的代码警告
missing_docs = "deny"         # 缺少文档就报错

#配置Clippy 的规则
[lints.clippy]
enum_glob_use = "deny"        # 禁止使用 enum 的 glob import
type_complexity = "warn"      # 类型过于复杂时警告

# 配置开发依赖
[dev-dependencies]
# wasm-bindgen-test 是 wasm-bindgen 的测试库
wasm-bindgen-test = "0.3"

```

> 如果你的包比较复杂，并且要在全平台中通用且不同平台有不同的实现，就需要用到条件编译了，本篇主讲开发模板，这里不再赘述。

1. 创建 .cargo/config.toml

该部分无需改动，所有项目都可通用

```toml
[build]
# 指定编译目标
target = "wasm32-unknown-unknown"

```

**target = "wasm32-unknown-unknown"**

作用：

target = "wasm32-unknown-unknown" 是 Cargo（Rust 的构建工具）的配置选项，用于指定编译的目标平台为 WebAssembly。

通常在 .cargo/config.toml 文件（全局配置）或项目根目录下的 .cargo/config.toml 文件（项目配置）中设置。也可以通过命令行参数 --target wasm32-unknown-unknown 传递给 cargo build 命令。

含义：

wasm32：表示 32 位的 WebAssembly 目标架构。
unknown-unknown：表示目标操作系统和环境是未知的，因为 WebAssembly 运行在各种环境中（浏览器、Node.js 等）。


1. 初始化 npm 项目
```bash
npm init
```

修改 package.json

```json
{
  "name": "md5-wasm",
  "version": "1.0.0",
  "description": "MD5 implemented with WebAssembly",
  "type": "module",
  "scripts": {
    // 打包脚本，自动打包多目标
    "build": "node scripts/build.js",
    // 使用html测试Web 和 no-modules 打包产物
    "test:html": "http-server . -p 8080 -o /tests/index.html",
    // 测试node打包产物
    "test:node": "ava"
  },
  "devDependencies": {
    "ava": "^5.3.1",
    "md5": "^2.3.0",
    "http-server": "^14.1.1"
  },
  "license": "MIT",
  "ava": {
    "files": [
      "tests/**/*test.cjs"
    ],
    "timeout": "2m"
  }
}
```

6. scripts/build.js

该脚本用于构建多Target的wasm包，一般情况下我们都会生成以下几种Target的包：

- web
- no-modules
- nodejs
- bundler


以满足不同场景下的使用需求。

> 具体支持构建的目标可参照<https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#nodejs>
>

> 注意，target 只影响代码构建的模块化方案，不影响代码本身是否支持node或浏览器，影响代码是否支持多平台的是“条件编译”，这个我们之后说。

7. rust-toolchain.toml

配置编译工具链，用于统一编译和开发环境，避免 "在我机器上是好的" 问题

8. README.md

根目录下的README.md，用于描述被打包产物的信息，会被放到pkg打包产物中

README_GUIDE.md 为模板项目的说明文档

> 完整项目请参照<https://github.com/Pursue-LLL/md5-wasm>，然后根据需要修改

## 开发步骤

### 实现 Rust 代码

在 src/lib.rs 中写你的 Rust 代码


### 构建 WebAssembly

构建多Target的wasm包

```bash
# 该命令会执行自定义构建脚本
npm run build
```

如果你只编写特定平台的代码，可以构建单目标的包

```bash
wasm-pack build --out-dir pkg/nodejs --target nodejs
```

### 测试

**测试构建产物**

编写测试用例

- 如果产物 target 为nodejs，在tests目录下编写 .cjs 文件的测试用例

使用以下命令测试

```bash
npm run test:node
```

- 如果产物 target 为web或no-modules，在tests目录下编写 .html 文件的测试用例

代码如下，具体可查看项目示例

**es 模块**

```html
<script type="module">
  import init, { md5 } from '../pkg/web/md5_wasm.js';
  init().then(() => {
    console.log(md5('hello')); // 输出: 5d41402abc4b2a76b9719d911017c592
  });
</script>
```

**非模块**

考虑不兼容esm的浏览器，使用非模块

wasm_bindgen 为wasm-bindgen库的初始化函数，在no-modules模式下，需要手动初始化wasm

```html
<script src="../pkg/no-modules/md5_wasm.js"></script>
<script>
  (async () => {
      // 初始化wasm
      await wasm_bindgen();

      // 获取md5函数
      const { md5 } = wasm_bindgen;
      console.log(md5('hello')); // 输出: 5d41402abc4b2a76b9719d911017c592
  })();
</script>
```

使用以下命令测试

```bash
npm run test:html
```

- 如果产物 target 为bundler，在rollup、webpack、vite等构建工具中进行测试

**测试rust代码**

参照：<https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html>

在 src/tests.rs 中编写测试用例，然后使用以下命令测试

```bash
wasm-pack test --chrome
wasm-pack test --firefox

wasm-pack test --node
```

## 发布流程

### 发布单Target的包

使用以下命令构建目标平台的包后，将pkg目录下的包发布到npm

> 注意，发布单Target的包需要在构建前编辑Cargo.toml中的version、description、repository、keywords、author等字段，构建后会自动更新到pkg目录下对应产物的package.json中

```bash
wasm-pack build --out-dir pkg/node --target node --scope gogors

cd pkg/node

npm publish
```
> --scope gogors 是发布到npm的scope

### 发布多Target的包

如果你使用 `npm run build` 命令，会自动构建多Target的包，包括：

- web
- no-modules
- nodejs
- bundler

其中 **bundler** 是 esm 模块的包，**nodejs** 是 cjs 模块的包，**web** 和 **no-modules** 是 IIFE 模块的包

1. 构建项目

```bash
npm run build
```

2. 发布到支持多目标的包

```bash
npm publish
```

该命令直接发布根目录的package.json，自行编辑package.json中的version、description、repository、keywords、author等字段即可

使用 `npm publish` 后，会发布支持esm、cjs，以及在html中可直接使用的IIFE的包，如：

```html
<script type="module" src="https://unpkg.com/@gogors/md5-wasm/pkg/web/md5_wasm.js"></script>


<script src="https://unpkg.com/@gogors/md5-wasm/pkg/no-modules/md5_wasm.js"></script>
```

当然你也可以将资源发布到自己的cdn地址。


## 使用示例


### 在构建工具中使用

**在 vite 中使用，需要使用 vite-plugin-wasm 插件**

> 参照：https://github.com/Menci/vite-plugin-wasm

```bash
npm install vite-plugin-wasm
```

```typescript
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ]
});
```

**在webpack中使用，需要使用 @wasm-tool/wasm-pack-plugin 插件**

> 参照：https://github.com/wasm-tool/wasm-pack-plugin#readme

```bash
npm install @wasm-tool/wasm-pack-plugin
```

```typescript
import wasmPackPlugin from '@wasm-tool/wasm-pack-plugin';

export default defineConfig({
  plugins: [wasmPackPlugin()],
});
```


**在rollup中使用，需要使用 rollup-plugin-esmwasm 插件**

> 参照：https://github.com/Pursue-LLL/rollup-plugin-esmwasm

```bash
npm install rollup-plugin-wasm
```

```typescript
import wasm from 'rollup-plugin-esmwasm';

export default {
  plugins: [wasm()],
};
```

在构建工具中使用时，引入 bundler 模式的包，可直接导入使用，无需手动初始化

```js
// 可直接导入使用，无需手动初始化
import { md5 } from '@gogors/md5-wasm';

console.log(md5('hello')); // 输出: 5d41402abc4b2a76b9719d911017c592
```

### 在nodejs中使用

```javascript
const { md5 } = require('@gogors/md5-wasm');

console.log(md5('hello')); // 输出: 5d41402abc4b2a76b9719d911017c592
```

### 在浏览器中使用

导入后需要先初始化再使用

**es 模块**

```html
<script type="module">
  import init, { md5 } from 'https://unpkg.com/@gogors/md5-wasm/pkg/web/md5_wasm.js';
  init().then(() => {
    console.log(md5('hello')); // 输出: 5d41402abc4b2a76b9719d911017c592
  });
</script>
```

**非模块**

考虑不兼容esm的浏览器，使用非模块

wasm_bindgen 为wasm-bindgen库的初始化函数，在no-modules模式下，需要手动初始化wasm

```html
<script src="https://unpkg.com/@gogors/md5-wasm/pkg/no-modules/md5_wasm.js"></script>
<script>
  (async () => {
      // 初始化wasm
      await wasm_bindgen();

      // 获取md5函数
      const { md5 } = wasm_bindgen;
      console.log(md5('hello')); // 输出: 5d41402abc4b2a76b9719d911017c592
  })();
</script>
```

> unpkg 的使用参照 <https://unpkg.com/>

## 总结

通过以上步骤，我们完成了使用Rust开发WebAssembly包的整个流程，有了该模板之后你可以快速开发WebAssembly包，大幅提高你的应用性能。

git 仓库地址：<https://github.com/Pursue-LLL/md5-wasm>