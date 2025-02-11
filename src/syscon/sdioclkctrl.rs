#[doc = "Register `SDIOCLKCTRL` reader"]
pub type R = crate::R<SdioclkctrlSpec>;
#[doc = "Register `SDIOCLKCTRL` writer"]
pub type W = crate::W<SdioclkctrlSpec>;
#[doc = "Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CclkDrvPhase {
    #[doc = "0: 0 degree shift."]
    Enum0Deg = 0,
    #[doc = "1: 90 degree shift."]
    Enum90Deg = 1,
    #[doc = "2: 180 degree shift."]
    Enum180Deg = 2,
    #[doc = "3: 270 degree shift."]
    Enum270Deg = 3,
}
impl From<CclkDrvPhase> for u8 {
    #[inline(always)]
    fn from(variant: CclkDrvPhase) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CclkDrvPhase {
    type Ux = u8;
}
impl crate::IsEnum for CclkDrvPhase {}
#[doc = "Field `CCLK_DRV_PHASE` reader - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub type CclkDrvPhaseR = crate::FieldReader<CclkDrvPhase>;
impl CclkDrvPhaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkDrvPhase {
        match self.bits {
            0 => CclkDrvPhase::Enum0Deg,
            1 => CclkDrvPhase::Enum90Deg,
            2 => CclkDrvPhase::Enum180Deg,
            3 => CclkDrvPhase::Enum270Deg,
            _ => unreachable!(),
        }
    }
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CclkDrvPhase::Enum0Deg
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CclkDrvPhase::Enum90Deg
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CclkDrvPhase::Enum180Deg
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CclkDrvPhase::Enum270Deg
    }
}
#[doc = "Field `CCLK_DRV_PHASE` writer - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub type CclkDrvPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, CclkDrvPhase, crate::Safe>;
impl<'a, REG> CclkDrvPhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkDrvPhase::Enum0Deg)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkDrvPhase::Enum90Deg)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkDrvPhase::Enum180Deg)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkDrvPhase::Enum270Deg)
    }
}
#[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CclkSamplePhase {
    #[doc = "0: 0 degree shift."]
    Enum0Deg = 0,
    #[doc = "1: 90 degree shift."]
    Enum90Deg = 1,
    #[doc = "2: 180 degree shift."]
    Enum180Deg = 2,
    #[doc = "3: 270 degree shift."]
    Enum270Deg = 3,
}
impl From<CclkSamplePhase> for u8 {
    #[inline(always)]
    fn from(variant: CclkSamplePhase) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CclkSamplePhase {
    type Ux = u8;
}
impl crate::IsEnum for CclkSamplePhase {}
#[doc = "Field `CCLK_SAMPLE_PHASE` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CclkSamplePhaseR = crate::FieldReader<CclkSamplePhase>;
impl CclkSamplePhaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkSamplePhase {
        match self.bits {
            0 => CclkSamplePhase::Enum0Deg,
            1 => CclkSamplePhase::Enum90Deg,
            2 => CclkSamplePhase::Enum180Deg,
            3 => CclkSamplePhase::Enum270Deg,
            _ => unreachable!(),
        }
    }
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CclkSamplePhase::Enum0Deg
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CclkSamplePhase::Enum90Deg
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CclkSamplePhase::Enum180Deg
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CclkSamplePhase::Enum270Deg
    }
}
#[doc = "Field `CCLK_SAMPLE_PHASE` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CclkSamplePhaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, CclkSamplePhase, crate::Safe>;
impl<'a, REG> CclkSamplePhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkSamplePhase::Enum0Deg)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkSamplePhase::Enum90Deg)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkSamplePhase::Enum180Deg)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut crate::W<REG> {
        self.variant(CclkSamplePhase::Enum270Deg)
    }
}
#[doc = "Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseActive {
    #[doc = "0: Bypassed."]
    Bypassed = 0,
    #[doc = "1: Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PhShift = 1,
}
impl From<PhaseActive> for bool {
    #[inline(always)]
    fn from(variant: PhaseActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHASE_ACTIVE` reader - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
pub type PhaseActiveR = crate::BitReader<PhaseActive>;
impl PhaseActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PhaseActive {
        match self.bits {
            false => PhaseActive::Bypassed,
            true => PhaseActive::PhShift,
        }
    }
    #[doc = "Bypassed."]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == PhaseActive::Bypassed
    }
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    #[inline(always)]
    pub fn is_ph_shift(&self) -> bool {
        *self == PhaseActive::PhShift
    }
}
#[doc = "Field `PHASE_ACTIVE` writer - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
pub type PhaseActiveW<'a, REG> = crate::BitWriter<'a, REG, PhaseActive>;
impl<'a, REG> PhaseActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bypassed."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(PhaseActive::Bypassed)
    }
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    #[inline(always)]
    pub fn ph_shift(self) -> &'a mut crate::W<REG> {
        self.variant(PhaseActive::PhShift)
    }
}
#[doc = "Field `CCLK_DRV_DELAY` reader - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub type CclkDrvDelayR = crate::FieldReader;
#[doc = "Field `CCLK_DRV_DELAY` writer - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub type CclkDrvDelayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Enables drive delay, as controlled by the CCLK_DRV_DELAY field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CclkDrvDelayActive {
    #[doc = "0: Disable drive delay."]
    Disable = 0,
    #[doc = "1: Enable drive delay."]
    Enable = 1,
}
impl From<CclkDrvDelayActive> for bool {
    #[inline(always)]
    fn from(variant: CclkDrvDelayActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` reader - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub type CclkDrvDelayActiveR = crate::BitReader<CclkDrvDelayActive>;
impl CclkDrvDelayActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkDrvDelayActive {
        match self.bits {
            false => CclkDrvDelayActive::Disable,
            true => CclkDrvDelayActive::Enable,
        }
    }
    #[doc = "Disable drive delay."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CclkDrvDelayActive::Disable
    }
    #[doc = "Enable drive delay."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CclkDrvDelayActive::Enable
    }
}
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` writer - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub type CclkDrvDelayActiveW<'a, REG> = crate::BitWriter<'a, REG, CclkDrvDelayActive>;
impl<'a, REG> CclkDrvDelayActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable drive delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CclkDrvDelayActive::Disable)
    }
    #[doc = "Enable drive delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CclkDrvDelayActive::Enable)
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CclkSampleDelayR = crate::FieldReader;
#[doc = "Field `CCLK_SAMPLE_DELAY` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CclkSampleDelayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CclkSampleDelayActive {
    #[doc = "0: Disables sample delay."]
    Disable = 0,
    #[doc = "1: Enables sample delay."]
    Enable = 1,
}
impl From<CclkSampleDelayActive> for bool {
    #[inline(always)]
    fn from(variant: CclkSampleDelayActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` reader - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub type CclkSampleDelayActiveR = crate::BitReader<CclkSampleDelayActive>;
impl CclkSampleDelayActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkSampleDelayActive {
        match self.bits {
            false => CclkSampleDelayActive::Disable,
            true => CclkSampleDelayActive::Enable,
        }
    }
    #[doc = "Disables sample delay."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CclkSampleDelayActive::Disable
    }
    #[doc = "Enables sample delay."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CclkSampleDelayActive::Enable
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` writer - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub type CclkSampleDelayActiveW<'a, REG> = crate::BitWriter<'a, REG, CclkSampleDelayActive>;
impl<'a, REG> CclkSampleDelayActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables sample delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CclkSampleDelayActive::Disable)
    }
    #[doc = "Enables sample delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CclkSampleDelayActive::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&self) -> CclkDrvPhaseR {
        CclkDrvPhaseR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&self) -> CclkSamplePhaseR {
        CclkSamplePhaseR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub fn phase_active(&self) -> PhaseActiveR {
        PhaseActiveR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&self) -> CclkDrvDelayR {
        CclkDrvDelayR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&self) -> CclkDrvDelayActiveR {
        CclkDrvDelayActiveR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&self) -> CclkSampleDelayR {
        CclkSampleDelayR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&self) -> CclkSampleDelayActiveR {
        CclkSampleDelayActiveR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&mut self) -> CclkDrvPhaseW<SdioclkctrlSpec> {
        CclkDrvPhaseW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&mut self) -> CclkSamplePhaseW<SdioclkctrlSpec> {
        CclkSamplePhaseW::new(self, 2)
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub fn phase_active(&mut self) -> PhaseActiveW<SdioclkctrlSpec> {
        PhaseActiveW::new(self, 7)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&mut self) -> CclkDrvDelayW<SdioclkctrlSpec> {
        CclkDrvDelayW::new(self, 16)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&mut self) -> CclkDrvDelayActiveW<SdioclkctrlSpec> {
        CclkDrvDelayActiveW::new(self, 23)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&mut self) -> CclkSampleDelayW<SdioclkctrlSpec> {
        CclkSampleDelayW::new(self, 24)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&mut self) -> CclkSampleDelayActiveW<SdioclkctrlSpec> {
        CclkSampleDelayActiveW::new(self, 31)
    }
}
#[doc = "SDIO CCLKIN phase and delay control\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdioclkctrlSpec;
impl crate::RegisterSpec for SdioclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdioclkctrl::R`](R) reader structure"]
impl crate::Readable for SdioclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdioclkctrl::W`](W) writer structure"]
impl crate::Writable for SdioclkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIOCLKCTRL to value 0"]
impl crate::Resettable for SdioclkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
