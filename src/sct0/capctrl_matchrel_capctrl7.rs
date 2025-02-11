#[doc = "Register `CAPCTRL7` reader"]
pub type R = crate::R<CapctrlMatchrelCapctrl7Spec>;
#[doc = "Register `CAPCTRL7` writer"]
pub type W = crate::W<CapctrlMatchrelCapctrl7Spec>;
#[doc = "Field `CAPCONn_L` reader - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnLR = crate::FieldReader<u16>;
#[doc = "Field `CAPCONn_L` writer - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CAPCONn_H` reader - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnHR = crate::FieldReader<u16>;
#[doc = "Field `CAPCONn_H` writer - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
pub type CapconnHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&self) -> CapconnLR {
        CapconnLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&self) -> CapconnHR {
        CapconnHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_l(&mut self) -> CapconnLW<CapctrlMatchrelCapctrl7Spec> {
        CapconnLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub fn capconn_h(&mut self) -> CapconnHW<CapctrlMatchrelCapctrl7Spec> {
        CapconnHW::new(self, 16)
    }
}
#[doc = "SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapctrlMatchrelCapctrl7Spec;
impl crate::RegisterSpec for CapctrlMatchrelCapctrl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capctrl_matchrel_capctrl7::R`](R) reader structure"]
impl crate::Readable for CapctrlMatchrelCapctrl7Spec {}
#[doc = "`write(|w| ..)` method takes [`capctrl_matchrel_capctrl7::W`](W) writer structure"]
impl crate::Writable for CapctrlMatchrelCapctrl7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPCTRL7 to value 0"]
impl crate::Resettable for CapctrlMatchrelCapctrl7Spec {
    const RESET_VALUE: u32 = 0;
}
