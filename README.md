# Summer2021-No.127 用Rust实现DPDK封装

#### 介绍
https://gitee.com/openeuler-competition/summer-2021/issues/I3R27U

#### 软件架构
dpdk文件下装的是dpdk20.11版本的dpdk库，本项目是基于这个版本的dpdk库进行封装的。
src文件下是测试代码和Rust safe API封装代码


#### 安装教程及使用说明
要使用本库，首先要配置DPDK环境

```bash
cd dpdk
sudo meson build
cd build
sudo ninja install
```
设置DPDK大页(运行时)

```bash
sudo chmod 777 /sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages
echo 1024 > /sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages
```
加载linux driver，详细可见：http://doc.dpdk.org/guides/linux_gsg/linux_drivers.html。

绑定网络端口

```bash
cd dpdk/usertools
//check the network card net_cd
ifconfig
sudo dpdk-devbind.py --bind=net_cd
//check the bindings
sudo dpdk-devbind.py --status
```

sudo cargo run即可运行封装测试程序（注：必须要在sudo模式下运行，不然无法访问内存大页）

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 详细参看
https://gitlab.summer-ospp.ac.cn/summer2021/210010816
