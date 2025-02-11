#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    random_number: RandomNumber,
    _reserved1: [u8; 0x04],
    counter_val: CounterVal,
    counter_cfg: CounterCfg,
    online_test_cfg: OnlineTestCfg,
    online_test_val: OnlineTestVal,
    _reserved5: [u8; 0x0fe4],
    moduleid: Moduleid,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains a random 32 bit number which is computed on demand, at each time it is read"]
    #[inline(always)]
    pub const fn random_number(&self) -> &RandomNumber {
        &self.random_number
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn counter_val(&self) -> &CounterVal {
        &self.counter_val
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn counter_cfg(&self) -> &CounterCfg {
        &self.counter_cfg
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn online_test_cfg(&self) -> &OnlineTestCfg {
        &self.online_test_cfg
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn online_test_val(&self) -> &OnlineTestVal {
        &self.online_test_val
    }
    #[doc = "0xffc - IP identifier"]
    #[inline(always)]
    pub const fn moduleid(&self) -> &Moduleid {
        &self.moduleid
    }
}
#[doc = "RANDOM_NUMBER (r) register accessor: This register contains a random 32 bit number which is computed on demand, at each time it is read\n\nYou can [`read`](crate::Reg::read) this register and get [`random_number::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@random_number`]
module"]
#[doc(alias = "RANDOM_NUMBER")]
pub type RandomNumber = crate::Reg<random_number::RandomNumberSpec>;
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
pub mod random_number;
#[doc = "COUNTER_VAL (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter_val`]
module"]
#[doc(alias = "COUNTER_VAL")]
pub type CounterVal = crate::Reg<counter_val::CounterValSpec>;
#[doc = "no description available"]
pub mod counter_val;
#[doc = "COUNTER_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter_cfg`]
module"]
#[doc(alias = "COUNTER_CFG")]
pub type CounterCfg = crate::Reg<counter_cfg::CounterCfgSpec>;
#[doc = "no description available"]
pub mod counter_cfg;
#[doc = "ONLINE_TEST_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`online_test_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`online_test_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@online_test_cfg`]
module"]
#[doc(alias = "ONLINE_TEST_CFG")]
pub type OnlineTestCfg = crate::Reg<online_test_cfg::OnlineTestCfgSpec>;
#[doc = "no description available"]
pub mod online_test_cfg;
#[doc = "ONLINE_TEST_VAL (rw) register accessor: no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`online_test_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`online_test_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@online_test_val`]
module"]
#[doc(alias = "ONLINE_TEST_VAL")]
pub type OnlineTestVal = crate::Reg<online_test_val::OnlineTestValSpec>;
#[doc = "no description available"]
pub mod online_test_val;
#[doc = "MODULEID (r) register accessor: IP identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`moduleid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moduleid`]
module"]
#[doc(alias = "MODULEID")]
pub type Moduleid = crate::Reg<moduleid::ModuleidSpec>;
#[doc = "IP identifier"]
pub mod moduleid;
