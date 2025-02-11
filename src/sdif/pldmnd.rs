#[doc = "Register `PLDMND` reader"]
pub type R = crate::R<PldmndSpec>;
#[doc = "Register `PLDMND` writer"]
pub type W = crate::W<PldmndSpec>;
#[doc = "Field `PD` reader - Poll Demand."]
pub type PdR = crate::FieldReader<u32>;
#[doc = "Field `PD` writer - Poll Demand."]
pub type PdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Poll Demand."]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Poll Demand."]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<PldmndSpec> {
        PdW::new(self, 0)
    }
}
#[doc = "Poll Demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`pldmnd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pldmnd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PldmndSpec;
impl crate::RegisterSpec for PldmndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pldmnd::R`](R) reader structure"]
impl crate::Readable for PldmndSpec {}
#[doc = "`write(|w| ..)` method takes [`pldmnd::W`](W) writer structure"]
impl crate::Writable for PldmndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLDMND to value 0"]
impl crate::Resettable for PldmndSpec {
    const RESET_VALUE: u32 = 0;
}
