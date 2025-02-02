#[doc = "Register `CONSUMER_VDAC0_SYNCTRIGCH1` reader"]
pub type R = crate::R<ConsumerVdac0Synctrigch1Spec>;
#[doc = "Register `CONSUMER_VDAC0_SYNCTRIGCH1` writer"]
pub type W = crate::W<ConsumerVdac0Synctrigch1Spec>;
#[doc = "Field `SPRSSEL` reader - SYNCTRIG sync channel select"]
pub type SprsselR = crate::FieldReader;
#[doc = "Field `SPRSSEL` writer - SYNCTRIG sync channel select"]
pub type SprsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 8:9 - SYNCTRIG sync channel select"]
    #[inline(always)]
    pub fn sprssel(&self) -> SprsselR {
        SprsselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - SYNCTRIG sync channel select"]
    #[inline(always)]
    #[must_use]
    pub fn sprssel(&mut self) -> SprsselW<ConsumerVdac0Synctrigch1Spec> {
        SprsselW::new(self, 8)
    }
}
#[doc = "SYNCTRIG Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac0_synctrigch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac0_synctrigch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerVdac0Synctrigch1Spec;
impl crate::RegisterSpec for ConsumerVdac0Synctrigch1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_vdac0_synctrigch1::R`](R) reader structure"]
impl crate::Readable for ConsumerVdac0Synctrigch1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_vdac0_synctrigch1::W`](W) writer structure"]
impl crate::Writable for ConsumerVdac0Synctrigch1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONSUMER_VDAC0_SYNCTRIGCH1 to value 0"]
impl crate::Resettable for ConsumerVdac0Synctrigch1Spec {
    const RESET_VALUE: u32 = 0;
}
