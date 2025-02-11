#[doc = "Register `DEBUG_FEATURES` reader"]
pub type R = crate::R<DebugFeaturesSpec>;
#[doc = "Register `DEBUG_FEATURES` writer"]
pub type W = crate::W<DebugFeaturesSpec>;
#[doc = "CPU0 Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Dbgen {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    Disable = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    Enable = 2,
}
impl From<Cpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Dbgen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Dbgen {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Dbgen {}
#[doc = "Field `CPU0_DBGEN` reader - CPU0 Invasive debug control:."]
pub type Cpu0DbgenR = crate::FieldReader<Cpu0Dbgen>;
impl Cpu0DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0Dbgen> {
        match self.bits {
            1 => Some(Cpu0Dbgen::Disable),
            2 => Some(Cpu0Dbgen::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu0Dbgen::Disable
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0Dbgen::Enable
    }
}
#[doc = "Field `CPU0_DBGEN` writer - CPU0 Invasive debug control:."]
pub type Cpu0DbgenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0Dbgen>;
impl<'a, REG> Cpu0DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Dbgen::Disable)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Dbgen::Enable)
    }
}
#[doc = "CPU0 Non Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Niden {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    Disable = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    Enable = 2,
}
impl From<Cpu0Niden> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Niden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Niden {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Niden {}
#[doc = "Field `CPU0_NIDEN` reader - CPU0 Non Invasive debug control:."]
pub type Cpu0NidenR = crate::FieldReader<Cpu0Niden>;
impl Cpu0NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0Niden> {
        match self.bits {
            1 => Some(Cpu0Niden::Disable),
            2 => Some(Cpu0Niden::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu0Niden::Disable
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0Niden::Enable
    }
}
#[doc = "Field `CPU0_NIDEN` writer - CPU0 Non Invasive debug control:."]
pub type Cpu0NidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0Niden>;
impl<'a, REG> Cpu0NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Niden::Disable)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Niden::Enable)
    }
}
#[doc = "CPU0 Secure Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Spiden {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    Disable = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    Enable = 2,
}
impl From<Cpu0Spiden> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Spiden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Spiden {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Spiden {}
#[doc = "Field `CPU0_SPIDEN` reader - CPU0 Secure Invasive debug control:."]
pub type Cpu0SpidenR = crate::FieldReader<Cpu0Spiden>;
impl Cpu0SpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0Spiden> {
        match self.bits {
            1 => Some(Cpu0Spiden::Disable),
            2 => Some(Cpu0Spiden::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu0Spiden::Disable
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0Spiden::Enable
    }
}
#[doc = "Field `CPU0_SPIDEN` writer - CPU0 Secure Invasive debug control:."]
pub type Cpu0SpidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0Spiden>;
impl<'a, REG> Cpu0SpidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Spiden::Disable)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Spiden::Enable)
    }
}
#[doc = "CPU0 Secure Non Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu0Spniden {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    Disable = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    Enable = 2,
}
impl From<Cpu0Spniden> for u8 {
    #[inline(always)]
    fn from(variant: Cpu0Spniden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu0Spniden {
    type Ux = u8;
}
impl crate::IsEnum for Cpu0Spniden {}
#[doc = "Field `CPU0_SPNIDEN` reader - CPU0 Secure Non Invasive debug control:."]
pub type Cpu0SpnidenR = crate::FieldReader<Cpu0Spniden>;
impl Cpu0SpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu0Spniden> {
        match self.bits {
            1 => Some(Cpu0Spniden::Disable),
            2 => Some(Cpu0Spniden::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu0Spniden::Disable
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu0Spniden::Enable
    }
}
#[doc = "Field `CPU0_SPNIDEN` writer - CPU0 Secure Non Invasive debug control:."]
pub type Cpu0SpnidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu0Spniden>;
impl<'a, REG> Cpu0SpnidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Spniden::Disable)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu0Spniden::Enable)
    }
}
#[doc = "CPU1 Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1Dbgen {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    Disable = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    Enable = 2,
}
impl From<Cpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1Dbgen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1Dbgen {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1Dbgen {}
#[doc = "Field `CPU1_DBGEN` reader - CPU1 Invasive debug control:."]
pub type Cpu1DbgenR = crate::FieldReader<Cpu1Dbgen>;
impl Cpu1DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu1Dbgen> {
        match self.bits {
            1 => Some(Cpu1Dbgen::Disable),
            2 => Some(Cpu1Dbgen::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu1Dbgen::Disable
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1Dbgen::Enable
    }
}
#[doc = "Field `CPU1_DBGEN` writer - CPU1 Invasive debug control:."]
pub type Cpu1DbgenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu1Dbgen>;
impl<'a, REG> Cpu1DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Dbgen::Disable)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Dbgen::Enable)
    }
}
#[doc = "CPU1 Non Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1Niden {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    Disable = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    Enable = 2,
}
impl From<Cpu1Niden> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1Niden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1Niden {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1Niden {}
#[doc = "Field `CPU1_NIDEN` reader - CPU1 Non Invasive debug control:."]
pub type Cpu1NidenR = crate::FieldReader<Cpu1Niden>;
impl Cpu1NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu1Niden> {
        match self.bits {
            1 => Some(Cpu1Niden::Disable),
            2 => Some(Cpu1Niden::Enable),
            _ => None,
        }
    }
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu1Niden::Disable
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1Niden::Enable
    }
}
#[doc = "Field `CPU1_NIDEN` writer - CPU1 Non Invasive debug control:."]
pub type Cpu1NidenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu1Niden>;
impl<'a, REG> Cpu1NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Niden::Disable)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Niden::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU0 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_dbgen(&self) -> Cpu0DbgenR {
        Cpu0DbgenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_niden(&self) -> Cpu0NidenR {
        Cpu0NidenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spiden(&self) -> Cpu0SpidenR {
        Cpu0SpidenR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spniden(&self) -> Cpu0SpnidenR {
        Cpu0SpnidenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CPU1 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_dbgen(&self) -> Cpu1DbgenR {
        Cpu1DbgenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CPU1 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_niden(&self) -> Cpu1NidenR {
        Cpu1NidenR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_dbgen(&mut self) -> Cpu0DbgenW<DebugFeaturesSpec> {
        Cpu0DbgenW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_niden(&mut self) -> Cpu0NidenW<DebugFeaturesSpec> {
        Cpu0NidenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spiden(&mut self) -> Cpu0SpidenW<DebugFeaturesSpec> {
        Cpu0SpidenW::new(self, 4)
    }
    #[doc = "Bits 6:7 - CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spniden(&mut self) -> Cpu0SpnidenW<DebugFeaturesSpec> {
        Cpu0SpnidenW::new(self, 6)
    }
    #[doc = "Bits 8:9 - CPU1 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_dbgen(&mut self) -> Cpu1DbgenW<DebugFeaturesSpec> {
        Cpu1DbgenW::new(self, 8)
    }
    #[doc = "Bits 10:11 - CPU1 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu1_niden(&mut self) -> Cpu1NidenW<DebugFeaturesSpec> {
        Cpu1NidenW::new(self, 10)
    }
}
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_features::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_features::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugFeaturesSpec;
impl crate::RegisterSpec for DebugFeaturesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_features::R`](R) reader structure"]
impl crate::Readable for DebugFeaturesSpec {}
#[doc = "`write(|w| ..)` method takes [`debug_features::W`](W) writer structure"]
impl crate::Writable for DebugFeaturesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_FEATURES to value 0"]
impl crate::Resettable for DebugFeaturesSpec {
    const RESET_VALUE: u32 = 0;
}
