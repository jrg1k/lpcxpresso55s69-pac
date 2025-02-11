#[repr(C)]
#[doc = "no description available"]
#[doc(alias = "MBOXIRQ")]
pub struct Mboxirq {
    irq: Irq,
    irqset: Irqset,
    irqclr: Irqclr,
}
impl Mboxirq {
    #[doc = "0x00 - Interrupt request register for the Cortex-M0+ CPU."]
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    #[doc = "0x04 - Set bits in IRQ0"]
    #[inline(always)]
    pub const fn irqset(&self) -> &Irqset {
        &self.irqset
    }
    #[doc = "0x08 - Clear bits in IRQ0"]
    #[inline(always)]
    pub const fn irqclr(&self) -> &Irqclr {
        &self.irqclr
    }
}
#[doc = "IRQ (rw) register accessor: Interrupt request register for the Cortex-M0+ CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`]
module"]
#[doc(alias = "IRQ")]
pub type Irq = crate::Reg<irq::IrqSpec>;
#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
pub mod irq;
#[doc = "IRQSET (w) register accessor: Set bits in IRQ0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqset`]
module"]
#[doc(alias = "IRQSET")]
pub type Irqset = crate::Reg<irqset::IrqsetSpec>;
#[doc = "Set bits in IRQ0"]
pub mod irqset;
#[doc = "IRQCLR (w) register accessor: Clear bits in IRQ0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqclr`]
module"]
#[doc(alias = "IRQCLR")]
pub type Irqclr = crate::Reg<irqclr::IrqclrSpec>;
#[doc = "Clear bits in IRQ0"]
pub mod irqclr;
