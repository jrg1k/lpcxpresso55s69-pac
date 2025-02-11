#[doc = "Register `BACKENDPWR` reader"]
pub type R = crate::R<BackendpwrSpec>;
#[doc = "Register `BACKENDPWR` writer"]
pub type W = crate::W<BackendpwrSpec>;
#[doc = "Field `BACKENDPWR` reader - Back-end Power control for card application."]
pub type BackendpwrR = crate::BitReader;
#[doc = "Field `BACKENDPWR` writer - Back-end Power control for card application."]
pub type BackendpwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Back-end Power control for card application."]
    #[inline(always)]
    pub fn backendpwr(&self) -> BackendpwrR {
        BackendpwrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Back-end Power control for card application."]
    #[inline(always)]
    pub fn backendpwr(&mut self) -> BackendpwrW<BackendpwrSpec> {
        BackendpwrW::new(self, 0)
    }
}
#[doc = "Power control\n\nYou can [`read`](crate::Reg::read) this register and get [`backendpwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backendpwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BackendpwrSpec;
impl crate::RegisterSpec for BackendpwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backendpwr::R`](R) reader structure"]
impl crate::Readable for BackendpwrSpec {}
#[doc = "`write(|w| ..)` method takes [`backendpwr::W`](W) writer structure"]
impl crate::Writable for BackendpwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACKENDPWR to value 0"]
impl crate::Resettable for BackendpwrSpec {
    const RESET_VALUE: u32 = 0;
}
