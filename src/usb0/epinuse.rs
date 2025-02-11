#[doc = "Register `EPINUSE` reader"]
pub type R = crate::R<EpinuseSpec>;
#[doc = "Register `EPINUSE` writer"]
pub type W = crate::W<EpinuseSpec>;
#[doc = "Field `BUF` reader - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
pub type BufR = crate::FieldReader;
#[doc = "Field `BUF` writer - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
pub type BufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:9 - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
    #[inline(always)]
    pub fn buf(&self) -> BufR {
        BufR::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:9 - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
    #[inline(always)]
    pub fn buf(&mut self) -> BufW<EpinuseSpec> {
        BufW::new(self, 2)
    }
}
#[doc = "USB Endpoint Buffer in use\n\nYou can [`read`](crate::Reg::read) this register and get [`epinuse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinuse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpinuseSpec;
impl crate::RegisterSpec for EpinuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epinuse::R`](R) reader structure"]
impl crate::Readable for EpinuseSpec {}
#[doc = "`write(|w| ..)` method takes [`epinuse::W`](W) writer structure"]
impl crate::Writable for EpinuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPINUSE to value 0"]
impl crate::Resettable for EpinuseSpec {
    const RESET_VALUE: u32 = 0;
}
