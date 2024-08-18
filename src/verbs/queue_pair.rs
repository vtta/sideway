use rdma_mummy_sys::ibv_qp_type::IBV_QPT_RC;
use rdma_mummy_sys::{self, ibv_create_qp, ibv_qp, ibv_qp_cap, ibv_qp_init_attr, ibv_srq};
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::{marker::PhantomData, ptr::NonNull};

use super::{completion::CompletionQueue, protection_domain::ProtectionDomain};

// pd, cq
pub struct QueuePair<'a> {
    pub(crate) qp: NonNull<ibv_qp>,
    _phantom: PhantomData<&'a ()>,
}

pub struct QueuePairBuilder<'a> {
    pd: &'a ProtectionDomain<'a>,
    init_attr: ibv_qp_init_attr,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> QueuePairBuilder<'a> {
    pub fn new(pd: &'a ProtectionDomain, send_cq: &'a CompletionQueue, recv_cq: &'a CompletionQueue) -> Self {
        // set default params for init_attr
        QueuePairBuilder {
            pd,
            init_attr: ibv_qp_init_attr {
                qp_context: null_mut::<c_void>(),
                send_cq: send_cq.cq.as_ptr(),
                recv_cq: recv_cq.cq.as_ptr(),
                srq: null_mut::<ibv_srq>(),
                cap: ibv_qp_cap {
                    max_send_wr: 16,
                    max_recv_wr: 16,
                    max_send_sge: 1,
                    max_recv_sge: 1,
                    max_inline_data: 0,
                },
                qp_type: IBV_QPT_RC,
                sq_sig_all: 0,
            },
            _phantom: PhantomData,
        }
    }
}
