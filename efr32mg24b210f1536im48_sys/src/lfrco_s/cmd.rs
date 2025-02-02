#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `REDUCETCINT` writer - Reduce Temperature Check Interval"]
pub type ReducetcintW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Reduce Temperature Check Interval"]
    #[inline(always)]
    #[must_use]
    pub fn reducetcint(&mut self) -> ReducetcintW<CmdSpec> {
        ReducetcintW::new(self, 0)
    }
}
#[doc = "Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
