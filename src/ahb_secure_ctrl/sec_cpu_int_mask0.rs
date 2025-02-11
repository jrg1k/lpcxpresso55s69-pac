#[doc = "Register `SEC_CPU_INT_MASK0` reader"]
pub type R = crate::R<SecCpuIntMask0Spec>;
#[doc = "Register `SEC_CPU_INT_MASK0` writer"]
pub type W = crate::W<SecCpuIntMask0Spec>;
#[doc = "Watchdog Timer, Brown Out Detectors and Flash Controller interrupts\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SysIrq> for bool {
    #[inline(always)]
    fn from(variant: SysIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_IRQ` reader - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
pub type SysIrqR = crate::BitReader<SysIrq>;
impl SysIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysIrq {
        match self.bits {
            false => SysIrq::Invisible,
            true => SysIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SysIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SysIrq::Visible
    }
}
#[doc = "Field `SYS_IRQ` writer - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
pub type SysIrqW<'a, REG> = crate::BitWriter<'a, REG, SysIrq>;
impl<'a, REG> SysIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SysIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SysIrq::Visible)
    }
}
#[doc = "System DMA 0 (non-secure) interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdma0Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Sdma0Irq> for bool {
    #[inline(always)]
    fn from(variant: Sdma0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA0_IRQ` reader - System DMA 0 (non-secure) interrupt."]
pub type Sdma0IrqR = crate::BitReader<Sdma0Irq>;
impl Sdma0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma0Irq {
        match self.bits {
            false => Sdma0Irq::Invisible,
            true => Sdma0Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Sdma0Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Sdma0Irq::Visible
    }
}
#[doc = "Field `SDMA0_IRQ` writer - System DMA 0 (non-secure) interrupt."]
pub type Sdma0IrqW<'a, REG> = crate::BitWriter<'a, REG, Sdma0Irq>;
impl<'a, REG> Sdma0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0Irq::Visible)
    }
}
#[doc = "GPIO Group 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioGlobalint0Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioGlobalint0Irq> for bool {
    #[inline(always)]
    fn from(variant: GpioGlobalint0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_GLOBALINT0_IRQ` reader - GPIO Group 0 interrupt."]
pub type GpioGlobalint0IrqR = crate::BitReader<GpioGlobalint0Irq>;
impl GpioGlobalint0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioGlobalint0Irq {
        match self.bits {
            false => GpioGlobalint0Irq::Invisible,
            true => GpioGlobalint0Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioGlobalint0Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioGlobalint0Irq::Visible
    }
}
#[doc = "Field `GPIO_GLOBALINT0_IRQ` writer - GPIO Group 0 interrupt."]
pub type GpioGlobalint0IrqW<'a, REG> = crate::BitWriter<'a, REG, GpioGlobalint0Irq>;
impl<'a, REG> GpioGlobalint0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioGlobalint0Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioGlobalint0Irq::Visible)
    }
}
#[doc = "GPIO Group 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioGlobalint1Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioGlobalint1Irq> for bool {
    #[inline(always)]
    fn from(variant: GpioGlobalint1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_GLOBALINT1_IRQ` reader - GPIO Group 1 interrupt."]
pub type GpioGlobalint1IrqR = crate::BitReader<GpioGlobalint1Irq>;
impl GpioGlobalint1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioGlobalint1Irq {
        match self.bits {
            false => GpioGlobalint1Irq::Invisible,
            true => GpioGlobalint1Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioGlobalint1Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioGlobalint1Irq::Visible
    }
}
#[doc = "Field `GPIO_GLOBALINT1_IRQ` writer - GPIO Group 1 interrupt."]
pub type GpioGlobalint1IrqW<'a, REG> = crate::BitWriter<'a, REG, GpioGlobalint1Irq>;
impl<'a, REG> GpioGlobalint1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioGlobalint1Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioGlobalint1Irq::Visible)
    }
}
#[doc = "Pin interrupt 0 or pattern match engine slice 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq0 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq0> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` reader - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type GpioInt0Irq0R = crate::BitReader<GpioInt0Irq0>;
impl GpioInt0Irq0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq0 {
        match self.bits {
            false => GpioInt0Irq0::Invisible,
            true => GpioInt0Irq0::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq0::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq0::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ0` writer - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type GpioInt0Irq0W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq0>;
impl<'a, REG> GpioInt0Irq0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq0::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq0::Visible)
    }
}
#[doc = "Pin interrupt 1 or pattern match engine slice 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq1 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq1> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` reader - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type GpioInt0Irq1R = crate::BitReader<GpioInt0Irq1>;
impl GpioInt0Irq1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq1 {
        match self.bits {
            false => GpioInt0Irq1::Invisible,
            true => GpioInt0Irq1::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq1::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq1::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ1` writer - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type GpioInt0Irq1W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq1>;
impl<'a, REG> GpioInt0Irq1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq1::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq1::Visible)
    }
}
#[doc = "Pin interrupt 2 or pattern match engine slice 2 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq2 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq2> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` reader - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
pub type GpioInt0Irq2R = crate::BitReader<GpioInt0Irq2>;
impl GpioInt0Irq2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq2 {
        match self.bits {
            false => GpioInt0Irq2::Invisible,
            true => GpioInt0Irq2::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq2::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq2::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ2` writer - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
pub type GpioInt0Irq2W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq2>;
impl<'a, REG> GpioInt0Irq2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq2::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq2::Visible)
    }
}
#[doc = "Pin interrupt 3 or pattern match engine slice 3 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq3 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq3> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` reader - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
pub type GpioInt0Irq3R = crate::BitReader<GpioInt0Irq3>;
impl GpioInt0Irq3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq3 {
        match self.bits {
            false => GpioInt0Irq3::Invisible,
            true => GpioInt0Irq3::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq3::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq3::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ3` writer - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
pub type GpioInt0Irq3W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq3>;
impl<'a, REG> GpioInt0Irq3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq3::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq3::Visible)
    }
}
#[doc = "Micro Tick Timer interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UtickIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<UtickIrq> for bool {
    #[inline(always)]
    fn from(variant: UtickIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UTICK_IRQ` reader - Micro Tick Timer interrupt."]
pub type UtickIrqR = crate::BitReader<UtickIrq>;
impl UtickIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UtickIrq {
        match self.bits {
            false => UtickIrq::Invisible,
            true => UtickIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == UtickIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == UtickIrq::Visible
    }
}
#[doc = "Field `UTICK_IRQ` writer - Micro Tick Timer interrupt."]
pub type UtickIrqW<'a, REG> = crate::BitWriter<'a, REG, UtickIrq>;
impl<'a, REG> UtickIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(UtickIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(UtickIrq::Visible)
    }
}
#[doc = "Multi-Rate Timer interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MrtIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<MrtIrq> for bool {
    #[inline(always)]
    fn from(variant: MrtIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT_IRQ` reader - Multi-Rate Timer interrupt."]
pub type MrtIrqR = crate::BitReader<MrtIrq>;
impl MrtIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MrtIrq {
        match self.bits {
            false => MrtIrq::Invisible,
            true => MrtIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == MrtIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == MrtIrq::Visible
    }
}
#[doc = "Field `MRT_IRQ` writer - Multi-Rate Timer interrupt."]
pub type MrtIrqW<'a, REG> = crate::BitWriter<'a, REG, MrtIrq>;
impl<'a, REG> MrtIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(MrtIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(MrtIrq::Visible)
    }
}
#[doc = "Standard counter/timer 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctimer0Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Ctimer0Irq> for bool {
    #[inline(always)]
    fn from(variant: Ctimer0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER0_IRQ` reader - Standard counter/timer 0 interrupt."]
pub type Ctimer0IrqR = crate::BitReader<Ctimer0Irq>;
impl Ctimer0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer0Irq {
        match self.bits {
            false => Ctimer0Irq::Invisible,
            true => Ctimer0Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Ctimer0Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Ctimer0Irq::Visible
    }
}
#[doc = "Field `CTIMER0_IRQ` writer - Standard counter/timer 0 interrupt."]
pub type Ctimer0IrqW<'a, REG> = crate::BitWriter<'a, REG, Ctimer0Irq>;
impl<'a, REG> Ctimer0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer0Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer0Irq::Visible)
    }
}
#[doc = "Standard counter/timer 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctimer1Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Ctimer1Irq> for bool {
    #[inline(always)]
    fn from(variant: Ctimer1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER1_IRQ` reader - Standard counter/timer 1 interrupt."]
pub type Ctimer1IrqR = crate::BitReader<Ctimer1Irq>;
impl Ctimer1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer1Irq {
        match self.bits {
            false => Ctimer1Irq::Invisible,
            true => Ctimer1Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Ctimer1Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Ctimer1Irq::Visible
    }
}
#[doc = "Field `CTIMER1_IRQ` writer - Standard counter/timer 1 interrupt."]
pub type Ctimer1IrqW<'a, REG> = crate::BitWriter<'a, REG, Ctimer1Irq>;
impl<'a, REG> Ctimer1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer1Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer1Irq::Visible)
    }
}
#[doc = "SCTimer/PWM interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SctIrq> for bool {
    #[inline(always)]
    fn from(variant: SctIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT_IRQ` reader - SCTimer/PWM interrupt."]
pub type SctIrqR = crate::BitReader<SctIrq>;
impl SctIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctIrq {
        match self.bits {
            false => SctIrq::Invisible,
            true => SctIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SctIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SctIrq::Visible
    }
}
#[doc = "Field `SCT_IRQ` writer - SCTimer/PWM interrupt."]
pub type SctIrqW<'a, REG> = crate::BitWriter<'a, REG, SctIrq>;
impl<'a, REG> SctIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SctIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SctIrq::Visible)
    }
}
#[doc = "Standard counter/timer 3 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctimer3Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Ctimer3Irq> for bool {
    #[inline(always)]
    fn from(variant: Ctimer3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER3_IRQ` reader - Standard counter/timer 3 interrupt."]
pub type Ctimer3IrqR = crate::BitReader<Ctimer3Irq>;
impl Ctimer3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer3Irq {
        match self.bits {
            false => Ctimer3Irq::Invisible,
            true => Ctimer3Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Ctimer3Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Ctimer3Irq::Visible
    }
}
#[doc = "Field `CTIMER3_IRQ` writer - Standard counter/timer 3 interrupt."]
pub type Ctimer3IrqW<'a, REG> = crate::BitWriter<'a, REG, Ctimer3Irq>;
impl<'a, REG> Ctimer3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer3Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer3Irq::Visible)
    }
}
#[doc = "Flexcomm 0 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm0Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm0Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM0_IRQ` reader - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm0IrqR = crate::BitReader<Flexcomm0Irq>;
impl Flexcomm0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm0Irq {
        match self.bits {
            false => Flexcomm0Irq::Invisible,
            true => Flexcomm0Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm0Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm0Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM0_IRQ` writer - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm0IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm0Irq>;
impl<'a, REG> Flexcomm0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm0Irq::Visible)
    }
}
#[doc = "Flexcomm 1 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm1Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm1Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM1_IRQ` reader - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm1IrqR = crate::BitReader<Flexcomm1Irq>;
impl Flexcomm1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm1Irq {
        match self.bits {
            false => Flexcomm1Irq::Invisible,
            true => Flexcomm1Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm1Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm1Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM1_IRQ` writer - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm1IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm1Irq>;
impl<'a, REG> Flexcomm1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm1Irq::Visible)
    }
}
#[doc = "Flexcomm 2 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm2Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm2Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM2_IRQ` reader - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm2IrqR = crate::BitReader<Flexcomm2Irq>;
impl Flexcomm2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm2Irq {
        match self.bits {
            false => Flexcomm2Irq::Invisible,
            true => Flexcomm2Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm2Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm2Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM2_IRQ` writer - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm2IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm2Irq>;
impl<'a, REG> Flexcomm2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Irq::Visible)
    }
}
#[doc = "Flexcomm 3 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm3Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm3Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm3Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM3_IRQ` reader - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm3IrqR = crate::BitReader<Flexcomm3Irq>;
impl Flexcomm3IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm3Irq {
        match self.bits {
            false => Flexcomm3Irq::Invisible,
            true => Flexcomm3Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm3Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm3Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM3_IRQ` writer - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm3IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm3Irq>;
impl<'a, REG> Flexcomm3IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Irq::Visible)
    }
}
#[doc = "Flexcomm 4 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm4Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm4Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM4_IRQ` reader - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm4IrqR = crate::BitReader<Flexcomm4Irq>;
impl Flexcomm4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm4Irq {
        match self.bits {
            false => Flexcomm4Irq::Invisible,
            true => Flexcomm4Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm4Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm4Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM4_IRQ` writer - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm4IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm4Irq>;
impl<'a, REG> Flexcomm4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Irq::Visible)
    }
}
#[doc = "Flexcomm 5 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm5Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm5Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm5Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM5_IRQ` reader - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm5IrqR = crate::BitReader<Flexcomm5Irq>;
impl Flexcomm5IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm5Irq {
        match self.bits {
            false => Flexcomm5Irq::Invisible,
            true => Flexcomm5Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm5Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm5Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM5_IRQ` writer - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm5IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm5Irq>;
impl<'a, REG> Flexcomm5IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm5Irq::Visible)
    }
}
#[doc = "Flexcomm 6 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm6Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm6Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm6Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM6_IRQ` reader - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm6IrqR = crate::BitReader<Flexcomm6Irq>;
impl Flexcomm6IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm6Irq {
        match self.bits {
            false => Flexcomm6Irq::Invisible,
            true => Flexcomm6Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm6Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm6Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM6_IRQ` writer - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm6IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm6Irq>;
impl<'a, REG> Flexcomm6IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm6Irq::Visible)
    }
}
#[doc = "Flexcomm 7 interrupt (USART, SPI, I2C, I2S).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexcomm7Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Flexcomm7Irq> for bool {
    #[inline(always)]
    fn from(variant: Flexcomm7Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXCOMM7_IRQ` reader - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm7IrqR = crate::BitReader<Flexcomm7Irq>;
impl Flexcomm7IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm7Irq {
        match self.bits {
            false => Flexcomm7Irq::Invisible,
            true => Flexcomm7Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Flexcomm7Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Flexcomm7Irq::Visible
    }
}
#[doc = "Field `FLEXCOMM7_IRQ` writer - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
pub type Flexcomm7IrqW<'a, REG> = crate::BitWriter<'a, REG, Flexcomm7Irq>;
impl<'a, REG> Flexcomm7IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm7Irq::Visible)
    }
}
#[doc = "General Purpose ADC interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<AdcIrq> for bool {
    #[inline(always)]
    fn from(variant: AdcIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_IRQ` reader - General Purpose ADC interrupt."]
pub type AdcIrqR = crate::BitReader<AdcIrq>;
impl AdcIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcIrq {
        match self.bits {
            false => AdcIrq::Invisible,
            true => AdcIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == AdcIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == AdcIrq::Visible
    }
}
#[doc = "Field `ADC_IRQ` writer - General Purpose ADC interrupt."]
pub type AdcIrqW<'a, REG> = crate::BitWriter<'a, REG, AdcIrq>;
impl<'a, REG> AdcIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(AdcIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(AdcIrq::Visible)
    }
}
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved0 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Reserved0> for bool {
    #[inline(always)]
    fn from(variant: Reserved0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED0` reader - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved0R = crate::BitReader<Reserved0>;
impl Reserved0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved0 {
        match self.bits {
            false => Reserved0::Invisible,
            true => Reserved0::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Reserved0::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Reserved0::Visible
    }
}
#[doc = "Field `RESERVED0` writer - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG, Reserved0>;
impl<'a, REG> Reserved0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved0::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved0::Visible)
    }
}
#[doc = "Analog Comparator interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmpIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<AcmpIrq> for bool {
    #[inline(always)]
    fn from(variant: AcmpIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_IRQ` reader - Analog Comparator interrupt."]
pub type AcmpIrqR = crate::BitReader<AcmpIrq>;
impl AcmpIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AcmpIrq {
        match self.bits {
            false => AcmpIrq::Invisible,
            true => AcmpIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == AcmpIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == AcmpIrq::Visible
    }
}
#[doc = "Field `ACMP_IRQ` writer - Analog Comparator interrupt."]
pub type AcmpIrqW<'a, REG> = crate::BitWriter<'a, REG, AcmpIrq>;
impl<'a, REG> AcmpIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpIrq::Visible)
    }
}
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved1 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Reserved1> for bool {
    #[inline(always)]
    fn from(variant: Reserved1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED1` reader - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved1R = crate::BitReader<Reserved1>;
impl Reserved1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved1 {
        match self.bits {
            false => Reserved1::Invisible,
            true => Reserved1::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Reserved1::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Reserved1::Visible
    }
}
#[doc = "Field `RESERVED1` writer - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG, Reserved1>;
impl<'a, REG> Reserved1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved1::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved1::Visible)
    }
}
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved2 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Reserved2> for bool {
    #[inline(always)]
    fn from(variant: Reserved2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED2` reader - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved2R = crate::BitReader<Reserved2>;
impl Reserved2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved2 {
        match self.bits {
            false => Reserved2::Invisible,
            true => Reserved2::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Reserved2::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Reserved2::Visible
    }
}
#[doc = "Field `RESERVED2` writer - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG, Reserved2>;
impl<'a, REG> Reserved2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved2::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved2::Visible)
    }
}
#[doc = "USB Full Speed Controller Clock request interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0Needclk {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Usb0Needclk> for bool {
    #[inline(always)]
    fn from(variant: Usb0Needclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_NEEDCLK` reader - USB Full Speed Controller Clock request interrupt."]
pub type Usb0NeedclkR = crate::BitReader<Usb0Needclk>;
impl Usb0NeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0Needclk {
        match self.bits {
            false => Usb0Needclk::Invisible,
            true => Usb0Needclk::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Usb0Needclk::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Usb0Needclk::Visible
    }
}
#[doc = "Field `USB0_NEEDCLK` writer - USB Full Speed Controller Clock request interrupt."]
pub type Usb0NeedclkW<'a, REG> = crate::BitWriter<'a, REG, Usb0Needclk>;
impl<'a, REG> Usb0NeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Needclk::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Needclk::Visible)
    }
}
#[doc = "USB Full Speed Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb0Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Usb0Irq> for bool {
    #[inline(always)]
    fn from(variant: Usb0Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB0_IRQ` reader - USB Full Speed Controller interrupt."]
pub type Usb0IrqR = crate::BitReader<Usb0Irq>;
impl Usb0IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb0Irq {
        match self.bits {
            false => Usb0Irq::Invisible,
            true => Usb0Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Usb0Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Usb0Irq::Visible
    }
}
#[doc = "Field `USB0_IRQ` writer - USB Full Speed Controller interrupt."]
pub type Usb0IrqW<'a, REG> = crate::BitWriter<'a, REG, Usb0Irq>;
impl<'a, REG> Usb0IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb0Irq::Visible)
    }
}
#[doc = "RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<RtcIrq> for bool {
    #[inline(always)]
    fn from(variant: RtcIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_IRQ` reader - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
pub type RtcIrqR = crate::BitReader<RtcIrq>;
impl RtcIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcIrq {
        match self.bits {
            false => RtcIrq::Invisible,
            true => RtcIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == RtcIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == RtcIrq::Visible
    }
}
#[doc = "Field `RTC_IRQ` writer - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
pub type RtcIrqW<'a, REG> = crate::BitWriter<'a, REG, RtcIrq>;
impl<'a, REG> RtcIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(RtcIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(RtcIrq::Visible)
    }
}
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved3 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Reserved3> for bool {
    #[inline(always)]
    fn from(variant: Reserved3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED3` reader - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved3R = crate::BitReader<Reserved3>;
impl Reserved3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved3 {
        match self.bits {
            false => Reserved3::Invisible,
            true => Reserved3::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Reserved3::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Reserved3::Visible
    }
}
#[doc = "Field `RESERVED3` writer - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG, Reserved3>;
impl<'a, REG> Reserved3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved3::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved3::Visible)
    }
}
#[doc = "Mailbox interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MailboxIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<MailboxIrq> for bool {
    #[inline(always)]
    fn from(variant: MailboxIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAILBOX_IRQ` reader - Mailbox interrupt."]
pub type MailboxIrqR = crate::BitReader<MailboxIrq>;
impl MailboxIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MailboxIrq {
        match self.bits {
            false => MailboxIrq::Invisible,
            true => MailboxIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == MailboxIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == MailboxIrq::Visible
    }
}
#[doc = "Field `MAILBOX_IRQ` writer - Mailbox interrupt."]
pub type MailboxIrqW<'a, REG> = crate::BitWriter<'a, REG, MailboxIrq>;
impl<'a, REG> MailboxIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxIrq::Visible)
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    pub fn sys_irq(&self) -> SysIrqR {
        SysIrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub fn sdma0_irq(&self) -> Sdma0IrqR {
        Sdma0IrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint0_irq(&self) -> GpioGlobalint0IrqR {
        GpioGlobalint0IrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint1_irq(&self) -> GpioGlobalint1IrqR {
        GpioGlobalint1IrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq0(&self) -> GpioInt0Irq0R {
        GpioInt0Irq0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq1(&self) -> GpioInt0Irq1R {
        GpioInt0Irq1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq2(&self) -> GpioInt0Irq2R {
        GpioInt0Irq2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq3(&self) -> GpioInt0Irq3R {
        GpioInt0Irq3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline(always)]
    pub fn utick_irq(&self) -> UtickIrqR {
        UtickIrqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub fn mrt_irq(&self) -> MrtIrqR {
        MrtIrqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub fn ctimer0_irq(&self) -> Ctimer0IrqR {
        Ctimer0IrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub fn ctimer1_irq(&self) -> Ctimer1IrqR {
        Ctimer1IrqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline(always)]
    pub fn sct_irq(&self) -> SctIrqR {
        SctIrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub fn ctimer3_irq(&self) -> Ctimer3IrqR {
        Ctimer3IrqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm0_irq(&self) -> Flexcomm0IrqR {
        Flexcomm0IrqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm1_irq(&self) -> Flexcomm1IrqR {
        Flexcomm1IrqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm2_irq(&self) -> Flexcomm2IrqR {
        Flexcomm2IrqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm3_irq(&self) -> Flexcomm3IrqR {
        Flexcomm3IrqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm4_irq(&self) -> Flexcomm4IrqR {
        Flexcomm4IrqR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm5_irq(&self) -> Flexcomm5IrqR {
        Flexcomm5IrqR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm6_irq(&self) -> Flexcomm6IrqR {
        Flexcomm6IrqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm7_irq(&self) -> Flexcomm7IrqR {
        Flexcomm7IrqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline(always)]
    pub fn adc_irq(&self) -> AdcIrqR {
        AdcIrqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline(always)]
    pub fn acmp_irq(&self) -> AcmpIrqR {
        AcmpIrqR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> Usb0NeedclkR {
        Usb0NeedclkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb0_irq(&self) -> Usb0IrqR {
        Usb0IrqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    pub fn rtc_irq(&self) -> RtcIrqR {
        RtcIrqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline(always)]
    pub fn mailbox_irq(&self) -> MailboxIrqR {
        MailboxIrqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timer, Brown Out Detectors and Flash Controller interrupts"]
    #[inline(always)]
    pub fn sys_irq(&mut self) -> SysIrqW<SecCpuIntMask0Spec> {
        SysIrqW::new(self, 0)
    }
    #[doc = "Bit 1 - System DMA 0 (non-secure) interrupt."]
    #[inline(always)]
    pub fn sdma0_irq(&mut self) -> Sdma0IrqW<SecCpuIntMask0Spec> {
        Sdma0IrqW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO Group 0 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint0_irq(&mut self) -> GpioGlobalint0IrqW<SecCpuIntMask0Spec> {
        GpioGlobalint0IrqW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO Group 1 interrupt."]
    #[inline(always)]
    pub fn gpio_globalint1_irq(&mut self) -> GpioGlobalint1IrqW<SecCpuIntMask0Spec> {
        GpioGlobalint1IrqW::new(self, 3)
    }
    #[doc = "Bit 4 - Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq0(&mut self) -> GpioInt0Irq0W<SecCpuIntMask0Spec> {
        GpioInt0Irq0W::new(self, 4)
    }
    #[doc = "Bit 5 - Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq1(&mut self) -> GpioInt0Irq1W<SecCpuIntMask0Spec> {
        GpioInt0Irq1W::new(self, 5)
    }
    #[doc = "Bit 6 - Pin interrupt 2 or pattern match engine slice 2 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq2(&mut self) -> GpioInt0Irq2W<SecCpuIntMask0Spec> {
        GpioInt0Irq2W::new(self, 6)
    }
    #[doc = "Bit 7 - Pin interrupt 3 or pattern match engine slice 3 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq3(&mut self) -> GpioInt0Irq3W<SecCpuIntMask0Spec> {
        GpioInt0Irq3W::new(self, 7)
    }
    #[doc = "Bit 8 - Micro Tick Timer interrupt."]
    #[inline(always)]
    pub fn utick_irq(&mut self) -> UtickIrqW<SecCpuIntMask0Spec> {
        UtickIrqW::new(self, 8)
    }
    #[doc = "Bit 9 - Multi-Rate Timer interrupt."]
    #[inline(always)]
    pub fn mrt_irq(&mut self) -> MrtIrqW<SecCpuIntMask0Spec> {
        MrtIrqW::new(self, 9)
    }
    #[doc = "Bit 10 - Standard counter/timer 0 interrupt."]
    #[inline(always)]
    pub fn ctimer0_irq(&mut self) -> Ctimer0IrqW<SecCpuIntMask0Spec> {
        Ctimer0IrqW::new(self, 10)
    }
    #[doc = "Bit 11 - Standard counter/timer 1 interrupt."]
    #[inline(always)]
    pub fn ctimer1_irq(&mut self) -> Ctimer1IrqW<SecCpuIntMask0Spec> {
        Ctimer1IrqW::new(self, 11)
    }
    #[doc = "Bit 12 - SCTimer/PWM interrupt."]
    #[inline(always)]
    pub fn sct_irq(&mut self) -> SctIrqW<SecCpuIntMask0Spec> {
        SctIrqW::new(self, 12)
    }
    #[doc = "Bit 13 - Standard counter/timer 3 interrupt."]
    #[inline(always)]
    pub fn ctimer3_irq(&mut self) -> Ctimer3IrqW<SecCpuIntMask0Spec> {
        Ctimer3IrqW::new(self, 13)
    }
    #[doc = "Bit 14 - Flexcomm 0 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm0_irq(&mut self) -> Flexcomm0IrqW<SecCpuIntMask0Spec> {
        Flexcomm0IrqW::new(self, 14)
    }
    #[doc = "Bit 15 - Flexcomm 1 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm1_irq(&mut self) -> Flexcomm1IrqW<SecCpuIntMask0Spec> {
        Flexcomm1IrqW::new(self, 15)
    }
    #[doc = "Bit 16 - Flexcomm 2 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm2_irq(&mut self) -> Flexcomm2IrqW<SecCpuIntMask0Spec> {
        Flexcomm2IrqW::new(self, 16)
    }
    #[doc = "Bit 17 - Flexcomm 3 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm3_irq(&mut self) -> Flexcomm3IrqW<SecCpuIntMask0Spec> {
        Flexcomm3IrqW::new(self, 17)
    }
    #[doc = "Bit 18 - Flexcomm 4 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm4_irq(&mut self) -> Flexcomm4IrqW<SecCpuIntMask0Spec> {
        Flexcomm4IrqW::new(self, 18)
    }
    #[doc = "Bit 19 - Flexcomm 5 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm5_irq(&mut self) -> Flexcomm5IrqW<SecCpuIntMask0Spec> {
        Flexcomm5IrqW::new(self, 19)
    }
    #[doc = "Bit 20 - Flexcomm 6 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm6_irq(&mut self) -> Flexcomm6IrqW<SecCpuIntMask0Spec> {
        Flexcomm6IrqW::new(self, 20)
    }
    #[doc = "Bit 21 - Flexcomm 7 interrupt (USART, SPI, I2C, I2S)."]
    #[inline(always)]
    pub fn flexcomm7_irq(&mut self) -> Flexcomm7IrqW<SecCpuIntMask0Spec> {
        Flexcomm7IrqW::new(self, 21)
    }
    #[doc = "Bit 22 - General Purpose ADC interrupt."]
    #[inline(always)]
    pub fn adc_irq(&mut self) -> AdcIrqW<SecCpuIntMask0Spec> {
        AdcIrqW::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> Reserved0W<SecCpuIntMask0Spec> {
        Reserved0W::new(self, 23)
    }
    #[doc = "Bit 24 - Analog Comparator interrupt."]
    #[inline(always)]
    pub fn acmp_irq(&mut self) -> AcmpIrqW<SecCpuIntMask0Spec> {
        AcmpIrqW::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<SecCpuIntMask0Spec> {
        Reserved1W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<SecCpuIntMask0Spec> {
        Reserved2W::new(self, 26)
    }
    #[doc = "Bit 27 - USB Full Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> Usb0NeedclkW<SecCpuIntMask0Spec> {
        Usb0NeedclkW::new(self, 27)
    }
    #[doc = "Bit 28 - USB Full Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb0_irq(&mut self) -> Usb0IrqW<SecCpuIntMask0Spec> {
        Usb0IrqW::new(self, 28)
    }
    #[doc = "Bit 29 - RTC_LITE0_ALARM_IRQ, RTC_LITE0_WAKEUP_IRQ"]
    #[inline(always)]
    pub fn rtc_irq(&mut self) -> RtcIrqW<SecCpuIntMask0Spec> {
        RtcIrqW::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<SecCpuIntMask0Spec> {
        Reserved3W::new(self, 30)
    }
    #[doc = "Bit 31 - Mailbox interrupt."]
    #[inline(always)]
    pub fn mailbox_irq(&mut self) -> MailboxIrqW<SecCpuIntMask0Spec> {
        MailboxIrqW::new(self, 31)
    }
}
#[doc = "Secure Interrupt mask for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_cpu_int_mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_cpu_int_mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCpuIntMask0Spec;
impl crate::RegisterSpec for SecCpuIntMask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_cpu_int_mask0::R`](R) reader structure"]
impl crate::Readable for SecCpuIntMask0Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_cpu_int_mask0::W`](W) writer structure"]
impl crate::Writable for SecCpuIntMask0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CPU_INT_MASK0 to value 0xffff_ffff"]
impl crate::Resettable for SecCpuIntMask0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
