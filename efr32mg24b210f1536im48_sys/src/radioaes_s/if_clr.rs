#[doc = "Register `IF_CLR` writer"]
pub type W = crate::W<IfClrSpec>;
#[doc = "Field `FETCHERENDOFBLOCK` writer - End of block interrupt flag clear"]
pub type FetcherendofblockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHERSTOPPED` writer - Stopped interrupt flag clear"]
pub type FetcherstoppedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHERERROR` writer - Error interrupt flag clear"]
pub type FetchererrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERENDOFBLOCK` writer - FETCHERENDOFBLOCKIFC"]
pub type PusherendofblockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERSTOPPED` writer - FETCHERSTOPPEDIFC"]
pub type PusherstoppedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERERROR` writer - FETCHERERRORIFC"]
pub type PushererrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - End of block interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn fetcherendofblock(&mut self) -> FetcherendofblockW<IfClrSpec> {
        FetcherendofblockW::new(self, 0)
    }
    #[doc = "Bit 1 - Stopped interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn fetcherstopped(&mut self) -> FetcherstoppedW<IfClrSpec> {
        FetcherstoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Error interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn fetchererror(&mut self) -> FetchererrorW<IfClrSpec> {
        FetchererrorW::new(self, 2)
    }
    #[doc = "Bit 3 - FETCHERENDOFBLOCKIFC"]
    #[inline(always)]
    #[must_use]
    pub fn pusherendofblock(&mut self) -> PusherendofblockW<IfClrSpec> {
        PusherendofblockW::new(self, 3)
    }
    #[doc = "Bit 4 - FETCHERSTOPPEDIFC"]
    #[inline(always)]
    #[must_use]
    pub fn pusherstopped(&mut self) -> PusherstoppedW<IfClrSpec> {
        PusherstoppedW::new(self, 4)
    }
    #[doc = "Bit 5 - FETCHERERRORIFC"]
    #[inline(always)]
    #[must_use]
    pub fn pushererror(&mut self) -> PushererrorW<IfClrSpec> {
        PushererrorW::new(self, 5)
    }
}
#[doc = "Writing a '1' clears the interrupt status. Writing a '0' has no effect.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfClrSpec;
impl crate::RegisterSpec for IfClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if_clr::W`](W) writer structure"]
impl crate::Writable for IfClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IF_CLR to value 0"]
impl crate::Resettable for IfClrSpec {
    const RESET_VALUE: u32 = 0;
}
