#[doc = "Register `PLL0STAT` reader"]
pub type R = crate::R<Pll0statSpec>;
#[doc = "Register `PLL0STAT` writer"]
pub type W = crate::W<Pll0statSpec>;
#[doc = "Field `LOCK` reader - lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\]
:100 kHz to 20 MHz."]
pub type LockR = crate::BitReader;
#[doc = "Field `PREDIVACK` reader - pre-divider ratio change acknowledge."]
pub type PredivackR = crate::BitReader;
#[doc = "Field `FEEDDIVACK` reader - feedback divider ratio change acknowledge."]
pub type FeeddivackR = crate::BitReader;
#[doc = "Field `POSTDIVACK` reader - post-divider ratio change acknowledge."]
pub type PostdivackR = crate::BitReader;
#[doc = "Field `FRMDET` reader - free running detector output (active high)."]
pub type FrmdetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - lock detector output (active high) Warning: The lock signal is only reliable between fref\\[2\\]
:100 kHz to 20 MHz."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pre-divider ratio change acknowledge."]
    #[inline(always)]
    pub fn predivack(&self) -> PredivackR {
        PredivackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - feedback divider ratio change acknowledge."]
    #[inline(always)]
    pub fn feeddivack(&self) -> FeeddivackR {
        FeeddivackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - post-divider ratio change acknowledge."]
    #[inline(always)]
    pub fn postdivack(&self) -> PostdivackR {
        PostdivackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - free running detector output (active high)."]
    #[inline(always)]
    pub fn frmdet(&self) -> FrmdetR {
        FrmdetR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {}
#[doc = "PLL0 550m status\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll0statSpec;
impl crate::RegisterSpec for Pll0statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll0stat::R`](R) reader structure"]
impl crate::Readable for Pll0statSpec {}
#[doc = "`write(|w| ..)` method takes [`pll0stat::W`](W) writer structure"]
impl crate::Writable for Pll0statSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL0STAT to value 0"]
impl crate::Resettable for Pll0statSpec {
    const RESET_VALUE: u32 = 0;
}
