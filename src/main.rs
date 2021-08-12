mod rust_bindings;
use crate::rust_bindings::*;
use std::env;
use std::convert::TryInto;
fn lcore_hello(input:i32)->i32{
	let lcore_id = unsafe {rte_lcore_id()};
	println!("hello from core {}", lcore_id);
	return 0;
}
fn main() {
    
	//rte_eal_init 
	//argv为一个字符串集合和C语言中对应的char **argv含义相同
	let argv: Vec<String> = env::args().collect();
	//argc代表字符串集合的长度和C语言中对应的int argc含义相同
	let argc :u32= argv.len().try_into().unwrap();
	let ret = unsafe {rte_eal_init(argc, argv)};
    //failed to init
	if (ret < 0){
		println!("Cannot init EAL!");
    }
	/* call lcore_hello() on every worker lcore */
	/*for (lcore_id = unsafe {rte_get_next_lcore(-1, 1, 0)};
         lcore_id < RTE_MAX_LCORE;
         lcore_id = unsafe {rte_get_next_lcore(lcore_id, 1, 0)}){
		unsafe {rte_eal_remote_launch(lcore_hello, 1, lcore_id)};
	}*/
    //change to while
	let RTE_MAX_LCORE =128;
	type LcoreFunctionT = fn(i32)->i32;
	let func: LcoreFunctionT = lcore_hello;
    let mut lcore_id = unsafe{rte_get_next_lcore(-1,1,0)};
    while lcore_id < RTE_MAX_LCORE{
        
        unsafe {rte_eal_remote_launch(func, 1, lcore_id)};
		lcore_id = unsafe{rte_get_next_lcore(lcore_id,1,0)};
    }

	/* call it on main lcore too */
	lcore_hello(1);

	unsafe {
    	rte_eal_mp_wait_lcore();
	/* clean up the EAL */
		rte_eal_cleanup();
    }

}
