
位于单独环境

推送结束

永远不许在windows中 创建python环境
永远不允许破坏 conda的 base环境


执行cmd指令的时候 必须位于 conda的 coolmechaforge环境里执行
不要位于 base

任何指令结束之后 必然推送
MechaForge文件夹
MechaForge文件夹是一个 支持obsidian的 文件夹
CoolMechaForge 文件夹位于 
C:\Arsenal\CoolAI\Launchers\.cool-system中的
coolbot-config.toml 设置
例如：

CoolMechaForge = 'C:\Arsenal\CoolAI\CoolMechaForge'

执行 Cool operation的时候，CoolBot会在
MechaForge文件夹 中的Logs
根据Workspace路径名字生成的文件夹

例如`C:\Arsenal\CoolAI\CoolBot`
生成C_Arsenal_CoolAI_CoolBot

其中存放 以本次指令命名的 详细md文档
还有其他必要信息

之后要记得定时清理

conda create -n coolmechaforge python=3.12 -y
conda activate coolmechaforge
python --version
pip --version