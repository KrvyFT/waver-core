# waver-core

Graph IR、静态模块目录（`ModuleDesc` / `MODULE_CATALOG`）、拓扑编译、无锁 `ParamCell`，以及 GUI → 音频的 `RtCommand`。

本仓库是 [waver](https://github.com/KrvyFT/waver) workspace 的一部分，在伞仓中位于 `crates/waver-core`（git submodule）。

## 仓库

- GitHub：https://github.com/KrvyFT/waver-core
- 默认分支：`main`
- License：MIT OR Apache-2.0

## 在伞仓里开发（推荐）

```bash
git clone --recurse-submodules https://github.com/KrvyFT/waver.git
cd waver/crates/waver-core
# 改代码后：
git add -A && git commit -m "…" && git push
cd ../..
./scripts/repos.sh sync
git commit -m "chore: bump waver-core" && git push
```

`Cargo.toml` 使用 workspace 继承；请在伞仓根目录执行 `cargo test -p waver-core` / `cargo build`。

## 单独 clone

```bash
git clone https://github.com/KrvyFT/waver-core.git
```

可独立查看历史与提 PR；**构建仍需伞仓**（path / workspace 依赖）。约定见 [doc/repos.md](https://github.com/KrvyFT/waver/blob/master/doc/repos.md)。

## 内容概要

| 模块 | 说明 |
|------|------|
| `graph` / `compile` / `schedule` | 可编辑图 → 拓扑序（反馈时插 `Delay`） |
| `module` | `ModuleFamily`（类型）、`ModuleDesc` / `MODULE_CATALOG`、参数默认值与文案 |
| `patch` / `param` | `CompiledPatch`、`ParamRegistry`、`ParamCell` |
| `command` / `status` | `RtCommand`、`EngineStatus` |

扩展模块步骤见伞仓 [doc/modules.md](https://github.com/KrvyFT/waver/blob/master/doc/modules.md)。
