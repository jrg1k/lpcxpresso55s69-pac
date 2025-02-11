#[doc = "Register `PORTSC1` reader"]
pub type R = crate::R<Portsc1Spec>;
#[doc = "Register `PORTSC1` writer"]
pub type W = crate::W<Portsc1Spec>;
#[doc = "Field `CCS` reader - Current Connect Status: Logic 1 indicates a device is present on the port."]
pub type CcsR = crate::BitReader;
#[doc = "Field `CCS` writer - Current Connect Status: Logic 1 indicates a device is present on the port."]
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - Connect Status Change: Logic 1 means that the value of CCS has changed."]
pub type CscR = crate::BitReader;
#[doc = "Field `CSC` writer - Connect Status Change: Logic 1 means that the value of CCS has changed."]
pub type CscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - Port Enabled/Disabled."]
pub type PedR = crate::BitReader;
#[doc = "Field `PED` writer - Port Enabled/Disabled."]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDC` reader - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
pub type PedcR = crate::BitReader;
#[doc = "Field `PEDC` writer - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
pub type PedcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCA` reader - Over-current active: Logic 1 means that this port has an over-current condition."]
pub type OcaR = crate::BitReader;
#[doc = "Field `OCA` writer - Over-current active: Logic 1 means that this port has an over-current condition."]
pub type OcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCC` reader - Over-current change: Logic 1 means that the value of OCA has changed."]
pub type OccR = crate::BitReader;
#[doc = "Field `OCC` writer - Over-current change: Logic 1 means that the value of OCA has changed."]
pub type OccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPR` reader - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
pub type FprR = crate::BitReader;
#[doc = "Field `FPR` writer - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
pub type FprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend: Logic 1 means port is in the suspend state."]
pub type SuspR = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend: Logic 1 means port is in the suspend state."]
pub type SuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Port Reset: Logic 1 means the port is in the reset state."]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Port Reset: Logic 1 means the port is in the reset state."]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
pub type LsR = crate::FieldReader;
#[doc = "Field `PP` reader - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
pub type PpR = crate::BitReader;
#[doc = "Field `PP` writer - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
pub type PpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIC` reader - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
pub type PicR = crate::FieldReader;
#[doc = "Field `PIC` writer - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
pub type PicW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PTC` reader - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
pub type PtcR = crate::FieldReader;
#[doc = "Field `PTC` writer - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
pub type PtcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSPD` reader - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
pub type PspdR = crate::FieldReader;
#[doc = "Field `PSPD` writer - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
pub type PspdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WOO` reader - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
pub type WooR = crate::BitReader;
#[doc = "Field `WOO` writer - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
pub type WooW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&self) -> PedcR {
        PedcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&self) -> OcaR {
        OcaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&self) -> OccR {
        OccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&self) -> FprR {
        FprR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&self) -> SuspR {
        SuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&self) -> PpR {
        PpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&self) -> PicR {
        PicR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&self) -> PtcR {
        PtcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&self) -> PspdR {
        PspdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&self) -> WooR {
        WooR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&mut self) -> CcsW<Portsc1Spec> {
        CcsW::new(self, 0)
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&mut self) -> CscW<Portsc1Spec> {
        CscW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&mut self) -> PedW<Portsc1Spec> {
        PedW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&mut self) -> PedcW<Portsc1Spec> {
        PedcW::new(self, 3)
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&mut self) -> OcaW<Portsc1Spec> {
        OcaW::new(self, 4)
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&mut self) -> OccW<Portsc1Spec> {
        OccW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&mut self) -> FprW<Portsc1Spec> {
        FprW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&mut self) -> SuspW<Portsc1Spec> {
        SuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<Portsc1Spec> {
        PrW::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&mut self) -> PpW<Portsc1Spec> {
        PpW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&mut self) -> PicW<Portsc1Spec> {
        PicW::new(self, 14)
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&mut self) -> PtcW<Portsc1Spec> {
        PtcW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&mut self) -> PspdW<Portsc1Spec> {
        PspdW::new(self, 20)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&mut self) -> WooW<Portsc1Spec> {
        WooW::new(self, 22)
    }
}
#[doc = "Port Status and Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`portsc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portsc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Portsc1Spec;
impl crate::RegisterSpec for Portsc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portsc1::R`](R) reader structure"]
impl crate::Readable for Portsc1Spec {}
#[doc = "`write(|w| ..)` method takes [`portsc1::W`](W) writer structure"]
impl crate::Writable for Portsc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PORTSC1 to value 0"]
impl crate::Resettable for Portsc1Spec {
    const RESET_VALUE: u32 = 0;
}
