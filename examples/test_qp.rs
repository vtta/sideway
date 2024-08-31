use std::net::Ipv6Addr;

use rdma_mummy_sys::ibv_access_flags;
use sideway::verbs::{
    address_handle::{AddressHandleAttribute, Gid},
    device,
    device_context::Mtu,
    queue_pair::{QueuePairAttribute, QueuePairState},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_list = device::DeviceList::new()?;
    for device in &device_list {
        let ctx = device.open().unwrap();

        let pd = ctx.alloc_pd().unwrap();
        let mr = pd.reg_managed_mr(64).unwrap();

        let comp_channel = ctx.create_comp_channel().unwrap();
        let mut cq_builder = ctx.create_cq_builder();
        let sq = cq_builder.setup_cqe(128).build().unwrap();
        let rq = cq_builder.setup_cqe(128).build().unwrap();

        let mut builder = pd.create_qp_builder();

        let mut qp = builder
            .setup_max_inline_data(128)
            .setup_send_cq(&sq)
            .setup_recv_cq(&rq)
            .build()
            .unwrap();

        println!("qp pointer is {:?}", qp);
        // modify QP: RESET -> INIT
        // IBV_QP_STATE, IBV_QP_PKEY_INDEX, IBV_QP_PORT, IBV_QP_ACCESS_FLAGS
        let mut attr = QueuePairAttribute::new();
        attr.setup_state(QueuePairState::Init)
            .setup_pkey_index(0)
            .setup_port(1)
            .setup_access_flags(ibv_access_flags::IBV_ACCESS_REMOTE_WRITE);
        qp.modify(&attr).unwrap();

        // modify QP: INIT -> RTR
        // IBV_QP_STATE, IBV_QP_AV, IBV_QP_PATH_MTU, IBV_QP_DEST_QPN, IBV_QP_RQ_PSN, IBV_QP_MAX_DEST_RD_ATOMIC, IBV_QP_MIN_RNR_TIMER
        let mut attr = QueuePairAttribute::new();
        attr.setup_state(QueuePairState::ReadyToReceive)
            .setup_path_mtu(Mtu::Mtu1024)
            .setup_dest_qp_num(12345)
            .setup_rq_psn(1)
            .setup_max_dest_read_atomic(0)
            .setup_min_rnr_timer(0);
        // setup address vector
        let mut ah_attr = AddressHandleAttribute::new();
        ah_attr
            .setup_dest_lid(1)
            .setup_port(1)
            .setup_service_level(1)
            .setup_grh_src_gid_index(1)
            .setup_grh_dest_gid(&Gid {
                raw: "::ffff:192.168.1.1".parse::<Ipv6Addr>().unwrap().octets(),
            })
            .setup_grh_hop_limit(64);
        attr.setup_address_vector(&ah_attr);
        qp.modify(&attr).unwrap();

        // modify QP: RTR -> RTS
        // IBV_QP_STATE, IBV_QP_SQ_PSN, IBV_QP_TIMEOUT, IBV_QP_RETRY_CNT, IBV_QP_RNR_RETRY, IBV_QP_MAX_QP_RD_ATOMIC
        let mut attr = QueuePairAttribute::new();
        attr.setup_state(QueuePairState::ReadyToSend)
            .setup_sq_psn(1)
            .setup_timeout(0)
            .setup_retry_cnt(3)
            .setup_rnr_retry(0)
            .setup_max_read_atomic(0);
        qp.modify(&attr).unwrap();
    }

    Ok(())
}
