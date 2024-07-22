#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FETCHERSCATTERGATHER` reader - Fetcher scatter/gather"]
pub type FetcherscattergatherR = crate::BitReader;
#[doc = "Field `FETCHERSCATTERGATHER` writer - Fetcher scatter/gather"]
pub type FetcherscattergatherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERSCATTERGATHER` reader - Pusher scatter/gather"]
pub type PusherscattergatherR = crate::BitReader;
#[doc = "Field `PUSHERSCATTERGATHER` writer - Pusher scatter/gather"]
pub type PusherscattergatherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPFETCHER` reader - Stop fetcher"]
pub type StopfetcherR = crate::BitReader;
#[doc = "Field `STOPFETCHER` writer - Stop fetcher"]
pub type StopfetcherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPPUSHER` reader - Stop pusher"]
pub type StoppusherR = crate::BitReader;
#[doc = "Field `STOPPUSHER` writer - Stop pusher"]
pub type StoppusherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRESET` reader - Software reset"]
pub type SwresetR = crate::BitReader;
#[doc = "Field `SWRESET` writer - Software reset"]
pub type SwresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fetcher scatter/gather"]
    #[inline(always)]
    pub fn fetcherscattergather(&self) -> FetcherscattergatherR {
        FetcherscattergatherR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pusher scatter/gather"]
    #[inline(always)]
    pub fn pusherscattergather(&self) -> PusherscattergatherR {
        PusherscattergatherR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop fetcher"]
    #[inline(always)]
    pub fn stopfetcher(&self) -> StopfetcherR {
        StopfetcherR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop pusher"]
    #[inline(always)]
    pub fn stoppusher(&self) -> StoppusherR {
        StoppusherR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software reset"]
    #[inline(always)]
    pub fn swreset(&self) -> SwresetR {
        SwresetR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fetcher scatter/gather"]
    #[inline(always)]
    #[must_use]
    pub fn fetcherscattergather(&mut self) -> FetcherscattergatherW<CtrlSpec> {
        FetcherscattergatherW::new(self, 0)
    }
    #[doc = "Bit 1 - Pusher scatter/gather"]
    #[inline(always)]
    #[must_use]
    pub fn pusherscattergather(&mut self) -> PusherscattergatherW<CtrlSpec> {
        PusherscattergatherW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop fetcher"]
    #[inline(always)]
    #[must_use]
    pub fn stopfetcher(&mut self) -> StopfetcherW<CtrlSpec> {
        StopfetcherW::new(self, 2)
    }
    #[doc = "Bit 3 - Stop pusher"]
    #[inline(always)]
    #[must_use]
    pub fn stoppusher(&mut self) -> StoppusherW<CtrlSpec> {
        StoppusherW::new(self, 3)
    }
    #[doc = "Bit 4 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SwresetW<CtrlSpec> {
        SwresetW::new(self, 4)
    }
}
#[doc = "Control register, called CONFIG in Barco datasheet.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
