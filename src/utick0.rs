#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    stat: Stat,
    cfg: Cfg,
    capclr: Capclr,
    cap: [Cap; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status register."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - Capture configuration register."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x0c - Capture clear register."]
    #[inline(always)]
    pub const fn capclr(&self) -> &Capclr {
        &self.capclr
    }
    #[doc = "0x10..0x20 - Capture register ."]
    #[inline(always)]
    pub const fn cap(&self, n: usize) -> &Cap {
        &self.cap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Capture register ."]
    #[inline(always)]
    pub fn cap_iter(&self) -> impl Iterator<Item = &Cap> {
        self.cap.iter()
    }
}
#[doc = "CTRL (rw) register accessor: Control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register."]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register."]
pub mod stat;
#[doc = "CFG (rw) register accessor: Capture configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Capture configuration register."]
pub mod cfg;
#[doc = "CAPCLR (w) register accessor: Capture clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capclr`]
module"]
#[doc(alias = "CAPCLR")]
pub type Capclr = crate::Reg<capclr::CapclrSpec>;
#[doc = "Capture clear register."]
pub mod capclr;
#[doc = "CAP (r) register accessor: Capture register .\n\nYou can [`read`](crate::Reg::read) this register and get [`cap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
#[doc(alias = "CAP")]
pub type Cap = crate::Reg<cap::CapSpec>;
#[doc = "Capture register ."]
pub mod cap;
