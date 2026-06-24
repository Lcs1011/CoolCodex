## 配置文件概述

bat 中 设置

CoolSystem级 配置位于： 该路径简称为 CoolSystemDir

```text
LauncherDir\.cool-system\
```

CoolCharacter 配置位于： 该路径简称为 CoolDir

```text
CoolCharacterRoot\.cool\
```

###
CoolSystemDir 和 CoolDir
各有
config.toml  //默认设置 暂时为空
scope.toml // 视野设置
command.toml //命令设置

## Launcher bat中一共要设置 8样东西
1/ CoolCharacterRoot 路径
2/ CoolWorkspace 是否跟随 启动窗口
3/ CoolWorkspace 默认路径
4/ CToolScopeBase 枚举 默认值
5/ CToolCommandPolicy 枚举 默认值

6/ CoolCLIExePath 设置
7/ SafeMode 开关
8/ PermissionProfile枚举 默认值


## Launcher bat 开头部分

```bat
@echo off
setlocal

rem ============================================================
rem CTool Settings
rem ============================================================

rem Enter path
set "COOL_CHARACTER_ROOT=C:\Arsenal\CoolAI\CoolBot"

rem on/off
set "COOL_WORKSPACE_FOLLOW_LAUNCH_DIR=off"

rem Enter path
rem Relative path starts from COOL_CHARACTER_ROOT
set "COOL_WORKSPACE=C:\Arsenal\CoolAI\CoolBot"

rem none/selected-only/cool-workspace/the-eye-of-providence
set "CTOOL_SCOPE_BASE=cool-workspace"

rem green/yellow/red/block/block-all
set "CTOOL_COMMAND_POLICY=yellow"


rem ============================================================
rem CoolCLI Settings
rem Recommended: do not change.
rem ============================================================

rem Enter exe absolute path 
set "COOL_CLI_EXE_PATH=C:\Arsenal\CoolAI\CoolCodex\codex.exe"

rem on/off
set "COOL_SAFE_MODE=on"

rem cool-read-write/read-only/workspace-write/full-access
set "COOL_PERMISSION_PROFILE=cool-read-write"

rem ============================================================
rem Internal Launch Settings
rem Do not edit.
rem ============================================================

rem Launch directory
set "LAUNCH_DIR=%CD%"

rem Launcher directory
set "LAUNCHER_DIR=%~dp0"

rem Codex executable path
set "CODEX_EXE=C:\Arsenal\CoolCurrent\codex.exe"

rem Cool system directory
set "COOL_SYSTEM_DIR=%LAUNCHER_DIR%.cool-system"

```

注：
SAFE_MODE和 PERMISSION_PROFILE 太通用 前面加 COOL_为了防止在CMD中意外撞名

## 启动流程
只允许通过bat启动 Codex.exe ，不允许双击启动。双击无法启动

#### 1. 先展示一下设定，然后提示 y确认继续启动
如果SafeMode 不是on 
或者 PermissionProfile枚举不是 cool开头 
或者 CToolCommandPolicy 是 Green
则需要确认两次，第二次要求输入 launch

1/ CoolCharacterRoot 路径
2/ CoolWorkSpace 是否跟随 启动窗口
3/ CoolWorkSpace 默认路径
4/ CToolScopeBase 枚举 默认值
5/ CToolCommandPolicy 枚举 默认值

6/ SafeMode 开关
7/ PermissionProfile枚举 默认值

#### 2. 启动CLI在做任何事情之前
第一时间 初始化SafeMode 
SafeMode必须只读

#### 3. 然后在做其他任何事情之前，包括启动之前
展示Scope.toml中的详细设置
系统级和项目级都展示
（command相关设置先不展示）
然后提示是否继续，y继续

#### 4. 之后就进入正常流程




## Scope 设置介绍


### CToolScopeBase 枚举
#### none 
无视野，纯粹无任何视野 toml文件也无法添加视野。  启动时，找不到CToolScopeBase设定，则将其设置为 none
#### selected-only 
无基础视野仅凭toml设定
#### cool-workspace  
CoolWorkspace文件夹为基础视野
#### the-eye-of-providence 
全局视野//暂不开放，不制作无视和 cool-workspace 相同处理


 话说有两点要交代  00014_yellow_command_request.md 这类信息是给你看的。但是现在位置不对 要从.cool移动到 CoolHunterHall
  这个文件夹。 你要改一下。 另外  request_log.md  这不是给ai看的
 要放在cool里 不变，但是ai根本不应该知道发生过这个事情。
 
 
 按照前三个字母判断
 第一个字母y就算通过 
 n就算不通过
 后面可一个接上对ai的话而不仅仅光发一个字母
 yyy就代表相同命令以后不用再说了
commandrequest允许提前申请


### 原则与权重

要满足下面三个情况
1/ 系统封杀的文件夹 角色的文件许可也绝对不能开开
2/ 系统开放的 角色要能封掉 （解决办法就是系统 尽量不要默认别开任何视野，否则角色级只能动用特权封）
3/ 同一个级别的文件 开放 文件夹关闭 要让该文件夹可访问


原则等级排序
1/ 特权 大于 非特权    //轻易不要填写特权，特权要留着关键时候用
2/ 系统 大于 角色  //系统尽量不要给 ReadWrite 或 ReadOnly
3/ 文件 大于 文件夹
4/ 禁止 大于 开放
5/ 都大于  CToolScopeBase  //None除外



```
设置冲突解决原则：

CToolScopeBase:none >

Privileged 内部如下不赘述 整体 > 

System.Files.Hidden > System.Files.ReadOnly > System.Files.ReadWrite >
System.Folders.Hidden > System.Folders.ReadOnly > System.Folders.ReadWrite >
Character.Files.Hidden > Character.Files.ReadOnly > Character.Files.ReadWrite >
Character.Folders.Hidden > Character.Folders.ReadOnly > Character.Folders.ReadWrite >
CToolScopeBase:非none
```


### Scope.toml 空表演示

Scope.toml 运行中重新设置无效

``` toml
[privileged_files]

readwrite = []
readonly = []
hidden = []


[privileged_folders]

readwrite = []
readonly = []
hidden = []


[files]

readwrite = []
readonly = []
hidden = []


[folders]

readwrite = []
readonly = []
hidden = []

```

### Scope.toml 系统级推荐
``` toml
[privileged_files]

readwrite = []
readonly = []
hidden = []


[privileged_folders]

readwrite = []
readonly = []
hidden = []


[files]

readwrite = []
readonly = []
hidden = []


[folders]

readwrite = []
readonly = []
hidden = [
	"LauncherDir/.cool-system",  
	"./.cool",
	"./.git",
]

```


### Scope.toml 角色级推荐
默认空表即可


## Command 设置介绍

CToolCommandPolicy 枚举 
包括 Green/Yellow/Red/Blocked/BlockAll
表示对待 Command 的处理方式
Green/Yellow/Red/Blocked/ 皆为如果在 command.toml找不到该指令的设置时的默认对待方式 其实就相当于省略了 Treat No Match As...
BlockAll表示 阻挡任何 CommandRequest

注：
如果启动时 找不到 CToolCommandPolicy的设置，则将 CToolCommandPolicy设置为 BlockAll

注：
Command 本身还有级别枚举
注：
Command 本身还有级别枚举
CommandRequestLevel 包括 Green/Yellow/Red/Blocked/

### 权重排序后面的操作我们就不管，launcher 文件夹下的内容了，我们就先单纯只修改源代码，明白吗？那么第一步就是我们先修改启动相关的源代码，还有枚举变量相关的，开始了。然后你要记得一件事情。
特权 大于 非特权  //轻易不要填写特权，特权要留着关键时候用
禁止 大于 开放
都大于 默认 //blockall除外

做法是 先合并 相同性质的集合，然后再排序。

```
CToolCommandPolicy:BlockAll > 

System.Privileged.Block + Character.Privileged.Block >
System.Privileged.Red + Character.Privileged.Red >
System.Privileged.Yellow + Character.Privileged.Yellow >
System.Privileged.Green + Character.Privileged.Green >

System.Normal.Block + Character.Normal.Block >
System.Normal.Red + Character.Normal.Red >
System.Normal.Yellow + Character.Normal.Yellow >
System.Normal.Green + Character.Normal.Green >

CToolCommandPolicy 不为BlockAll
```

### command.toml 空表演示

``` toml
[ctool_command_privileged]

blocked_prefixes = []
blocked_contains = []

red_prefixes = []
red_contains = []

yellow_prefixes = []

green_exact_commands = []
green_prefixes = []


[ctool_command]

enabled = true

blocked_prefixes = []
blocked_contains = []

red_prefixes = []
red_contains = []

yellow_prefixes = []

green_exact_commands = []
green_prefixes = []
```

### 系统级默认





### 角色级默认空

