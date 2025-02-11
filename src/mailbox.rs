#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mboxirq: (),
    _reserved1: [u8; 0xf8],
    mutex: Mutex,
}
impl RegisterBlock {
    #[doc = "0x00..0x18 - no description available"]
    #[inline(always)]
    pub const fn mboxirq(&self, n: usize) -> &Mboxirq {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - no description available"]
    #[inline(always)]
    pub fn mboxirq_iter(&self) -> impl Iterator<Item = &Mboxirq> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() })
    }
    #[doc = "0xf8 - Mutual exclusion register\\[1\\]"]
    #[inline(always)]
    pub const fn mutex(&self) -> &Mutex {
        &self.mutex
    }
}
#[doc = "no description available"]
pub use self::mboxirq::Mboxirq;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod mboxirq;
#[doc = "MUTEX (rw) register accessor: Mutual exclusion register\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mutex`]
module"]
#[doc(alias = "MUTEX")]
pub type Mutex = crate::Reg<mutex::MutexSpec>;
#[doc = "Mutual exclusion register\\[1\\]"]
pub mod mutex;
