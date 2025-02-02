#[doc = "Register `TIMER4_ROUTEEN` reader"]
pub type R = crate::R<Timer4RouteenSpec>;
#[doc = "Register `TIMER4_ROUTEEN` writer"]
pub type W = crate::W<Timer4RouteenSpec>;
#[doc = "Field `CC0PEN` reader - CC0 pin enable control bit"]
pub type Cc0penR = crate::BitReader;
#[doc = "Field `CC0PEN` writer - CC0 pin enable control bit"]
pub type Cc0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1PEN` reader - CC1 pin enable control bit"]
pub type Cc1penR = crate::BitReader;
#[doc = "Field `CC1PEN` writer - CC1 pin enable control bit"]
pub type Cc1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2PEN` reader - CC2 pin enable control bit"]
pub type Cc2penR = crate::BitReader;
#[doc = "Field `CC2PEN` writer - CC2 pin enable control bit"]
pub type Cc2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCC0PEN` reader - CDTI0 pin enable control bit"]
pub type Ccc0penR = crate::BitReader;
#[doc = "Field `CCC0PEN` writer - CDTI0 pin enable control bit"]
pub type Ccc0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCC1PEN` reader - CDTI1 pin enable control bit"]
pub type Ccc1penR = crate::BitReader;
#[doc = "Field `CCC1PEN` writer - CDTI1 pin enable control bit"]
pub type Ccc1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCC2PEN` reader - CDTI2 pin enable control bit"]
pub type Ccc2penR = crate::BitReader;
#[doc = "Field `CCC2PEN` writer - CDTI2 pin enable control bit"]
pub type Ccc2penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC0 pin enable control bit"]
    #[inline(always)]
    pub fn cc0pen(&self) -> Cc0penR {
        Cc0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1 pin enable control bit"]
    #[inline(always)]
    pub fn cc1pen(&self) -> Cc1penR {
        Cc1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2 pin enable control bit"]
    #[inline(always)]
    pub fn cc2pen(&self) -> Cc2penR {
        Cc2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CDTI0 pin enable control bit"]
    #[inline(always)]
    pub fn ccc0pen(&self) -> Ccc0penR {
        Ccc0penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDTI1 pin enable control bit"]
    #[inline(always)]
    pub fn ccc1pen(&self) -> Ccc1penR {
        Ccc1penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CDTI2 pin enable control bit"]
    #[inline(always)]
    pub fn ccc2pen(&self) -> Ccc2penR {
        Ccc2penR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cc0pen(&mut self) -> Cc0penW<Timer4RouteenSpec> {
        Cc0penW::new(self, 0)
    }
    #[doc = "Bit 1 - CC1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cc1pen(&mut self) -> Cc1penW<Timer4RouteenSpec> {
        Cc1penW::new(self, 1)
    }
    #[doc = "Bit 2 - CC2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn cc2pen(&mut self) -> Cc2penW<Timer4RouteenSpec> {
        Cc2penW::new(self, 2)
    }
    #[doc = "Bit 3 - CDTI0 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccc0pen(&mut self) -> Ccc0penW<Timer4RouteenSpec> {
        Ccc0penW::new(self, 3)
    }
    #[doc = "Bit 4 - CDTI1 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccc1pen(&mut self) -> Ccc1penW<Timer4RouteenSpec> {
        Ccc1penW::new(self, 4)
    }
    #[doc = "Bit 5 - CDTI2 pin enable control bit"]
    #[inline(always)]
    #[must_use]
    pub fn ccc2pen(&mut self) -> Ccc2penW<Timer4RouteenSpec> {
        Ccc2penW::new(self, 5)
    }
}
#[doc = "TIMER4 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`timer4_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer4_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer4RouteenSpec;
impl crate::RegisterSpec for Timer4RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer4_routeen::R`](R) reader structure"]
impl crate::Readable for Timer4RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`timer4_routeen::W`](W) writer structure"]
impl crate::Writable for Timer4RouteenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER4_ROUTEEN to value 0"]
impl crate::Resettable for Timer4RouteenSpec {
    const RESET_VALUE: u32 = 0;
}
