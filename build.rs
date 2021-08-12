fn main() {
    /*rte_lcore_id() 定义在dpdk/lib/librte_eal/include/rte_lcore.h
      rte_eal_init(argc, argv); 定义在dpdk/lib/librte_eal/linux/eal.c
	  rte_eal_remote_launch();  定义在dpdk/lib/librte_eal/linux/eal_thread.c
	  rte_eal_mp_wait_lcore;    定义在dpdk/lib/librte_eal/common/eal_common_launch.c
	  rte_eal_cleanup();        定义在dpdk/lib/librte_eal/linux/eal.c
	  rte_get_next_lcore        定义在dpdk/lib/librte_eal/common/eal_common_lcore.c*/
    cc::Build::new()
        .file("dpdk/lib/librte_eal/include/rte_lcore.h")
        .file("dpdk/lib/librte_eal/linux/eal.c")
        .file("dpdk/lib/librte_eal/linux/eal_thread.c")
        .file("dpdk/lib/librte_eal/common/eal_common_launch.c")
        .file("dpdk/lib/librte_eal/common/eal_common_lcore.c")
        .compile("DPDK");
        
    println!("cargo:rerun-if-changed=dpdk/lib/librte_eal/include/rte_lcore.h");
    println!("cargo:rerun-if-changed=dpdk/lib/librte_eal/linux/eal.c");
    println!("cargo:rerun-if-changed=dpdk/lib/librte_eal/linux/eal_thread.c");
    println!("cargo:rerun-if-changed=dpdk/lib/librte_eal/common/eal_common_launch.c");
    println!("cargo:rerun-if-changed=dpdk/lib/librte_eal/common/eal_common_lcore.c");
}