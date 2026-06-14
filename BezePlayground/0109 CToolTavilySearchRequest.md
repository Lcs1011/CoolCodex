
退出的时候清理缓存
位于 CoolSystemDir\tavily.toml 中设置 token

示范：(里面的 token 是瞎写的)

``` toml
  enabled = true  # 表示是否允许使用

  [[tokens]]
  name = "main"
  api_key = "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"
  enabled = true

  [[tokens]]
  name = "backup_1"
  api_key = "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"
  enabled = true

  [[tokens]]
  name = "backup_2"
  api_key = "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"
  enabled = false
```

#### KP
在 TOML 语法中，双括号 `[[表格名]]` 声明的是一个表数组
因而叫tokens而不是 token
toml文件需要用 # 注释而不是 //

明明倾向：代码类用下划线，设置类用横杠
toml 专门用来 config的类型。所以文件名没必要config



## 函数需要的参数

### action (必填)

告诉工具你要干嘛。只能填 "search", "extract", "zoom", "research", 或 "search_with_images"。

### query (条件必填)

你的搜索词。只有在做 search 或 research 时才传。

### url (条件必填)

目标网页的网址。只有在做 extract（提取正文）时才传。

### source_file (条件必填)

上一次搜索结果的本地文件路径。只有在做 extract 或 zoom 时需要传，用来追溯上下文来源。

### target (条件必填)

你要局部聚焦的章节名或文本。只有在做 zoom（聚焦放大）时才传。

### file_name_hint (可选)

给接下来要生成的 Markdown 文件随便起个名字线索，比如 "rust_cargo_docs"。

### 确认类字段 (按需填)

如果是高风险请求，根据系统提示补全 risk_confirmation、red_first_confirmation 或 red_second_confirmation。


#### 123

