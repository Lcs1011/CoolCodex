



``` 
@echo off
setlocal

rem ============================================================
rem User Launch Settings
rem ============================================================

rem CharacterRoot:
rem Codex process will finally cd into this folder before launch.
set "COOL_CHARACTER_ROOT=C:\Arsenal\CoolAI\CoolBot"

rem Workspace follow mode:
rem 1 = CoolWorkspace follows the directory where this BAT was launched from.
rem 0 = CoolWorkspace uses COOL_WORKSPACE below.
set "COOL_WORKSPACE_FOLLOW_LAUNCH_DIR=0"

rem Fixed CoolWorkspace:
rem Used only when COOL_WORKSPACE_FOLLOW_LAUNCH_DIR=0.
set "COOL_WORKSPACE=C:\Arsenal\CoolAI\CoolBot"

rem ============================================================
rem Core Safety Settings
rem Recommended: do not change these two.
rem ============================================================

rem SafeMode:
rem Recommended value: on.
set "COOL_SAFE_MODE=on"

rem PermissionProfile:
rem Recommended value: CoolReadWrite.
set "COOL_PERMISSION_PROFILE=CoolReadWrite"

rem ============================================================
rem Internal Launch Paths
rem Do not edit these unless you are changing launcher structure.
rem ============================================================

rem Directory where this BAT was launched from.
set "LAUNCH_DIR=%CD%"

rem Directory where this BAT file itself is located.
set "LAUNCHER_DIR=%~dp0"
```



## 设计哲学

1/ 限制CLI的设置 必须位于bat

bat 中  限制CLI的设置 无法在启动后修改

2/ 限制 CTool的设置 部分为了继承方便，也位于bat中设置

bat 中 限制 CTool的设置 启动后可修改，但无法保存 （技术上可以保存，暂时设置无法保存降低风险）

对  CTool 大量的细节设置 位于 toml
toml 中 对 CTool的设置  启动后可修改 且可保存

所有的运行时设置
只能设置
character级别 toml
不能设置系统级


### CommandRequest 相关设置
应该所有的设置都尽可能的位于。System
character级别只做补充



CToolScope 枚举

CToolScope
### 一些补充原则：

位于CLI启动后
CTool的启动 


任何时候需要找 设置内容
无论是找不到设置文件（包括找不到bat 和 toml）
或者设置文件中找不到对应的设置
都按照最小权限 作为默认

时，无论是设置内容理应出现在bat中找不到，或者理应出现在toml中找不到，权限都应为最低

裸exe启动要么权限最低，要么直接拦截

cbot 命令 被视为红色 commandrequest

所有用`.` 表示的路径都是指 CharacterRoot
所有的相对路径都是以 CharacterRoot 作为根路径




设置
bat

CoolSystemDir

CoolDir



设置分层




byte 中的设置就是限制 CLI 的必须在 byte 里设置。
为了方便，其中的 Ctwo 设置也要放在 BAT，一目了然。
而非常细致详细的设置，则需要在 TOML 里。


## KP
bat中设置的变量可以存活到 cmd窗口结束


bat 不能读取 toml 文件中的设置
而且 限制CLI的设置 必须位于 bat中

bat 中设置的变量 被加载并不会马上影响 Rust程序里的值
影响Rust程序里面的值，必须要main 函数运行以后 初始化
因而限制CLI的 变量要在 main启动后 立刻初始化 ，早于做任何事情之前


那么之所以导航到 character root，然后启动 CLI，目的是为了，因为有一些我们改造过的 CLI，有可能有残留的改造没有进行完，那么一旦让它生成文件，让它生成到这个专属的自己的沙箱里，兜个底。