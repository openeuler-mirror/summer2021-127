use libc::{c_int,c_void};
use std::convert::TryInto;
pub type LcoreFunctionT = fn(c_int)->c_int;
pub type FUNC = unsafe extern fn(*mut c_void,*mut cmdline,*mut c_void);
pub type FUNC1 = unsafe extern fn(*mut cmdline_token_hdr,*const u8,u32)->c_int;
pub type FUNC2 = unsafe extern fn(*mut cmdline_token_hdr)->c_int;
pub type FUNC3 = unsafe extern fn(*mut cmdline_token_hdr,c_int,*mut u8,u32)->c_int;
pub type FUNC4 = unsafe extern fn(*mut cmdline_token_hdr,*mut u8,u32)->c_int;
pub type F1 = unsafe extern fn(*mut rdline,u8)->c_int;
pub type F2 = unsafe extern fn(*mut rdline,*const u8,u32)->c_void;
pub type F3 = unsafe extern fn(*mut rdline,*const u8,*mut u8,u32,*mut c_int)->c_int;
#[repr(C)]
pub struct termios{
	c_iflag:u16,   
	c_oflag:u16,  
	c_cflag:u16,   
	c_lflag:u16,   
	c_line :u8 ,   
	c_cc   :[u8;8],
}
#[repr(C)]
pub struct rte_eth_rss_reta_entry64 {
	mask:u64,
	reta:[u16;64],
}
#[repr(C)]
pub struct rte_eth_rss_conf {
	rss_key:*mut u8,    
	rss_key_len:u8,
	rss_hf:u64, 
}
#[repr(C)]
pub struct rte_eth_udp_tunnel {
	udp_port:u16,
	prot_type:u8, 
}
#[repr(C,align(2))]
pub struct rte_ether_addr {
	addr_bytes:[u8;6],
} 
#[repr(C)]
pub struct rte_eth_vlan_mirror {
	vlan_mask:u64,
	vlan_id:[u16;64],
}
pub struct rte_eth_mirror_conf {
	rule_type:u8,
	dst_pool:u8, 
	pool_mask:u64, 
	vlan:rte_eth_vlan_mirror,
}
//rdline 
#[repr(C)]
pub enum rdline_status{
	RdlineInit,
	RdlineRunning,
	RdlineExited,
}
#[repr(C)]
pub enum rte_eth_fc_mode {
	RTE_FC_NONE = 0, 
	RTE_FC_RX_PAUSE, 
	RTE_FC_TX_PAUSE, 
	RTE_FC_FULL      
}
#[repr(C)]
pub struct cirbuf{
	maxlen:u32,
	start:u32,
	end:u32,
	len:u32,
	buf:*mut u8,
}
#[repr(C)]
pub enum cmdline_vt100_parser_state {
	CmdlineVt100Init,
	CmdlineVt100Escape,
	CmdlineVt100EscapeCsi,
}
#[repr(C)]
pub enum rte_vlan_type {
	ETH_VLAN_TYPE_UNKNOWN = 0,
	ETH_VLAN_TYPE_INNER, /**< Inner VLAN. */
	ETH_VLAN_TYPE_OUTER, /**< Single VLAN, or outer VLAN. */
	ETH_VLAN_TYPE_MAX,
}
#[repr(C)]
pub struct rte_eth_hairpin_cap {
	max_nb_queues:u16,
	max_rx_2_tx:u16,
	max_tx_2_rx:u16,
	max_nb_desc:u16, 
}
#[repr(C)]
pub struct cmdline_vt100{
	bufpos:u8,
	buf:[u8;8],
	state:cmdline_vt100_parser_state,
}
#[repr(C)]
pub struct rdline{
	status: rdline_status,
	left: cirbuf,
	right: cirbuf,
	left_buff: [u8;514],
	right_buff: [u8;512],
	
	prompt:[u8;32],
	prompt_size:u32,

	kill_buf:[u8;512],
	kill_size:u32,
	history:cirbuf,
	history_buf:[u8;32],
	history_cur_line:c_int,
	write_char:*mut F1,
	validate:*mut F2,
	complete:*mut F3,
	vt100:cmdline_vt100,
	opaque:*mut c_void,
}
#[repr(C)]
pub struct cmdline_token_ops{
	parse:FUNC1,
	complete_get_nb:FUNC2,
	complete_get_elt:FUNC3,
	get_help:FUNC4,
}
#[repr(C)]
pub struct cmdline_token_hdr{
	ops:*mut cmdline_token_ops,
	offset:u32,
}
#[repr(C)]
pub struct cmdline_inst{
	f:FUNC,
	data:*mut c_void,
	help_str:*const u8,
	tokens:*mut(*mut cmdline_token_hdr),
}
//cmdline结构体
#[repr(C)]
pub struct cmdline{
	pub s_int:c_int,
	pub s_out:c_int,
  //ctx是指向结构体的二级指针
	pub ctx: *mut (*mut cmdline_inst),
	pub rdl:rdline,
	pub prompt: [u8;32],
	pub oldterm:termios,
}
#[repr(C)]
pub struct rte_eth_dev_owner{
	id:u64,
	name:[u8;64]
}
#[repr(C)]
pub struct rte_eth_xstat_name {
	name:[u8;64],
}
#[repr(C)]
pub struct rte_eth_xstat {
	id:u64,        
	value:u64,    
}
#[repr(C)]
pub struct rte_eth_fc_conf {
	high_water:u32, 
	low_water:u32,   
	pause_time:u16,  
	send_xon:u16,   
	mode:rte_eth_fc_mode,  
	mac_ctrl_frame_fwd:u8, 
	autoneg:u8,     
}
#[repr(C)]
pub struct rte_eth_pfc_conf {
	fc:rte_eth_fc_conf,
	priority:u8,         
}
#[link(name = "rte_eal")]
extern "C"{
	pub fn rte_eal_init(argc: u32,argv:&Vec<String>)->c_int;
}
pub fn rte_eal_init_safe(argv:Vec<String>)->c_int{
	let ret = unsafe{
		rte_eal_init(argv.len().try_into().unwrap(),&argv)
	};
	ret
}
#[link(name = "rte_eal")]
extern "C"{
	pub fn rte_get_next_lcore(i:i32,skip_main:i32,wrap:i32)->i32;
}
#[link(name = "rte_eal")]
extern "C"{
    //in c unsigned = u32
    pub fn rte_eal_remote_launch(f:LcoreFunctionT,arg:c_int,worker_id:i32)->i32;
}
#[link(name = "rte_eal")]
extern "C"{
	pub fn rte_eal_mp_wait_lcore()->();
}
#[link(name = "rte_eal")]
extern "C"{
	pub fn rte_eal_cleanup()->c_int;
}
#[link(name = "rte_cmdline")]
extern "C" {
	pub fn cmdline_stdin_new(ctx: *mut (*mut cmdline_inst), prompt:*const u8)->*mut cmdline;
}
#[link(name = "rte_cmdline")]
extern "C" {
	pub fn cmdline_interact(cl: *mut cmdline);
}
#[link(name = "rte_cmdline")]
extern "C" {
	pub fn cmdline_stdin_exit(cl: *mut cmdline);
}
#[link(name = "rte_ethdev")]
extern "C" {
	pub fn rte_eth_dev_is_valid_port(port_id:u16)->c_int;
	pub fn rte_eth_find_next_owned_by(port_id:u16,owner_id:u64)->u64;
	//unsafe
	pub fn rte_eth_dev_owner_new(owner_id:*const u64)->c_int;
	//unsafe
	pub fn rte_eth_dev_owner_set(port_id:u16,owner:*const rte_eth_dev_owner)->c_int;
	pub fn rte_eth_dev_owner_unset(port_id:u16,owner_id:u64)->c_int;
	pub fn rte_eth_dev_owner_delete(owner_id:u64)->c_int;
	//unsafe
    pub fn rte_eth_dev_owner_get(port_id:u16, owner:*const rte_eth_dev_owner)->c_int;
	pub fn rte_eth_dev_socket_id(port_id:u16)->c_int;
	pub fn rte_eth_dev_get_sec_ctx(port_id:u16)->*mut c_void;
	pub fn rte_eth_dev_count_avail()->u16;
	pub fn rte_eth_dev_count_total()->u16;
	//unsafe
	pub fn rte_eth_dev_get_name_by_port(port_id:u16, name:*const u8)->c_int;
	//unsafe
	pub fn rte_eth_dev_get_port_by_name(name:*const u8, port_id:*const u16)->c_int;
	pub fn rte_eth_dev_rx_queue_start(port_id:u16, rx_queue_id:u16)->c_int;
	pub fn rte_eth_dev_rx_queue_stop(port_id:u16, rx_queue_id:u16)->c_int;
	pub fn rte_eth_dev_tx_queue_start(port_id:u16, tx_queue_id:u16)->c_int;
	pub fn rte_eth_dev_tx_queue_stop(port_id:u16, tx_queue_id:u16)->c_int;
	pub fn rte_eth_dev_rx_offload_name(offload:u64)->*const u8;
	pub fn rte_eth_dev_tx_offload_name(offload:u64)->*const u8;
	pub fn rte_eth_dev_start(port_id:u16)->c_int;
	pub fn rte_eth_dev_stop(port_id:u16)->c_int;
	pub fn rte_eth_dev_set_link_up(port_id:u16)->c_int;
	pub fn rte_eth_dev_set_link_down(port_id:u16)->c_int;
	pub fn rte_eth_dev_close(port_id:u16)->c_int;
	pub fn rte_eth_dev_reset(port_id:u16)->c_int;
	pub fn rte_eth_dev_is_removed(port_id:u16)->c_int;
	//unsafe
	pub fn rte_eth_xstats_get_id_by_name(port_id:u16, xstat_name:*const u8,id:*const u64)->c_int;
	//unsafe 
	pub fn rte_eth_xstats_get_names_by_id(port_id:u16,
		xstats_names:*const rte_eth_xstat_name, size:u32,ids:*const u64)->c_int;
	//unsafe
	pub fn rte_eth_xstats_get_names(port_id:u16,
		xstats_names:*const rte_eth_xstat_name,size:u32)->c_int;
	//unsafe
	pub fn rte_eth_xstats_get_by_id(port_id:u16, ids:*const u64,
		values:*const u64, size:u32)->c_int;
	//unsafe
	pub fn rte_eth_xstats_get(port_id:u16,xstats:*const rte_eth_xstat,n:u32)->c_int;
	pub fn rte_eth_xstats_reset(port_id:u16)->c_int;
	pub fn rte_eth_dev_set_tx_queue_stats_mapping(port_id:u16, 
		tx_queue_id:u16,stat_idx:u8)->c_int;
	pub fn rte_eth_dev_set_rx_queue_stats_mapping(port_id:u16, rx_queue_id:u16,
		stat_idx:u8)->c_int;
	//unsafe
	pub fn rte_eth_dev_fw_version_get(port_id:u16, fw_version:*const u8, fw_size:c_int)->c_int;
	//unsafe
	pub fn rte_eth_dev_get_supported_ptypes(port_id:u16, ptype_mask:u32,
		ptypes:*const u32, num:c_int)->c_int;
	//unsafe
	pub fn rte_eth_dev_set_ptypes(port_id:u16, ptype_mask:u32,
		set_ptypes:*const u32, num:u32)->c_int;
	//unsafe
	pub fn rte_eth_dev_get_mtu(port_id:u16, mtu:*const u16)->c_int;
	pub fn rte_eth_dev_set_mtu(port_id:u16, mtu:u16)->c_int;
	pub fn rte_eth_dev_vlan_filter(port_id:u16, vlan_id:u16,on:c_int)->c_int;
	pub fn rte_eth_dev_set_vlan_strip_on_queue(port_id:u16, rx_queue_id:u16,on:c_int)->c_int;
	pub fn rte_eth_dev_set_vlan_ether_type(port_id:u16,vlan_type:rte_vlan_type,tpid:u16)->c_int;
	pub fn rte_eth_dev_set_vlan_offload(port_id:u16, offload_mask:c_int)->c_int;
	pub fn rte_eth_dev_get_vlan_offload(port_id:u16)->c_int;
	pub fn rte_eth_dev_set_vlan_pvid(port_id:u16, pvid:u16, on:c_int)->c_int;
	//unsafe
	pub fn rte_eth_dev_flow_ctrl_get(port_id:u16, fc_conf:*const rte_eth_fc_conf)->c_int;
	//unsafe
	pub fn rte_eth_dev_flow_ctrl_set(port_id:u16, fc_conf:*const rte_eth_fc_conf)->c_int;
	//unsafe
	pub fn rte_eth_dev_priority_flow_ctrl_set(port_id:u16,pfc_conf:*const rte_eth_pfc_conf)->c_int;
	//unsafe 
	pub fn rte_eth_dev_rss_reta_update(port_id:u16,
		reta_conf:*const rte_eth_rss_reta_entry64,
			reta_size:u16)->c_int;
	pub fn rte_eth_dev_rss_reta_query(port_id:u16,
		reta_conf:*const rte_eth_rss_reta_entry64,
			reta_size:u16)->c_int;	
	//unsafe	
	pub fn rte_eth_dev_rss_hash_update(port_id:u16,rss_conf:*const rte_eth_rss_conf)->c_int;
	//unsafe	 
	pub fn rte_eth_dev_rss_hash_conf_get(port_id:u16,rss_conf:*const rte_eth_rss_conf)->c_int; 
	//unsafe 
	pub fn rte_eth_dev_udp_tunnel_port_add(port_id:u16,udp_tunnel:*const rte_eth_udp_tunnel)->c_int;
	//unsafe
	pub fn rte_eth_dev_udp_tunnel_port_delete(port_id:u16,udp_tunnel:*const rte_eth_udp_tunnel)->c_int;
	//unsafe
	pub fn rte_eth_dev_mac_addr_add(port_id:u16, addr:*const rte_ether_addr,
			pool:u32)->c_int;
	//unsafe
	pub fn rte_eth_dev_mac_addr_remove(port_id:u16, addr:*const rte_ether_addr,
			pool:u32)->c_int;
	//unsafe
	pub fn rte_eth_dev_default_mac_addr_set(port_id:u16, addr:*const rte_ether_addr)->c_int;
	//unsafe
	pub fn rte_eth_dev_uc_hash_table_set(port_id:u16, addr:*const rte_ether_addr,
				on:u8)->c_int;
	pub fn rte_eth_dev_uc_all_hash_table_set(port_id:u16, on:u8)->c_int;
	pub fn rte_eth_set_queue_rate_limit(port_id:u16, queue_idx:u16,tx_rate:u16)->c_int;
	//unsafe
	pub fn rte_eth_mirror_rule_set(port_id:u16,
			mirror_conf:*const rte_eth_mirror_conf,
			rule_id:u8, on:u8)->c_int;
	pub fn rte_eth_mirror_rule_reset(port_id:u16, rule_id:u8)->c_int;
	pub fn rte_eth_dev_rx_intr_ctl_q_get_fd(port_id:u16, queue_id:u16)->c_int;
	pub fn rte_eth_dev_rx_intr_ctl_q(port_id:u16, queue_id:u16,
			  epfd:c_int, op:c_int, data:*mut c_void)->c_int;
	pub fn rte_eth_dev_rx_intr_enable(port_id:u16,queue_id:u16)->c_int;
	pub fn rte_eth_dev_rx_intr_disable(port_id:u16,queue_id:u16)->c_int;
	pub fn rte_eth_timesync_enable(port_id:u16)->c_int;
	pub fn rte_eth_timesync_disable(port_id:u16)->c_int;
	pub fn rte_eth_timesync_adjust_time(port_id:u16, delta:i64)->c_int;
	//unsafe
	pub fn rte_eth_read_clock(port_id:u16, clock:*const u64)->c_int;
	pub fn rte_eth_dev_get_eeprom_length(port_id:u16)->c_int;
	//unsafe
	pub fn rte_eth_dev_adjust_nb_rx_tx_desc(port_id:u16,
				 nb_rx_desc:*const u16,
				 nb_tx_desc:*const u16)->c_int;
	//unsafe
	pub fn rte_eth_dev_hairpin_capability_get(port_id:u16,cap:*const rte_eth_hairpin_cap)->c_int;
	//unsafe
	pub fn rte_eth_dev_pool_ops_supported(port_id:u16, pool:*const u8)->c_int;
	//unsafe
	pub fn rte_eth_switch_domain_alloc(domain_id:*const u16)->c_int;
	pub fn rte_eth_switch_domain_free(domain_id:u16)->c_int;
	pub fn rte_eth_hairpin_queue_peer_unbind(cur_port:u16, cur_queue:u16,direction:u32)->c_int;
}
//safe-ffi
pub fn rte_eth_dev_owner_new_safe(src: &[u64])->c_int{
	unsafe{ rte_eth_dev_owner_new(src.as_ptr()) }
}
pub fn rte_eth_dev_get_name_by_port_safe(port_id:u16,src: &[u8])->c_int{
	unsafe{ rte_eth_dev_get_name_by_port(port_id, src.as_ptr())}
}
pub fn rte_eth_dev_get_port_by_name_safe(name:&[u8], port_id:&[u16]){
	unsafe{ rte_eth_dev_get_port_by_name(name.as_ptr(), port_id.as_ptr())};
}
pub fn rte_eth_dev_owner_set_safe(port_id:u16,owner:&[rte_eth_dev_owner])->c_int{
	unsafe{ rte_eth_dev_owner_set(port_id,owner.as_ptr())}
}
pub fn rte_eth_dev_owner_get_safe(port_id:u16,owner:&[rte_eth_dev_owner])->c_int{
	unsafe{ rte_eth_dev_owner_get(port_id, owner.as_ptr())}
}
pub fn rte_eth_xstats_get_id_by_name_safe(port_id:u16, xstat_name:&[u8],id:&[u64])->c_int{
	unsafe{ rte_eth_xstats_get_id_by_name(port_id, xstat_name.as_ptr(),id.as_ptr())}
}
pub fn rte_eth_xstats_get_names_by_id_safe(port_id:u16,
	xstats_names:&[rte_eth_xstat_name], size:u32,ids:&[u64])->c_int{
	unsafe{ rte_eth_xstats_get_names_by_id(port_id,
	xstats_names.as_ptr(), size,ids.as_ptr())}
}
pub fn rte_eth_xstats_get_names_safe(port_id:u16,
	xstats_names:&[rte_eth_xstat_name],size:u32)->c_int{
	unsafe{ rte_eth_xstats_get_names(port_id,
	xstats_names.as_ptr(),size)}
}
pub fn rte_eth_xstats_get_by_id_safe(port_id:u16, ids:&[u64],
	values:&[u64], size:u32)->c_int{
	unsafe{ rte_eth_xstats_get_by_id(port_id, ids.as_ptr(),
		values.as_ptr(), size)}
}
pub fn rte_eth_xstats_get_safe(port_id:u16,xstats:&[rte_eth_xstat],n:u32)->c_int{
	unsafe{ rte_eth_xstats_get(port_id,xstats.as_ptr(),n)}
}
pub fn rte_eth_dev_fw_version_get_safe(port_id:u16, fw_version:&[u8], fw_size:c_int)->c_int{
	unsafe{ rte_eth_dev_fw_version_get(port_id, fw_version.as_ptr(), fw_size)}
}
pub fn rte_eth_dev_get_supported_ptypes_safe(port_id:u16, ptype_mask:u32,
	ptypes:&[u32], num:c_int)->c_int{
	unsafe{ rte_eth_dev_get_supported_ptypes(port_id, ptype_mask,
			ptypes.as_ptr(), num)}
}
pub fn rte_eth_dev_set_ptypes_safe(port_id:u16, ptype_mask:u32,
	set_ptypes:&[u32], num:u32)->c_int{
	unsafe{ rte_eth_dev_set_ptypes(port_id, ptype_mask,
		set_ptypes.as_ptr(), num)}
}
pub fn rte_eth_dev_get_mtu_safe(port_id:u16, mtu:&[u16])->c_int{
	unsafe{ rte_eth_dev_get_mtu(port_id, mtu.as_ptr())}
}
pub fn rte_eth_dev_flow_ctrl_get_safe(port_id:u16, fc_conf:&[rte_eth_fc_conf])->c_int{
	unsafe{	rte_eth_dev_flow_ctrl_get(port_id, fc_conf.as_ptr())}
}
pub fn rte_eth_dev_flow_ctrl_set_safe(port_id:u16, fc_conf:&[rte_eth_fc_conf])->c_int{
	unsafe{	rte_eth_dev_flow_ctrl_set(port_id, fc_conf.as_ptr())}
}
pub fn rte_eth_dev_priority_flow_ctrl_set_safe(port_id:u16,pfc_conf:&[rte_eth_pfc_conf])->c_int{
	unsafe{ rte_eth_dev_priority_flow_ctrl_set(port_id,pfc_conf.as_ptr())}
}
pub fn rte_eth_dev_rss_reta_update_safe(port_id:u16,
	reta_conf:&[rte_eth_rss_reta_entry64],
	   reta_size:u16)->c_int{	
	unsafe{	rte_eth_dev_rss_reta_update(port_id,
			reta_conf.as_ptr(),
			   reta_size)}	
}
pub fn rte_eth_dev_rss_reta_query_safe(port_id:u16,
	reta_conf:&[rte_eth_rss_reta_entry64],
	   reta_size:u16)->c_int{	
	unsafe{	rte_eth_dev_rss_reta_query(port_id,
			reta_conf.as_ptr(),
			   reta_size)}	
}
pub fn rte_eth_dev_rss_hash_update_safe(port_id:u16,rss_conf:&[rte_eth_rss_conf])->c_int{
	unsafe{ rte_eth_dev_rss_hash_update(port_id,rss_conf.as_ptr())}
}
pub fn rte_eth_dev_rss_hash_conf_get_safe(port_id:u16,rss_conf:&[rte_eth_rss_conf])->c_int{
	unsafe{ rte_eth_dev_rss_hash_conf_get(port_id,rss_conf.as_ptr())}
}
pub fn rte_eth_dev_udp_tunnel_port_add_safe(port_id:u16,udp_tunnel:&[rte_eth_udp_tunnel])->c_int{
	unsafe{ rte_eth_dev_udp_tunnel_port_add(port_id,udp_tunnel.as_ptr())}
}
pub fn rte_eth_dev_udp_tunnel_port_delete_safe(port_id:u16,udp_tunnel:&[rte_eth_udp_tunnel])->c_int{
	unsafe{ rte_eth_dev_udp_tunnel_port_delete(port_id,udp_tunnel.as_ptr())}
}
pub fn rte_eth_dev_mac_addr_add_safe(port_id:u16, addr:&[rte_ether_addr],
	pool:u32)->c_int{
	unsafe{ rte_eth_dev_mac_addr_add(port_id, addr.as_ptr(),pool)}
}
pub fn rte_eth_dev_mac_addr_remove_safe(port_id:u16, addr:&[rte_ether_addr],
	pool:u32)->c_int{
	unsafe{ rte_eth_dev_mac_addr_remove(port_id, addr.as_ptr(),pool)}
}
pub fn rte_eth_dev_default_mac_addr_set_safe(port_id:u16, addr:&[rte_ether_addr])->c_int{
	unsafe{ rte_eth_dev_default_mac_addr_set(port_id, addr.as_ptr())}
}
pub fn rte_eth_dev_uc_hash_table_set_safe(port_id:u16, addr:&[rte_ether_addr],
	on:u8)->c_int{
	unsafe{ rte_eth_dev_uc_hash_table_set(port_id, addr.as_ptr(),on)}	
}
pub fn rte_eth_mirror_rule_set_safe(port_id:u16,
	mirror_conf:&[rte_eth_mirror_conf],
		rule_id:u8, on:u8)->c_int{
	unsafe{rte_eth_mirror_rule_set(port_id,
		mirror_conf.as_ptr(),rule_id, on)}		
}
pub fn rte_eth_read_clock_safe(port_id:u16, clock:&[u64])->c_int{
	unsafe{rte_eth_read_clock(port_id, clock.as_ptr())}
}
pub fn rte_eth_dev_adjust_nb_rx_tx_desc_safe(port_id:u16,
	nb_rx_desc:&[u16],
	nb_tx_desc:&[u16])->c_int{
	unsafe{rte_eth_dev_adjust_nb_rx_tx_desc(port_id,
		nb_rx_desc.as_ptr(),
		nb_tx_desc.as_ptr())}	
}
pub fn rte_eth_dev_hairpin_capability_get_safe(port_id:u16,cap:&[rte_eth_hairpin_cap])->c_int{
	unsafe{ rte_eth_dev_hairpin_capability_get(port_id,cap.as_ptr())}
}
pub fn rte_eth_dev_pool_ops_supported_safe(port_id:u16, pool:&[u8])->c_int{
	unsafe{ rte_eth_dev_pool_ops_supported(port_id, pool.as_ptr())}
}
pub fn rte_eth_switch_domain_alloc_safe(domain_id:&[u16])->c_int{
	unsafe{ rte_eth_switch_domain_alloc(domain_id.as_ptr())}
}