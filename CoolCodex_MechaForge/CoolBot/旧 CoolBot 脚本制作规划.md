
## 文件结构


### 标准路径

启动器位置，该位置已被添加到全局变量
`C:\Arsenal\CoolAI\Launchers\cbot.bat` 

配置位置
`C:\Arsenal\CoolAI\Launchers\.cool-system\coolbot-config.toml`

写rust脚本的位置
`C:\Arsenal\CoolAI\CoolBot` 


```
C:\Arsenal\CoolAI\
│
├── Launchers\
│   ├── cbot.bat  启动器位置
│   └── .cool-system\
│       └── coolbot-config.toml  Coolbot设置位置
│
└── CoolBot\
    ├── .git\
    ├── .gitignore
    ├── Cargo.toml
    └── src\
        └── main.rs   Rust主程序入口
```

##### KP
之前写 Obsidian 插件的时候，核心文件叫 main.ts 。和main.rs一字之差
巧的是它们都位于 src 文件夹下。src 代表的是 source 的意思。


### 需要在 coolbot-config.toml 设置的内容
默认执行md 绝对路径
输出md 绝对路径



# 包含的各个功能

### 支持的CMD指令
通过将启动器命名为 cbot ，并将启动位置添加到系统环境变量，从而做到 支持CMD指令

和设置 相关的指令 其实都是对 coolbot-config.toml   的读写


cbot c
执行剪切板的内容 c 代表 clipboard

cbot 回车
提示继续按回车执行 剪切板的内容

cbot d
执行 设置的default 路径的md
如果没有设置efautl 路径 就提示找不到 无法执行

`cbot <执行文件的绝对路径径>`
执行绝对路径的md文件

cbot status 
显示当前状态 所有设置

cbot set 执行文件路径

cbot set 输出结论文件绝对路径



###
成功执行完毕之后会执行 
依次对所有push文件夹 push
git add .
git commit 注释就是 我们指令的 ID名字
git push
如果push成功  
则给 telegram 发信息 （如果设置了的话）
汇报push成功

如果不成功 中途 出现任何问题 无法完成也要给 发指令

###

如果执行失败了，中途执行失败。 那么也要执行 Git push。
然后，告诉执行到第几个命令失败了。
这样子做的好处是可以让云端的 AI 时刻看见当前的源代码是什么样子。


#### 执行相关
默认执行路径
报告输出路径

#### git 相关
需要git push的路径
工程路径
笔记路径 空就是有



### telegrame

从主账号 接受信息并执行



执行完给 主账号发信息

#### telegram设置

telegram token
主人 id


###

从接受的信息中提取出 执行分段 然后执行


###
审查指令是否合格


第一行必须是 OperationID <起的名字>否则的话，就不能执行
然后脚本会先完整检查一下执行内容 其中有不可识别的部分，就不执行


还有各种审查规矩 依次补充
###
