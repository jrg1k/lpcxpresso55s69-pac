#[doc = "Register `COUNT` reader"]
pub type R = crate::R<CountSpec>;
#[doc = "Register `COUNT` writer"]
pub type W = crate::W<CountSpec>;
#[doc = "Field `CTR_L` reader - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
pub type CtrLR = crate::FieldReader<u16>;
#[doc = "Field `CTR_L` writer - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
pub type CtrLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CTR_H` reader - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
pub type CtrHR = crate::FieldReader<u16>;
#[doc = "Field `CTR_H` writer - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
pub type CtrHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_l(&self) -> CtrLR {
        CtrLR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_h(&self) -> CtrHR {
        CtrHR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_l(&mut self) -> CtrLW<CountSpec> {
        CtrLW::new(self, 0)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub fn ctr_h(&mut self) -> CtrHW<CountSpec> {
        CtrHW::new(self, 16)
    }
}
#[doc = "SCT counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountSpec;
impl crate::RegisterSpec for CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for CountSpec {}
#[doc = "`write(|w| ..)` method takes [`count::W`](W) writer structure"]
impl crate::Writable for CountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for CountSpec {
    const RESET_VALUE: u32 = 0;
}
