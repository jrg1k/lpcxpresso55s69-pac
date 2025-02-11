#[doc = "Register `SEC_CPU_INT_MASK1` reader"]
pub type R = crate::R<SecCpuIntMask1Spec>;
#[doc = "Register `SEC_CPU_INT_MASK1` writer"]
pub type W = crate::W<SecCpuIntMask1Spec>;
#[doc = "Pin interrupt 4 or pattern match engine slice 4 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq4 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq4> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` reader - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
pub type GpioInt0Irq4R = crate::BitReader<GpioInt0Irq4>;
impl GpioInt0Irq4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq4 {
        match self.bits {
            false => GpioInt0Irq4::Invisible,
            true => GpioInt0Irq4::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq4::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq4::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ4` writer - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
pub type GpioInt0Irq4W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq4>;
impl<'a, REG> GpioInt0Irq4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq4::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq4::Visible)
    }
}
#[doc = "Pin interrupt 5 or pattern match engine slice 5 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq5 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq5> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` reader - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
pub type GpioInt0Irq5R = crate::BitReader<GpioInt0Irq5>;
impl GpioInt0Irq5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq5 {
        match self.bits {
            false => GpioInt0Irq5::Invisible,
            true => GpioInt0Irq5::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq5::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq5::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ5` writer - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
pub type GpioInt0Irq5W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq5>;
impl<'a, REG> GpioInt0Irq5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq5::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq5::Visible)
    }
}
#[doc = "Pin interrupt 6 or pattern match engine slice 6 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq6 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq6> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` reader - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
pub type GpioInt0Irq6R = crate::BitReader<GpioInt0Irq6>;
impl GpioInt0Irq6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq6 {
        match self.bits {
            false => GpioInt0Irq6::Invisible,
            true => GpioInt0Irq6::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq6::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq6::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ6` writer - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
pub type GpioInt0Irq6W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq6>;
impl<'a, REG> GpioInt0Irq6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq6::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq6::Visible)
    }
}
#[doc = "Pin interrupt 7 or pattern match engine slice 7 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioInt0Irq7 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<GpioInt0Irq7> for bool {
    #[inline(always)]
    fn from(variant: GpioInt0Irq7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` reader - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
pub type GpioInt0Irq7R = crate::BitReader<GpioInt0Irq7>;
impl GpioInt0Irq7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioInt0Irq7 {
        match self.bits {
            false => GpioInt0Irq7::Invisible,
            true => GpioInt0Irq7::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == GpioInt0Irq7::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == GpioInt0Irq7::Visible
    }
}
#[doc = "Field `GPIO_INT0_IRQ7` writer - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
pub type GpioInt0Irq7W<'a, REG> = crate::BitWriter<'a, REG, GpioInt0Irq7>;
impl<'a, REG> GpioInt0Irq7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq7::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInt0Irq7::Visible)
    }
}
#[doc = "Standard counter/timer 2 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctimer2Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Ctimer2Irq> for bool {
    #[inline(always)]
    fn from(variant: Ctimer2Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER2_IRQ` reader - Standard counter/timer 2 interrupt."]
pub type Ctimer2IrqR = crate::BitReader<Ctimer2Irq>;
impl Ctimer2IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer2Irq {
        match self.bits {
            false => Ctimer2Irq::Invisible,
            true => Ctimer2Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Ctimer2Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Ctimer2Irq::Visible
    }
}
#[doc = "Field `CTIMER2_IRQ` writer - Standard counter/timer 2 interrupt."]
pub type Ctimer2IrqW<'a, REG> = crate::BitWriter<'a, REG, Ctimer2Irq>;
impl<'a, REG> Ctimer2IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer2Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer2Irq::Visible)
    }
}
#[doc = "Standard counter/timer 4 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctimer4Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Ctimer4Irq> for bool {
    #[inline(always)]
    fn from(variant: Ctimer4Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER4_IRQ` reader - Standard counter/timer 4 interrupt."]
pub type Ctimer4IrqR = crate::BitReader<Ctimer4Irq>;
impl Ctimer4IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctimer4Irq {
        match self.bits {
            false => Ctimer4Irq::Invisible,
            true => Ctimer4Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Ctimer4Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Ctimer4Irq::Visible
    }
}
#[doc = "Field `CTIMER4_IRQ` writer - Standard counter/timer 4 interrupt."]
pub type Ctimer4IrqW<'a, REG> = crate::BitWriter<'a, REG, Ctimer4Irq>;
impl<'a, REG> Ctimer4IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer4Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Ctimer4Irq::Visible)
    }
}
#[doc = "OS Event Timer and OS Event Timer Wakeup interrupts\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsEventTimerIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<OsEventTimerIrq> for bool {
    #[inline(always)]
    fn from(variant: OsEventTimerIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OS_EVENT_TIMER_IRQ` reader - OS Event Timer and OS Event Timer Wakeup interrupts"]
pub type OsEventTimerIrqR = crate::BitReader<OsEventTimerIrq>;
impl OsEventTimerIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OsEventTimerIrq {
        match self.bits {
            false => OsEventTimerIrq::Invisible,
            true => OsEventTimerIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == OsEventTimerIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == OsEventTimerIrq::Visible
    }
}
#[doc = "Field `OS_EVENT_TIMER_IRQ` writer - OS Event Timer and OS Event Timer Wakeup interrupts"]
pub type OsEventTimerIrqW<'a, REG> = crate::BitWriter<'a, REG, OsEventTimerIrq>;
impl<'a, REG> OsEventTimerIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(OsEventTimerIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(OsEventTimerIrq::Visible)
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
#[doc = "SDIO Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SdioIrq> for bool {
    #[inline(always)]
    fn from(variant: SdioIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_IRQ` reader - SDIO Controller interrupt."]
pub type SdioIrqR = crate::BitReader<SdioIrq>;
impl SdioIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdioIrq {
        match self.bits {
            false => SdioIrq::Invisible,
            true => SdioIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SdioIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SdioIrq::Visible
    }
}
#[doc = "Field `SDIO_IRQ` writer - SDIO Controller interrupt."]
pub type SdioIrqW<'a, REG> = crate::BitWriter<'a, REG, SdioIrq>;
impl<'a, REG> SdioIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SdioIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SdioIrq::Visible)
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
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved4 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Reserved4> for bool {
    #[inline(always)]
    fn from(variant: Reserved4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED4` reader - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved4R = crate::BitReader<Reserved4>;
impl Reserved4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved4 {
        match self.bits {
            false => Reserved4::Invisible,
            true => Reserved4::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Reserved4::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Reserved4::Visible
    }
}
#[doc = "Field `RESERVED4` writer - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG, Reserved4>;
impl<'a, REG> Reserved4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved4::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved4::Visible)
    }
}
#[doc = "Reserved. Read value is undefined, only zero should be written.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reserved5 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Reserved5> for bool {
    #[inline(always)]
    fn from(variant: Reserved5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESERVED5` reader - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved5R = crate::BitReader<Reserved5>;
impl Reserved5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reserved5 {
        match self.bits {
            false => Reserved5::Invisible,
            true => Reserved5::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Reserved5::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Reserved5::Visible
    }
}
#[doc = "Field `RESERVED5` writer - Reserved. Read value is undefined, only zero should be written."]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG, Reserved5>;
impl<'a, REG> Reserved5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved5::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Reserved5::Visible)
    }
}
#[doc = "USB High Speed PHY Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1PhyIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Usb1PhyIrq> for bool {
    #[inline(always)]
    fn from(variant: Usb1PhyIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_PHY_IRQ` reader - USB High Speed PHY Controller interrupt."]
pub type Usb1PhyIrqR = crate::BitReader<Usb1PhyIrq>;
impl Usb1PhyIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1PhyIrq {
        match self.bits {
            false => Usb1PhyIrq::Invisible,
            true => Usb1PhyIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Usb1PhyIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Usb1PhyIrq::Visible
    }
}
#[doc = "Field `USB1_PHY_IRQ` writer - USB High Speed PHY Controller interrupt."]
pub type Usb1PhyIrqW<'a, REG> = crate::BitWriter<'a, REG, Usb1PhyIrq>;
impl<'a, REG> Usb1PhyIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1PhyIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1PhyIrq::Visible)
    }
}
#[doc = "USB High Speed Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Usb1Irq> for bool {
    #[inline(always)]
    fn from(variant: Usb1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_IRQ` reader - USB High Speed Controller interrupt."]
pub type Usb1IrqR = crate::BitReader<Usb1Irq>;
impl Usb1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1Irq {
        match self.bits {
            false => Usb1Irq::Invisible,
            true => Usb1Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Usb1Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Usb1Irq::Visible
    }
}
#[doc = "Field `USB1_IRQ` writer - USB High Speed Controller interrupt."]
pub type Usb1IrqW<'a, REG> = crate::BitWriter<'a, REG, Usb1Irq>;
impl<'a, REG> Usb1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Irq::Visible)
    }
}
#[doc = "USB High Speed Controller Clock request interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb1Needclk {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Usb1Needclk> for bool {
    #[inline(always)]
    fn from(variant: Usb1Needclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB1_NEEDCLK` reader - USB High Speed Controller Clock request interrupt."]
pub type Usb1NeedclkR = crate::BitReader<Usb1Needclk>;
impl Usb1NeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb1Needclk {
        match self.bits {
            false => Usb1Needclk::Invisible,
            true => Usb1Needclk::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Usb1Needclk::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Usb1Needclk::Visible
    }
}
#[doc = "Field `USB1_NEEDCLK` writer - USB High Speed Controller Clock request interrupt."]
pub type Usb1NeedclkW<'a, REG> = crate::BitWriter<'a, REG, Usb1Needclk>;
impl<'a, REG> Usb1NeedclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Needclk::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Usb1Needclk::Visible)
    }
}
#[doc = "Secure fault Hyper Visor call interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecHypervisorCallIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SecHypervisorCallIrq> for bool {
    #[inline(always)]
    fn from(variant: SecHypervisorCallIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_HYPERVISOR_CALL_IRQ` reader - Secure fault Hyper Visor call interrupt."]
pub type SecHypervisorCallIrqR = crate::BitReader<SecHypervisorCallIrq>;
impl SecHypervisorCallIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecHypervisorCallIrq {
        match self.bits {
            false => SecHypervisorCallIrq::Invisible,
            true => SecHypervisorCallIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SecHypervisorCallIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SecHypervisorCallIrq::Visible
    }
}
#[doc = "Field `SEC_HYPERVISOR_CALL_IRQ` writer - Secure fault Hyper Visor call interrupt."]
pub type SecHypervisorCallIrqW<'a, REG> = crate::BitWriter<'a, REG, SecHypervisorCallIrq>;
impl<'a, REG> SecHypervisorCallIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SecHypervisorCallIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SecHypervisorCallIrq::Visible)
    }
}
#[doc = "Secure Pin interrupt 0 or pattern match engine slice 0 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecGpioInt0Irq0 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SecGpioInt0Irq0> for bool {
    #[inline(always)]
    fn from(variant: SecGpioInt0Irq0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ0` reader - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type SecGpioInt0Irq0R = crate::BitReader<SecGpioInt0Irq0>;
impl SecGpioInt0Irq0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecGpioInt0Irq0 {
        match self.bits {
            false => SecGpioInt0Irq0::Invisible,
            true => SecGpioInt0Irq0::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SecGpioInt0Irq0::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SecGpioInt0Irq0::Visible
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ0` writer - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
pub type SecGpioInt0Irq0W<'a, REG> = crate::BitWriter<'a, REG, SecGpioInt0Irq0>;
impl<'a, REG> SecGpioInt0Irq0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioInt0Irq0::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioInt0Irq0::Visible)
    }
}
#[doc = "Secure Pin interrupt 1 or pattern match engine slice 1 interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecGpioInt0Irq1 {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SecGpioInt0Irq1> for bool {
    #[inline(always)]
    fn from(variant: SecGpioInt0Irq1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ1` reader - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type SecGpioInt0Irq1R = crate::BitReader<SecGpioInt0Irq1>;
impl SecGpioInt0Irq1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecGpioInt0Irq1 {
        match self.bits {
            false => SecGpioInt0Irq1::Invisible,
            true => SecGpioInt0Irq1::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SecGpioInt0Irq1::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SecGpioInt0Irq1::Visible
    }
}
#[doc = "Field `SEC_GPIO_INT0_IRQ1` writer - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
pub type SecGpioInt0Irq1W<'a, REG> = crate::BitWriter<'a, REG, SecGpioInt0Irq1>;
impl<'a, REG> SecGpioInt0Irq1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioInt0Irq1::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioInt0Irq1::Visible)
    }
}
#[doc = "Programmable Look-Up Controller interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PluIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<PluIrq> for bool {
    #[inline(always)]
    fn from(variant: PluIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLU_IRQ` reader - Programmable Look-Up Controller interrupt."]
pub type PluIrqR = crate::BitReader<PluIrq>;
impl PluIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PluIrq {
        match self.bits {
            false => PluIrq::Invisible,
            true => PluIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PluIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PluIrq::Visible
    }
}
#[doc = "Field `PLU_IRQ` writer - Programmable Look-Up Controller interrupt."]
pub type PluIrqW<'a, REG> = crate::BitWriter<'a, REG, PluIrq>;
impl<'a, REG> PluIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(PluIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(PluIrq::Visible)
    }
}
#[doc = "Security Violation interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecVioIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<SecVioIrq> for bool {
    #[inline(always)]
    fn from(variant: SecVioIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEC_VIO_IRQ` reader - Security Violation interrupt."]
pub type SecVioIrqR = crate::BitReader<SecVioIrq>;
impl SecVioIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecVioIrq {
        match self.bits {
            false => SecVioIrq::Invisible,
            true => SecVioIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == SecVioIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == SecVioIrq::Visible
    }
}
#[doc = "Field `SEC_VIO_IRQ` writer - Security Violation interrupt."]
pub type SecVioIrqW<'a, REG> = crate::BitWriter<'a, REG, SecVioIrq>;
impl<'a, REG> SecVioIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(SecVioIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(SecVioIrq::Visible)
    }
}
#[doc = "HASH-AES interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<ShaIrq> for bool {
    #[inline(always)]
    fn from(variant: ShaIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHA_IRQ` reader - HASH-AES interrupt."]
pub type ShaIrqR = crate::BitReader<ShaIrq>;
impl ShaIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShaIrq {
        match self.bits {
            false => ShaIrq::Invisible,
            true => ShaIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == ShaIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == ShaIrq::Visible
    }
}
#[doc = "Field `SHA_IRQ` writer - HASH-AES interrupt."]
pub type ShaIrqW<'a, REG> = crate::BitWriter<'a, REG, ShaIrq>;
impl<'a, REG> ShaIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(ShaIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(ShaIrq::Visible)
    }
}
#[doc = "CASPER interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasperIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<CasperIrq> for bool {
    #[inline(always)]
    fn from(variant: CasperIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CASPER_IRQ` reader - CASPER interrupt."]
pub type CasperIrqR = crate::BitReader<CasperIrq>;
impl CasperIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CasperIrq {
        match self.bits {
            false => CasperIrq::Invisible,
            true => CasperIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == CasperIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == CasperIrq::Visible
    }
}
#[doc = "Field `CASPER_IRQ` writer - CASPER interrupt."]
pub type CasperIrqW<'a, REG> = crate::BitWriter<'a, REG, CasperIrq>;
impl<'a, REG> CasperIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(CasperIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(CasperIrq::Visible)
    }
}
#[doc = "PUF interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PufkeyIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<PufkeyIrq> for bool {
    #[inline(always)]
    fn from(variant: PufkeyIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUFKEY_IRQ` reader - PUF interrupt."]
pub type PufkeyIrqR = crate::BitReader<PufkeyIrq>;
impl PufkeyIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PufkeyIrq {
        match self.bits {
            false => PufkeyIrq::Invisible,
            true => PufkeyIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PufkeyIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PufkeyIrq::Visible
    }
}
#[doc = "Field `PUFKEY_IRQ` writer - PUF interrupt."]
pub type PufkeyIrqW<'a, REG> = crate::BitWriter<'a, REG, PufkeyIrq>;
impl<'a, REG> PufkeyIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(PufkeyIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(PufkeyIrq::Visible)
    }
}
#[doc = "Power Quad interrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PqIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<PqIrq> for bool {
    #[inline(always)]
    fn from(variant: PqIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PQ_IRQ` reader - Power Quad interrupt."]
pub type PqIrqR = crate::BitReader<PqIrq>;
impl PqIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PqIrq {
        match self.bits {
            false => PqIrq::Invisible,
            true => PqIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == PqIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == PqIrq::Visible
    }
}
#[doc = "Field `PQ_IRQ` writer - Power Quad interrupt."]
pub type PqIrqW<'a, REG> = crate::BitWriter<'a, REG, PqIrq>;
impl<'a, REG> PqIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(PqIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(PqIrq::Visible)
    }
}
#[doc = "System DMA 1 (Secure) interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdma1Irq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<Sdma1Irq> for bool {
    #[inline(always)]
    fn from(variant: Sdma1Irq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMA1_IRQ` reader - System DMA 1 (Secure) interrupt"]
pub type Sdma1IrqR = crate::BitReader<Sdma1Irq>;
impl Sdma1IrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma1Irq {
        match self.bits {
            false => Sdma1Irq::Invisible,
            true => Sdma1Irq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == Sdma1Irq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == Sdma1Irq::Visible
    }
}
#[doc = "Field `SDMA1_IRQ` writer - System DMA 1 (Secure) interrupt"]
pub type Sdma1IrqW<'a, REG> = crate::BitWriter<'a, REG, Sdma1Irq>;
impl<'a, REG> Sdma1IrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1Irq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1Irq::Visible)
    }
}
#[doc = "High Speed SPI interrupt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LspiHsIrq {
    #[doc = "0: no description available"]
    Invisible = 0,
    #[doc = "1: no description available"]
    Visible = 1,
}
impl From<LspiHsIrq> for bool {
    #[inline(always)]
    fn from(variant: LspiHsIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSPI_HS_IRQ` reader - High Speed SPI interrupt"]
pub type LspiHsIrqR = crate::BitReader<LspiHsIrq>;
impl LspiHsIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LspiHsIrq {
        match self.bits {
            false => LspiHsIrq::Invisible,
            true => LspiHsIrq::Visible,
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_invisible(&self) -> bool {
        *self == LspiHsIrq::Invisible
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn is_visible(&self) -> bool {
        *self == LspiHsIrq::Visible
    }
}
#[doc = "Field `LSPI_HS_IRQ` writer - High Speed SPI interrupt"]
pub type LspiHsIrqW<'a, REG> = crate::BitWriter<'a, REG, LspiHsIrq>;
impl<'a, REG> LspiHsIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no description available"]
    #[inline(always)]
    pub fn invisible(self) -> &'a mut crate::W<REG> {
        self.variant(LspiHsIrq::Invisible)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn visible(self) -> &'a mut crate::W<REG> {
        self.variant(LspiHsIrq::Visible)
    }
}
impl R {
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq4(&self) -> GpioInt0Irq4R {
        GpioInt0Irq4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq5(&self) -> GpioInt0Irq5R {
        GpioInt0Irq5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq6(&self) -> GpioInt0Irq6R {
        GpioInt0Irq6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq7(&self) -> GpioInt0Irq7R {
        GpioInt0Irq7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub fn ctimer2_irq(&self) -> Ctimer2IrqR {
        Ctimer2IrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub fn ctimer4_irq(&self) -> Ctimer4IrqR {
        Ctimer4IrqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    pub fn os_event_timer_irq(&self) -> OsEventTimerIrqR {
        OsEventTimerIrqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline(always)]
    pub fn sdio_irq(&self) -> SdioIrqR {
        SdioIrqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub fn usb1_phy_irq(&self) -> Usb1PhyIrqR {
        Usb1PhyIrqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb1_irq(&self) -> Usb1IrqR {
        Usb1IrqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb1_needclk(&self) -> Usb1NeedclkR {
        Usb1NeedclkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub fn sec_hypervisor_call_irq(&self) -> SecHypervisorCallIrqR {
        SecHypervisorCallIrqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq0(&self) -> SecGpioInt0Irq0R {
        SecGpioInt0Irq0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq1(&self) -> SecGpioInt0Irq1R {
        SecGpioInt0Irq1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub fn plu_irq(&self) -> PluIrqR {
        PluIrqR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline(always)]
    pub fn sec_vio_irq(&self) -> SecVioIrqR {
        SecVioIrqR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline(always)]
    pub fn sha_irq(&self) -> ShaIrqR {
        ShaIrqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline(always)]
    pub fn casper_irq(&self) -> CasperIrqR {
        CasperIrqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline(always)]
    pub fn pufkey_irq(&self) -> PufkeyIrqR {
        PufkeyIrqR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline(always)]
    pub fn pq_irq(&self) -> PqIrqR {
        PqIrqR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    pub fn sdma1_irq(&self) -> Sdma1IrqR {
        Sdma1IrqR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline(always)]
    pub fn lspi_hs_irq(&self) -> LspiHsIrqR {
        LspiHsIrqR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin interrupt 4 or pattern match engine slice 4 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq4(&mut self) -> GpioInt0Irq4W<SecCpuIntMask1Spec> {
        GpioInt0Irq4W::new(self, 0)
    }
    #[doc = "Bit 1 - Pin interrupt 5 or pattern match engine slice 5 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq5(&mut self) -> GpioInt0Irq5W<SecCpuIntMask1Spec> {
        GpioInt0Irq5W::new(self, 1)
    }
    #[doc = "Bit 2 - Pin interrupt 6 or pattern match engine slice 6 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq6(&mut self) -> GpioInt0Irq6W<SecCpuIntMask1Spec> {
        GpioInt0Irq6W::new(self, 2)
    }
    #[doc = "Bit 3 - Pin interrupt 7 or pattern match engine slice 7 interrupt."]
    #[inline(always)]
    pub fn gpio_int0_irq7(&mut self) -> GpioInt0Irq7W<SecCpuIntMask1Spec> {
        GpioInt0Irq7W::new(self, 3)
    }
    #[doc = "Bit 4 - Standard counter/timer 2 interrupt."]
    #[inline(always)]
    pub fn ctimer2_irq(&mut self) -> Ctimer2IrqW<SecCpuIntMask1Spec> {
        Ctimer2IrqW::new(self, 4)
    }
    #[doc = "Bit 5 - Standard counter/timer 4 interrupt."]
    #[inline(always)]
    pub fn ctimer4_irq(&mut self) -> Ctimer4IrqW<SecCpuIntMask1Spec> {
        Ctimer4IrqW::new(self, 5)
    }
    #[doc = "Bit 6 - OS Event Timer and OS Event Timer Wakeup interrupts"]
    #[inline(always)]
    pub fn os_event_timer_irq(&mut self) -> OsEventTimerIrqW<SecCpuIntMask1Spec> {
        OsEventTimerIrqW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> Reserved0W<SecCpuIntMask1Spec> {
        Reserved0W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<SecCpuIntMask1Spec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<SecCpuIntMask1Spec> {
        Reserved2W::new(self, 9)
    }
    #[doc = "Bit 10 - SDIO Controller interrupt."]
    #[inline(always)]
    pub fn sdio_irq(&mut self) -> SdioIrqW<SecCpuIntMask1Spec> {
        SdioIrqW::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<SecCpuIntMask1Spec> {
        Reserved3W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<SecCpuIntMask1Spec> {
        Reserved4W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved. Read value is undefined, only zero should be written."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<SecCpuIntMask1Spec> {
        Reserved5W::new(self, 13)
    }
    #[doc = "Bit 14 - USB High Speed PHY Controller interrupt."]
    #[inline(always)]
    pub fn usb1_phy_irq(&mut self) -> Usb1PhyIrqW<SecCpuIntMask1Spec> {
        Usb1PhyIrqW::new(self, 14)
    }
    #[doc = "Bit 15 - USB High Speed Controller interrupt."]
    #[inline(always)]
    pub fn usb1_irq(&mut self) -> Usb1IrqW<SecCpuIntMask1Spec> {
        Usb1IrqW::new(self, 15)
    }
    #[doc = "Bit 16 - USB High Speed Controller Clock request interrupt."]
    #[inline(always)]
    pub fn usb1_needclk(&mut self) -> Usb1NeedclkW<SecCpuIntMask1Spec> {
        Usb1NeedclkW::new(self, 16)
    }
    #[doc = "Bit 17 - Secure fault Hyper Visor call interrupt."]
    #[inline(always)]
    pub fn sec_hypervisor_call_irq(&mut self) -> SecHypervisorCallIrqW<SecCpuIntMask1Spec> {
        SecHypervisorCallIrqW::new(self, 17)
    }
    #[doc = "Bit 18 - Secure Pin interrupt 0 or pattern match engine slice 0 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq0(&mut self) -> SecGpioInt0Irq0W<SecCpuIntMask1Spec> {
        SecGpioInt0Irq0W::new(self, 18)
    }
    #[doc = "Bit 19 - Secure Pin interrupt 1 or pattern match engine slice 1 interrupt."]
    #[inline(always)]
    pub fn sec_gpio_int0_irq1(&mut self) -> SecGpioInt0Irq1W<SecCpuIntMask1Spec> {
        SecGpioInt0Irq1W::new(self, 19)
    }
    #[doc = "Bit 20 - Programmable Look-Up Controller interrupt."]
    #[inline(always)]
    pub fn plu_irq(&mut self) -> PluIrqW<SecCpuIntMask1Spec> {
        PluIrqW::new(self, 20)
    }
    #[doc = "Bit 21 - Security Violation interrupt."]
    #[inline(always)]
    pub fn sec_vio_irq(&mut self) -> SecVioIrqW<SecCpuIntMask1Spec> {
        SecVioIrqW::new(self, 21)
    }
    #[doc = "Bit 22 - HASH-AES interrupt."]
    #[inline(always)]
    pub fn sha_irq(&mut self) -> ShaIrqW<SecCpuIntMask1Spec> {
        ShaIrqW::new(self, 22)
    }
    #[doc = "Bit 23 - CASPER interrupt."]
    #[inline(always)]
    pub fn casper_irq(&mut self) -> CasperIrqW<SecCpuIntMask1Spec> {
        CasperIrqW::new(self, 23)
    }
    #[doc = "Bit 24 - PUF interrupt."]
    #[inline(always)]
    pub fn pufkey_irq(&mut self) -> PufkeyIrqW<SecCpuIntMask1Spec> {
        PufkeyIrqW::new(self, 24)
    }
    #[doc = "Bit 25 - Power Quad interrupt."]
    #[inline(always)]
    pub fn pq_irq(&mut self) -> PqIrqW<SecCpuIntMask1Spec> {
        PqIrqW::new(self, 25)
    }
    #[doc = "Bit 26 - System DMA 1 (Secure) interrupt"]
    #[inline(always)]
    pub fn sdma1_irq(&mut self) -> Sdma1IrqW<SecCpuIntMask1Spec> {
        Sdma1IrqW::new(self, 26)
    }
    #[doc = "Bit 27 - High Speed SPI interrupt"]
    #[inline(always)]
    pub fn lspi_hs_irq(&mut self) -> LspiHsIrqW<SecCpuIntMask1Spec> {
        LspiHsIrqW::new(self, 27)
    }
}
#[doc = "Secure Interrupt mask for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_cpu_int_mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_cpu_int_mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCpuIntMask1Spec;
impl crate::RegisterSpec for SecCpuIntMask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_cpu_int_mask1::R`](R) reader structure"]
impl crate::Readable for SecCpuIntMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`sec_cpu_int_mask1::W`](W) writer structure"]
impl crate::Writable for SecCpuIntMask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CPU_INT_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for SecCpuIntMask1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
