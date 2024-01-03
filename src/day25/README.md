https://www.reddit.com/r/adventofcode/comments/18qcer7/2023_day_25_graphviz_rescues_once_again/

# 安装 graphviz

1. 首先安装 MacPorts: https://www.macports.org/install.php

2. https://graphviz.org/download/

```console
sudo port install graphviz
```

# GraphViz Examples

[Examples](https://graphs.grevian.org/example)


# 生成 svg

```console
# 弹簧系统 Spring system
dot -K neato -T svg input.dot > output.neato.svg
```

or 

```console
# 力导向图 Scalable Force-directed graph drawing
dot -K sfdp -T svg input.dot > output.sfdp.svg
```

