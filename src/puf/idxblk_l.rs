#[doc = "Register `IDXBLK_L` reader"]
pub type R = crate::R<IdxblkLSpec>;
#[doc = "Register `IDXBLK_L` writer"]
pub type W = crate::W<IdxblkLSpec>;
#[doc = "Field `IDX1` reader - Use to block PUF index 1"]
pub type Idx1R = crate::FieldReader;
#[doc = "Field `IDX1` writer - Use to block PUF index 1"]
pub type Idx1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX2` reader - Use to block PUF index 2"]
pub type Idx2R = crate::FieldReader;
#[doc = "Field `IDX2` writer - Use to block PUF index 2"]
pub type Idx2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX3` reader - Use to block PUF index 3"]
pub type Idx3R = crate::FieldReader;
#[doc = "Field `IDX3` writer - Use to block PUF index 3"]
pub type Idx3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX4` reader - Use to block PUF index 4"]
pub type Idx4R = crate::FieldReader;
#[doc = "Field `IDX4` writer - Use to block PUF index 4"]
pub type Idx4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX5` reader - Use to block PUF index 5"]
pub type Idx5R = crate::FieldReader;
#[doc = "Field `IDX5` writer - Use to block PUF index 5"]
pub type Idx5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX6` reader - Use to block PUF index 6"]
pub type Idx6R = crate::FieldReader;
#[doc = "Field `IDX6` writer - Use to block PUF index 6"]
pub type Idx6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX7` reader - Use to block PUF index 7"]
pub type Idx7R = crate::FieldReader;
#[doc = "Field `IDX7` writer - Use to block PUF index 7"]
pub type Idx7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOCK_IDX` writer - Lock 0 to 7 PUF key indexes"]
pub type LockIdxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&self) -> Idx1R {
        Idx1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&self) -> Idx2R {
        Idx2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&self) -> Idx3R {
        Idx3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&self) -> Idx4R {
        Idx4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&self) -> Idx5R {
        Idx5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&self) -> Idx6R {
        Idx6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&self) -> Idx7R {
        Idx7R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&mut self) -> Idx1W<IdxblkLSpec> {
        Idx1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&mut self) -> Idx2W<IdxblkLSpec> {
        Idx2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&mut self) -> Idx3W<IdxblkLSpec> {
        Idx3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&mut self) -> Idx4W<IdxblkLSpec> {
        Idx4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&mut self) -> Idx5W<IdxblkLSpec> {
        Idx5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&mut self) -> Idx6W<IdxblkLSpec> {
        Idx6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&mut self) -> Idx7W<IdxblkLSpec> {
        Idx7W::new(self, 14)
    }
    #[doc = "Bits 30:31 - Lock 0 to 7 PUF key indexes"]
    #[inline(always)]
    pub fn lock_idx(&mut self) -> LockIdxW<IdxblkLSpec> {
        LockIdxW::new(self, 30)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdxblkLSpec;
impl crate::RegisterSpec for IdxblkLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idxblk_l::R`](R) reader structure"]
impl crate::Readable for IdxblkLSpec {}
#[doc = "`write(|w| ..)` method takes [`idxblk_l::W`](W) writer structure"]
impl crate::Writable for IdxblkLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDXBLK_L to value 0x8000_aaaa"]
impl crate::Resettable for IdxblkLSpec {
    const RESET_VALUE: u32 = 0x8000_aaaa;
}
