use libc::{c_void,c_int};

type LcoreFunctionT = fn(c_int)->c_int;
extern "C"{
	pub fn rte_lcore_id()->u32;
}
extern "C"{
	pub fn rte_eal_init(argc: u32,argv:Vec<String>)->c_int;
}
extern "C"{
	pub fn rte_get_next_lcore(i:i32,skip_main:i32,wrap:i32)->i32;
}
extern "C"{
    //in c unsigned = u32
    pub fn rte_eal_remote_launch(f:LcoreFunctionT,arg:c_int,worker_id:i32)->i32;
}
extern "C"{
	pub fn rte_eal_mp_wait_lcore()->();
}
extern "C"{
	pub fn rte_eal_cleanup()->c_int;
}
