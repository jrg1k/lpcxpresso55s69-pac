#[doc = "Register `SECURE_BOOT_CFG` reader"]
pub type R = crate::R<SecureBootCfgSpec>;
#[doc = "Register `SECURE_BOOT_CFG` writer"]
pub type W = crate::W<SecureBootCfgSpec>;
#[doc = "Use RSA4096 keys only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsa4k {
    #[doc = "0: Allow RSA2048 and higher"]
    Value0 = 0,
    #[doc = "1: RSA4096 only"]
    Value1 = 1,
    #[doc = "2: RSA4096 only"]
    Value2 = 2,
    #[doc = "3: RSA4096 only"]
    Value3 = 3,
}
impl From<Rsa4k> for u8 {
    #[inline(always)]
    fn from(variant: Rsa4k) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsa4k {
    type Ux = u8;
}
impl crate::IsEnum for Rsa4k {}
#[doc = "Field `RSA4K` reader - Use RSA4096 keys only."]
pub type Rsa4kR = crate::FieldReader<Rsa4k>;
impl Rsa4kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsa4k {
        match self.bits {
            0 => Rsa4k::Value0,
            1 => Rsa4k::Value1,
            2 => Rsa4k::Value2,
            3 => Rsa4k::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Allow RSA2048 and higher"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Rsa4k::Value0
    }
    #[doc = "RSA4096 only"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Rsa4k::Value1
    }
    #[doc = "RSA4096 only"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == Rsa4k::Value2
    }
    #[doc = "RSA4096 only"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == Rsa4k::Value3
    }
}
#[doc = "Field `RSA4K` writer - Use RSA4096 keys only."]
pub type Rsa4kW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rsa4k, crate::Safe>;
impl<'a, REG> Rsa4kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Allow RSA2048 and higher"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsa4k::Value0)
    }
    #[doc = "RSA4096 only"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsa4k::Value1)
    }
    #[doc = "RSA4096 only"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsa4k::Value2)
    }
    #[doc = "RSA4096 only"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rsa4k::Value3)
    }
}
#[doc = "Include NXP area in DICE computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DiceIncNxpCfg {
    #[doc = "0: not included"]
    NotInclud = 0,
    #[doc = "1: included"]
    Includ = 1,
    #[doc = "2: included"]
    Value2 = 2,
    #[doc = "3: included"]
    Value3 = 3,
}
impl From<DiceIncNxpCfg> for u8 {
    #[inline(always)]
    fn from(variant: DiceIncNxpCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DiceIncNxpCfg {
    type Ux = u8;
}
impl crate::IsEnum for DiceIncNxpCfg {}
#[doc = "Field `DICE_INC_NXP_CFG` reader - Include NXP area in DICE computation."]
pub type DiceIncNxpCfgR = crate::FieldReader<DiceIncNxpCfg>;
impl DiceIncNxpCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DiceIncNxpCfg {
        match self.bits {
            0 => DiceIncNxpCfg::NotInclud,
            1 => DiceIncNxpCfg::Includ,
            2 => DiceIncNxpCfg::Value2,
            3 => DiceIncNxpCfg::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "not included"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == DiceIncNxpCfg::NotInclud
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn is_includ(&self) -> bool {
        *self == DiceIncNxpCfg::Includ
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DiceIncNxpCfg::Value2
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DiceIncNxpCfg::Value3
    }
}
#[doc = "Field `DICE_INC_NXP_CFG` writer - Include NXP area in DICE computation."]
pub type DiceIncNxpCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, DiceIncNxpCfg, crate::Safe>;
impl<'a, REG> DiceIncNxpCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not included"]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut crate::W<REG> {
        self.variant(DiceIncNxpCfg::NotInclud)
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn includ(self) -> &'a mut crate::W<REG> {
        self.variant(DiceIncNxpCfg::Includ)
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(DiceIncNxpCfg::Value2)
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(DiceIncNxpCfg::Value3)
    }
}
#[doc = "Include Customer factory area (including keys) in DICE computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DiceCustCfg {
    #[doc = "0: not included"]
    NotInclud = 0,
    #[doc = "1: included"]
    Unclud = 1,
    #[doc = "2: included"]
    Value2 = 2,
    #[doc = "3: included"]
    Value3 = 3,
}
impl From<DiceCustCfg> for u8 {
    #[inline(always)]
    fn from(variant: DiceCustCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DiceCustCfg {
    type Ux = u8;
}
impl crate::IsEnum for DiceCustCfg {}
#[doc = "Field `DICE_CUST_CFG` reader - Include Customer factory area (including keys) in DICE computation."]
pub type DiceCustCfgR = crate::FieldReader<DiceCustCfg>;
impl DiceCustCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DiceCustCfg {
        match self.bits {
            0 => DiceCustCfg::NotInclud,
            1 => DiceCustCfg::Unclud,
            2 => DiceCustCfg::Value2,
            3 => DiceCustCfg::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "not included"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == DiceCustCfg::NotInclud
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn is_unclud(&self) -> bool {
        *self == DiceCustCfg::Unclud
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DiceCustCfg::Value2
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DiceCustCfg::Value3
    }
}
#[doc = "Field `DICE_CUST_CFG` writer - Include Customer factory area (including keys) in DICE computation."]
pub type DiceCustCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, DiceCustCfg, crate::Safe>;
impl<'a, REG> DiceCustCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "not included"]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut crate::W<REG> {
        self.variant(DiceCustCfg::NotInclud)
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn unclud(self) -> &'a mut crate::W<REG> {
        self.variant(DiceCustCfg::Unclud)
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(DiceCustCfg::Value2)
    }
    #[doc = "included"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(DiceCustCfg::Value3)
    }
}
#[doc = "Skip DICE computation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SkipDice {
    #[doc = "0: Enable DICE"]
    Enable = 0,
    #[doc = "1: Disable DICE"]
    Disable = 1,
    #[doc = "2: Disable DICE"]
    Value2 = 2,
    #[doc = "3: Disable DICE"]
    Value3 = 3,
}
impl From<SkipDice> for u8 {
    #[inline(always)]
    fn from(variant: SkipDice) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SkipDice {
    type Ux = u8;
}
impl crate::IsEnum for SkipDice {}
#[doc = "Field `SKIP_DICE` reader - Skip DICE computation"]
pub type SkipDiceR = crate::FieldReader<SkipDice>;
impl SkipDiceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SkipDice {
        match self.bits {
            0 => SkipDice::Enable,
            1 => SkipDice::Disable,
            2 => SkipDice::Value2,
            3 => SkipDice::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Enable DICE"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SkipDice::Enable
    }
    #[doc = "Disable DICE"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SkipDice::Disable
    }
    #[doc = "Disable DICE"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == SkipDice::Value2
    }
    #[doc = "Disable DICE"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == SkipDice::Value3
    }
}
#[doc = "Field `SKIP_DICE` writer - Skip DICE computation"]
pub type SkipDiceW<'a, REG> = crate::FieldWriter<'a, REG, 2, SkipDice, crate::Safe>;
impl<'a, REG> SkipDiceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable DICE"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SkipDice::Enable)
    }
    #[doc = "Disable DICE"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SkipDice::Disable)
    }
    #[doc = "Disable DICE"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(SkipDice::Value2)
    }
    #[doc = "Disable DICE"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(SkipDice::Value3)
    }
}
#[doc = "TrustZone-M mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TzmImageType {
    #[doc = "0: TZ-M image mode is taken from application image header"]
    Value0 = 0,
    #[doc = "1: TZ-M disabled image, boots to non-secure mode"]
    Value1 = 1,
    #[doc = "2: TZ-M enabled image, boots to secure mode"]
    Value2 = 2,
    #[doc = "3: TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header"]
    Value3 = 3,
}
impl From<TzmImageType> for u8 {
    #[inline(always)]
    fn from(variant: TzmImageType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TzmImageType {
    type Ux = u8;
}
impl crate::IsEnum for TzmImageType {}
#[doc = "Field `TZM_IMAGE_TYPE` reader - TrustZone-M mode"]
pub type TzmImageTypeR = crate::FieldReader<TzmImageType>;
impl TzmImageTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TzmImageType {
        match self.bits {
            0 => TzmImageType::Value0,
            1 => TzmImageType::Value1,
            2 => TzmImageType::Value2,
            3 => TzmImageType::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "TZ-M image mode is taken from application image header"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == TzmImageType::Value0
    }
    #[doc = "TZ-M disabled image, boots to non-secure mode"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == TzmImageType::Value1
    }
    #[doc = "TZ-M enabled image, boots to secure mode"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == TzmImageType::Value2
    }
    #[doc = "TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == TzmImageType::Value3
    }
}
#[doc = "Field `TZM_IMAGE_TYPE` writer - TrustZone-M mode"]
pub type TzmImageTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, TzmImageType, crate::Safe>;
impl<'a, REG> TzmImageTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TZ-M image mode is taken from application image header"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut crate::W<REG> {
        self.variant(TzmImageType::Value0)
    }
    #[doc = "TZ-M disabled image, boots to non-secure mode"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut crate::W<REG> {
        self.variant(TzmImageType::Value1)
    }
    #[doc = "TZ-M enabled image, boots to secure mode"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(TzmImageType::Value2)
    }
    #[doc = "TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(TzmImageType::Value3)
    }
}
#[doc = "Block PUF key code generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockSetKey {
    #[doc = "0: Allow PUF Key Code generation"]
    Allow = 0,
    #[doc = "1: Disable PUF Key Code generation"]
    Disable = 1,
    #[doc = "2: Disable PUF Key Code generation"]
    Value2 = 2,
    #[doc = "3: Disable PUF Key Code generation"]
    Value3 = 3,
}
impl From<BlockSetKey> for u8 {
    #[inline(always)]
    fn from(variant: BlockSetKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BlockSetKey {
    type Ux = u8;
}
impl crate::IsEnum for BlockSetKey {}
#[doc = "Field `BLOCK_SET_KEY` reader - Block PUF key code generation"]
pub type BlockSetKeyR = crate::FieldReader<BlockSetKey>;
impl BlockSetKeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockSetKey {
        match self.bits {
            0 => BlockSetKey::Allow,
            1 => BlockSetKey::Disable,
            2 => BlockSetKey::Value2,
            3 => BlockSetKey::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Allow PUF Key Code generation"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == BlockSetKey::Allow
    }
    #[doc = "Disable PUF Key Code generation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BlockSetKey::Disable
    }
    #[doc = "Disable PUF Key Code generation"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BlockSetKey::Value2
    }
    #[doc = "Disable PUF Key Code generation"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BlockSetKey::Value3
    }
}
#[doc = "Field `BLOCK_SET_KEY` writer - Block PUF key code generation"]
pub type BlockSetKeyW<'a, REG> = crate::FieldWriter<'a, REG, 2, BlockSetKey, crate::Safe>;
impl<'a, REG> BlockSetKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Allow PUF Key Code generation"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSetKey::Allow)
    }
    #[doc = "Disable PUF Key Code generation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSetKey::Disable)
    }
    #[doc = "Disable PUF Key Code generation"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSetKey::Value2)
    }
    #[doc = "Disable PUF Key Code generation"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(BlockSetKey::Value3)
    }
}
#[doc = "Block PUF enrollement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockEnroll {
    #[doc = "0: Allow PUF enroll operation"]
    Allow = 0,
    #[doc = "1: Disable PUF enroll operation"]
    Disable = 1,
    #[doc = "2: Disable PUF enroll operation"]
    Value2 = 2,
    #[doc = "3: Disable PUF enroll operation"]
    Value3 = 3,
}
impl From<BlockEnroll> for u8 {
    #[inline(always)]
    fn from(variant: BlockEnroll) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BlockEnroll {
    type Ux = u8;
}
impl crate::IsEnum for BlockEnroll {}
#[doc = "Field `BLOCK_ENROLL` reader - Block PUF enrollement"]
pub type BlockEnrollR = crate::FieldReader<BlockEnroll>;
impl BlockEnrollR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockEnroll {
        match self.bits {
            0 => BlockEnroll::Allow,
            1 => BlockEnroll::Disable,
            2 => BlockEnroll::Value2,
            3 => BlockEnroll::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Allow PUF enroll operation"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == BlockEnroll::Allow
    }
    #[doc = "Disable PUF enroll operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BlockEnroll::Disable
    }
    #[doc = "Disable PUF enroll operation"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BlockEnroll::Value2
    }
    #[doc = "Disable PUF enroll operation"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BlockEnroll::Value3
    }
}
#[doc = "Field `BLOCK_ENROLL` writer - Block PUF enrollement"]
pub type BlockEnrollW<'a, REG> = crate::FieldWriter<'a, REG, 2, BlockEnroll, crate::Safe>;
impl<'a, REG> BlockEnrollW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Allow PUF enroll operation"]
    #[inline(always)]
    pub fn allow(self) -> &'a mut crate::W<REG> {
        self.variant(BlockEnroll::Allow)
    }
    #[doc = "Disable PUF enroll operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BlockEnroll::Disable)
    }
    #[doc = "Disable PUF enroll operation"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(BlockEnroll::Value2)
    }
    #[doc = "Disable PUF enroll operation"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(BlockEnroll::Value3)
    }
}
#[doc = "Field `DICE_INC_SEC_EPOCH` reader - Include security EPOCH in DICE"]
pub type DiceIncSecEpochR = crate::FieldReader;
#[doc = "Field `DICE_INC_SEC_EPOCH` writer - Include security EPOCH in DICE"]
pub type DiceIncSecEpochW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Secure boot enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecBootEn {
    #[doc = "0: Plain image (internal flash with or without CRC)"]
    Disable = 0,
    #[doc = "1: Boot signed images. (internal flash, RSA signed)"]
    Enable = 1,
    #[doc = "2: Boot signed images. (internal flash, RSA signed)"]
    Value2 = 2,
    #[doc = "3: Boot signed images. (internal flash, RSA signed)"]
    Value3 = 3,
}
impl From<SecBootEn> for u8 {
    #[inline(always)]
    fn from(variant: SecBootEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecBootEn {
    type Ux = u8;
}
impl crate::IsEnum for SecBootEn {}
#[doc = "Field `SEC_BOOT_EN` reader - Secure boot enable"]
pub type SecBootEnR = crate::FieldReader<SecBootEn>;
impl SecBootEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecBootEn {
        match self.bits {
            0 => SecBootEn::Disable,
            1 => SecBootEn::Enable,
            2 => SecBootEn::Value2,
            3 => SecBootEn::Value3,
            _ => unreachable!(),
        }
    }
    #[doc = "Plain image (internal flash with or without CRC)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SecBootEn::Disable
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SecBootEn::Enable
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == SecBootEn::Value2
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == SecBootEn::Value3
    }
}
#[doc = "Field `SEC_BOOT_EN` writer - Secure boot enable"]
pub type SecBootEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecBootEn, crate::Safe>;
impl<'a, REG> SecBootEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Plain image (internal flash with or without CRC)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootEn::Disable)
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootEn::Enable)
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootEn::Value2)
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootEn::Value3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Use RSA4096 keys only."]
    #[inline(always)]
    pub fn rsa4k(&self) -> Rsa4kR {
        Rsa4kR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation."]
    #[inline(always)]
    pub fn dice_inc_nxp_cfg(&self) -> DiceIncNxpCfgR {
        DiceIncNxpCfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation."]
    #[inline(always)]
    pub fn dice_cust_cfg(&self) -> DiceCustCfgR {
        DiceCustCfgR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Skip DICE computation"]
    #[inline(always)]
    pub fn skip_dice(&self) -> SkipDiceR {
        SkipDiceR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode"]
    #[inline(always)]
    pub fn tzm_image_type(&self) -> TzmImageTypeR {
        TzmImageTypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation"]
    #[inline(always)]
    pub fn block_set_key(&self) -> BlockSetKeyR {
        BlockSetKeyR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement"]
    #[inline(always)]
    pub fn block_enroll(&self) -> BlockEnrollR {
        BlockEnrollR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Include security EPOCH in DICE"]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&self) -> DiceIncSecEpochR {
        DiceIncSecEpochR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Secure boot enable"]
    #[inline(always)]
    pub fn sec_boot_en(&self) -> SecBootEnR {
        SecBootEnR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use RSA4096 keys only."]
    #[inline(always)]
    pub fn rsa4k(&mut self) -> Rsa4kW<SecureBootCfgSpec> {
        Rsa4kW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation."]
    #[inline(always)]
    pub fn dice_inc_nxp_cfg(&mut self) -> DiceIncNxpCfgW<SecureBootCfgSpec> {
        DiceIncNxpCfgW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation."]
    #[inline(always)]
    pub fn dice_cust_cfg(&mut self) -> DiceCustCfgW<SecureBootCfgSpec> {
        DiceCustCfgW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Skip DICE computation"]
    #[inline(always)]
    pub fn skip_dice(&mut self) -> SkipDiceW<SecureBootCfgSpec> {
        SkipDiceW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode"]
    #[inline(always)]
    pub fn tzm_image_type(&mut self) -> TzmImageTypeW<SecureBootCfgSpec> {
        TzmImageTypeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation"]
    #[inline(always)]
    pub fn block_set_key(&mut self) -> BlockSetKeyW<SecureBootCfgSpec> {
        BlockSetKeyW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement"]
    #[inline(always)]
    pub fn block_enroll(&mut self) -> BlockEnrollW<SecureBootCfgSpec> {
        BlockEnrollW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Include security EPOCH in DICE"]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&mut self) -> DiceIncSecEpochW<SecureBootCfgSpec> {
        DiceIncSecEpochW::new(self, 14)
    }
    #[doc = "Bits 30:31 - Secure boot enable"]
    #[inline(always)]
    pub fn sec_boot_en(&mut self) -> SecBootEnW<SecureBootCfgSpec> {
        SecBootEnW::new(self, 30)
    }
}
#[doc = "Secure boot configuration flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_boot_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure_boot_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureBootCfgSpec;
impl crate::RegisterSpec for SecureBootCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_boot_cfg::R`](R) reader structure"]
impl crate::Readable for SecureBootCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`secure_boot_cfg::W`](W) writer structure"]
impl crate::Writable for SecureBootCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECURE_BOOT_CFG to value 0"]
impl crate::Resettable for SecureBootCfgSpec {
    const RESET_VALUE: u32 = 0;
}
