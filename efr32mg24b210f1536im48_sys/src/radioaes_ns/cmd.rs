#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `STARTFETCHER` writer - Start fetch"]
pub type StartfetcherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTPUSHER` writer - Start push"]
pub type StartpusherW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start fetch"]
    #[inline(always)]
    #[must_use]
    pub fn startfetcher(&mut self) -> StartfetcherW<CmdSpec> {
        StartfetcherW::new(self, 0)
    }
    #[doc = "Bit 1 - Start push"]
    #[inline(always)]
    #[must_use]
    pub fn startpusher(&mut self) -> StartpusherW<CmdSpec> {
        StartpusherW::new(self, 1)
    }
}
#[doc = "Command register for starting the fetcher and pusher\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
