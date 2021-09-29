# Summer2021-No.127 用Rust实现DPDK封装

#### Description
https://gitee.com/openeuler-competition/summer-2021/issues/I3R27U

#### Software Architecture
This project is a Rust package library for DPDK 20.11.By encapsulating the DPDK library written in C language, external Rust applications can directly use the functions in the DPDK library through the Rust API.This project currently only encapsulates about 200 commonly used APIs of DPDK, and the functions are still very imperfect. Users can add the DPDK functions they need according to the method of this project.

#### Installation

Configure DPDK environment

```bash
cd dpdk
sudo meson build
cd build
sudo ninja install
```
Set DPDK hugepages (runtime)

```bash
sudo chmod 777 /sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages
echo 1024 > /sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages
```
Load the linux driver, see in detail:
http://doc.dpdk.org/guides/linux_gsg/linux_drivers.html。

Bind the network port

```bash
cd dpdk/usertools
//check the network card net_cd
ifconfig
sudo dpdk-devbind.py --bind=net_cd
//check the bindings
sudo dpdk-devbind.py --status
```


#### Instructions

sudo cargo run

#### Contribution

1.  Fork the repository
2.  Create Feat_xxx branch
3.  Commit your code
4.  Create Pull Request

in details：https://github.com/cxxuser/rust_dpdk

