#[doc = "Register `CONSUMER_RAC_CTIIN0` reader"]
pub type R = crate::R<ConsumerRacCtiin0Spec>;
#[doc = "Register `CONSUMER_RAC_CTIIN0` writer"]
pub type W = crate::W<ConsumerRacCtiin0Spec>;
#[doc = "Field `PRSSEL` reader - CTI async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CTI async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CTI async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTI async channel select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<ConsumerRacCtiin0Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "CTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerRacCtiin0Spec;
impl crate::RegisterSpec for ConsumerRacCtiin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_rac_ctiin0::R`](R) reader structure"]
impl crate::Readable for ConsumerRacCtiin0Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_rac_ctiin0::W`](W) writer structure"]
impl crate::Writable for ConsumerRacCtiin0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONSUMER_RAC_CTIIN0 to value 0"]
impl crate::Resettable for ConsumerRacCtiin0Spec {
    const RESET_VALUE: u32 = 0;
}
