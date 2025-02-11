#[doc = "Register `WAKEUPIOCTRL` reader"]
pub type R = crate::R<WakeupioctrlSpec>;
#[doc = "Register `WAKEUPIOCTRL` writer"]
pub type W = crate::W<WakeupioctrlSpec>;
#[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup0 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup0> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP0` reader - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Risingedgewakeup0R = crate::BitReader<Risingedgewakeup0>;
impl Risingedgewakeup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup0 {
        match self.bits {
            false => Risingedgewakeup0::Disable,
            true => Risingedgewakeup0::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup0::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup0::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP0` writer - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Risingedgewakeup0W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup0>;
impl<'a, REG> Risingedgewakeup0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup0::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup0::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup0 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup0> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP0` reader - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup0R = crate::BitReader<Fallingedgewakeup0>;
impl Fallingedgewakeup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup0 {
        match self.bits {
            false => Fallingedgewakeup0::Disable,
            true => Fallingedgewakeup0::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup0::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup0::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP0` writer - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup0W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup0>;
impl<'a, REG> Fallingedgewakeup0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup0::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup0::Enable)
    }
}
#[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup1 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup1> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP1` reader - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Risingedgewakeup1R = crate::BitReader<Risingedgewakeup1>;
impl Risingedgewakeup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup1 {
        match self.bits {
            false => Risingedgewakeup1::Disable,
            true => Risingedgewakeup1::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup1::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup1::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP1` writer - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Risingedgewakeup1W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup1>;
impl<'a, REG> Risingedgewakeup1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup1::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup1::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup1 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup1> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP1` reader - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup1R = crate::BitReader<Fallingedgewakeup1>;
impl Fallingedgewakeup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup1 {
        match self.bits {
            false => Fallingedgewakeup1::Disable,
            true => Fallingedgewakeup1::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup1::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup1::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP1` writer - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup1W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup1>;
impl<'a, REG> Fallingedgewakeup1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup1::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup1::Enable)
    }
}
#[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup2 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup2> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP2` reader - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Risingedgewakeup2R = crate::BitReader<Risingedgewakeup2>;
impl Risingedgewakeup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup2 {
        match self.bits {
            false => Risingedgewakeup2::Disable,
            true => Risingedgewakeup2::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup2::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup2::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP2` writer - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Risingedgewakeup2W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup2>;
impl<'a, REG> Risingedgewakeup2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup2::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup2::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup2 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup2> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP2` reader - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup2R = crate::BitReader<Fallingedgewakeup2>;
impl Fallingedgewakeup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup2 {
        match self.bits {
            false => Fallingedgewakeup2::Disable,
            true => Fallingedgewakeup2::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup2::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup2::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP2` writer - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup2W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup2>;
impl<'a, REG> Fallingedgewakeup2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup2::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup2::Enable)
    }
}
#[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Risingedgewakeup3 {
    #[doc = "0: Rising edge detection is disable."]
    Disable = 0,
    #[doc = "1: Rising edge detection is enable."]
    Enable = 1,
}
impl From<Risingedgewakeup3> for bool {
    #[inline(always)]
    fn from(variant: Risingedgewakeup3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RISINGEDGEWAKEUP3` reader - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Risingedgewakeup3R = crate::BitReader<Risingedgewakeup3>;
impl Risingedgewakeup3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risingedgewakeup3 {
        match self.bits {
            false => Risingedgewakeup3::Disable,
            true => Risingedgewakeup3::Enable,
        }
    }
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Risingedgewakeup3::Disable
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Risingedgewakeup3::Enable
    }
}
#[doc = "Field `RISINGEDGEWAKEUP3` writer - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Risingedgewakeup3W<'a, REG> = crate::BitWriter<'a, REG, Risingedgewakeup3>;
impl<'a, REG> Risingedgewakeup3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup3::Disable)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Risingedgewakeup3::Enable)
    }
}
#[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallingedgewakeup3 {
    #[doc = "0: Falling edge detection is disable."]
    Disable = 0,
    #[doc = "1: Falling edge detection is enable."]
    Enable = 1,
}
impl From<Fallingedgewakeup3> for bool {
    #[inline(always)]
    fn from(variant: Fallingedgewakeup3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP3` reader - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup3R = crate::BitReader<Fallingedgewakeup3>;
impl Fallingedgewakeup3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fallingedgewakeup3 {
        match self.bits {
            false => Fallingedgewakeup3::Disable,
            true => Fallingedgewakeup3::Enable,
        }
    }
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fallingedgewakeup3::Disable
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fallingedgewakeup3::Enable
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP3` writer - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type Fallingedgewakeup3W<'a, REG> = crate::BitWriter<'a, REG, Fallingedgewakeup3>;
impl<'a, REG> Fallingedgewakeup3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup3::Disable)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fallingedgewakeup3::Enable)
    }
}
#[doc = "Field `MODEWAKEUP0` reader - Configure wake up I/O 0 in Deep Power Down mode"]
pub type Modewakeup0R = crate::BitReader;
#[doc = "Field `MODEWAKEUP0` writer - Configure wake up I/O 0 in Deep Power Down mode"]
pub type Modewakeup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEWAKEUP1` reader - Configure wake up I/O 1 in Deep Power Down mode"]
pub type Modewakeup1R = crate::BitReader;
#[doc = "Field `MODEWAKEUP1` writer - Configure wake up I/O 1 in Deep Power Down mode"]
pub type Modewakeup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEWAKEUP2` reader - Configure wake up I/O 2 in Deep Power Down mode"]
pub type Modewakeup2R = crate::BitReader;
#[doc = "Field `MODEWAKEUP2` writer - Configure wake up I/O 2 in Deep Power Down mode"]
pub type Modewakeup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEWAKEUP3` reader - Configure wake up I/O 3 in Deep Power Down mode"]
pub type Modewakeup3R = crate::BitReader;
#[doc = "Field `MODEWAKEUP3` writer - Configure wake up I/O 3 in Deep Power Down mode"]
pub type Modewakeup3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup0(&self) -> Risingedgewakeup0R {
        Risingedgewakeup0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup0(&self) -> Fallingedgewakeup0R {
        Fallingedgewakeup0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup1(&self) -> Risingedgewakeup1R {
        Risingedgewakeup1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup1(&self) -> Fallingedgewakeup1R {
        Fallingedgewakeup1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup2(&self) -> Risingedgewakeup2R {
        Risingedgewakeup2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup2(&self) -> Fallingedgewakeup2R {
        Fallingedgewakeup2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup3(&self) -> Risingedgewakeup3R {
        Risingedgewakeup3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup3(&self) -> Fallingedgewakeup3R {
        Fallingedgewakeup3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configure wake up I/O 0 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup0(&self) -> Modewakeup0R {
        Modewakeup0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configure wake up I/O 1 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup1(&self) -> Modewakeup1R {
        Modewakeup1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configure wake up I/O 2 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup2(&self) -> Modewakeup2R {
        Modewakeup2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configure wake up I/O 3 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup3(&self) -> Modewakeup3R {
        Modewakeup3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup0(&mut self) -> Risingedgewakeup0W<WakeupioctrlSpec> {
        Risingedgewakeup0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup0(&mut self) -> Fallingedgewakeup0W<WakeupioctrlSpec> {
        Fallingedgewakeup0W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup1(&mut self) -> Risingedgewakeup1W<WakeupioctrlSpec> {
        Risingedgewakeup1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup1(&mut self) -> Fallingedgewakeup1W<WakeupioctrlSpec> {
        Fallingedgewakeup1W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup2(&mut self) -> Risingedgewakeup2W<WakeupioctrlSpec> {
        Risingedgewakeup2W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup2(&mut self) -> Fallingedgewakeup2W<WakeupioctrlSpec> {
        Fallingedgewakeup2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup3(&mut self) -> Risingedgewakeup3W<WakeupioctrlSpec> {
        Risingedgewakeup3W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup3(&mut self) -> Fallingedgewakeup3W<WakeupioctrlSpec> {
        Fallingedgewakeup3W::new(self, 7)
    }
    #[doc = "Bit 8 - Configure wake up I/O 0 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup0(&mut self) -> Modewakeup0W<WakeupioctrlSpec> {
        Modewakeup0W::new(self, 8)
    }
    #[doc = "Bit 9 - Configure wake up I/O 1 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup1(&mut self) -> Modewakeup1W<WakeupioctrlSpec> {
        Modewakeup1W::new(self, 9)
    }
    #[doc = "Bit 10 - Configure wake up I/O 2 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup2(&mut self) -> Modewakeup2W<WakeupioctrlSpec> {
        Modewakeup2W::new(self, 10)
    }
    #[doc = "Bit 11 - Configure wake up I/O 3 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup3(&mut self) -> Modewakeup3W<WakeupioctrlSpec> {
        Modewakeup3W::new(self, 11)
    }
}
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeupioctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeupioctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupioctrlSpec;
impl crate::RegisterSpec for WakeupioctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeupioctrl::R`](R) reader structure"]
impl crate::Readable for WakeupioctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeupioctrl::W`](W) writer structure"]
impl crate::Writable for WakeupioctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUPIOCTRL to value 0"]
impl crate::Resettable for WakeupioctrlSpec {
    const RESET_VALUE: u32 = 0;
}
