#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    updatelckout: Updatelckout,
    _reserved1: [u8; 0x3c],
    fcctrlsel: [Fcctrlsel; 8],
    _reserved2: [u8; 0x20],
    sharedctrlset: [Sharedctrlset; 2],
    _reserved3: [u8; 0x78],
    usb_hs_status: UsbHsStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - update lock out control"]
    #[inline(always)]
    pub const fn updatelckout(&self) -> &Updatelckout {
        &self.updatelckout
    }
    #[doc = "0x40..0x60 - Selects the source for SCK going into Flexcomm index"]
    #[inline(always)]
    pub const fn fcctrlsel(&self, n: usize) -> &Fcctrlsel {
        &self.fcctrlsel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Selects the source for SCK going into Flexcomm index"]
    #[inline(always)]
    pub fn fcctrlsel_iter(&self) -> impl Iterator<Item = &Fcctrlsel> {
        self.fcctrlsel.iter()
    }
    #[doc = "0x80..0x88 - Selects sources and data combinations for shared signal set index."]
    #[inline(always)]
    pub const fn sharedctrlset(&self, n: usize) -> &Sharedctrlset {
        &self.sharedctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - Selects sources and data combinations for shared signal set index."]
    #[inline(always)]
    pub fn sharedctrlset_iter(&self) -> impl Iterator<Item = &Sharedctrlset> {
        self.sharedctrlset.iter()
    }
    #[doc = "0x100 - Status register for USB HS"]
    #[inline(always)]
    pub const fn usb_hs_status(&self) -> &UsbHsStatus {
        &self.usb_hs_status
    }
}
#[doc = "UPDATELCKOUT (rw) register accessor: update lock out control\n\nYou can [`read`](crate::Reg::read) this register and get [`updatelckout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`updatelckout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@updatelckout`]
module"]
#[doc(alias = "UPDATELCKOUT")]
pub type Updatelckout = crate::Reg<updatelckout::UpdatelckoutSpec>;
#[doc = "update lock out control"]
pub mod updatelckout;
#[doc = "FCCTRLSEL (rw) register accessor: Selects the source for SCK going into Flexcomm index\n\nYou can [`read`](crate::Reg::read) this register and get [`fcctrlsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcctrlsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcctrlsel`]
module"]
#[doc(alias = "FCCTRLSEL")]
pub type Fcctrlsel = crate::Reg<fcctrlsel::FcctrlselSpec>;
#[doc = "Selects the source for SCK going into Flexcomm index"]
pub mod fcctrlsel;
#[doc = "SHAREDCTRLSET (rw) register accessor: Selects sources and data combinations for shared signal set index.\n\nYou can [`read`](crate::Reg::read) this register and get [`sharedctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharedctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sharedctrlset`]
module"]
#[doc(alias = "SHAREDCTRLSET")]
pub type Sharedctrlset = crate::Reg<sharedctrlset::SharedctrlsetSpec>;
#[doc = "Selects sources and data combinations for shared signal set index."]
pub mod sharedctrlset;
#[doc = "USB_HS_STATUS (r) register accessor: Status register for USB HS\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_hs_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_hs_status`]
module"]
#[doc(alias = "USB_HS_STATUS")]
pub type UsbHsStatus = crate::Reg<usb_hs_status::UsbHsStatusSpec>;
#[doc = "Status register for USB HS"]
pub mod usb_hs_status;
