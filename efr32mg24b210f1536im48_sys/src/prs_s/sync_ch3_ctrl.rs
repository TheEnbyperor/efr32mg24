#[doc = "Register `SYNC_CH3_CTRL` reader"]
pub type R = crate::R<SyncCh3CtrlSpec>;
#[doc = "Register `SYNC_CH3_CTRL` writer"]
pub type W = crate::W<SyncCh3CtrlSpec>;
#[doc = "Signal Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sigsel {
    #[doc = "0: NONE"]
    None = 0,
}
impl From<Sigsel> for u8 {
    #[inline(always)]
    fn from(variant: Sigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sigsel {
    type Ux = u8;
}
impl crate::IsEnum for Sigsel {}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader<Sigsel>;
impl SigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sigsel> {
        match self.bits {
            0 => Some(Sigsel::None),
            _ => None,
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sigsel::None
    }
}
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sigsel>;
impl<'a, REG> SigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sigsel::None)
    }
}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader;
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SigselW<SyncCh3CtrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SourceselW<SyncCh3CtrlSpec> {
        SourceselW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncCh3CtrlSpec;
impl crate::RegisterSpec for SyncCh3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_ch3_ctrl::R`](R) reader structure"]
impl crate::Readable for SyncCh3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_ch3_ctrl::W`](W) writer structure"]
impl crate::Writable for SyncCh3CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC_CH3_CTRL to value 0"]
impl crate::Resettable for SyncCh3CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
