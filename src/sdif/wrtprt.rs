#[doc = "Register `WRTPRT` reader"]
pub type R = crate::R<WrtprtSpec>;
#[doc = "Register `WRTPRT` writer"]
pub type W = crate::W<WrtprtSpec>;
#[doc = "Field `WRITE_PROTECT` reader - Write protect."]
pub type WriteProtectR = crate::BitReader;
#[doc = "Field `WRITE_PROTECT` writer - Write protect."]
pub type WriteProtectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&self) -> WriteProtectR {
        WriteProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&mut self) -> WriteProtectW<WrtprtSpec> {
        WriteProtectW::new(self, 0)
    }
}
#[doc = "Write Protect register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrtprtSpec;
impl crate::RegisterSpec for WrtprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrtprt::R`](R) reader structure"]
impl crate::Readable for WrtprtSpec {}
#[doc = "`write(|w| ..)` method takes [`wrtprt::W`](W) writer structure"]
impl crate::Writable for WrtprtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRTPRT to value 0"]
impl crate::Resettable for WrtprtSpec {
    const RESET_VALUE: u32 = 0;
}
