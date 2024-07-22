#[doc = "Register `PORTB_DOUT` reader"]
pub type R = crate::R<PortbDoutSpec>;
#[doc = "Register `PORTB_DOUT` writer"]
pub type W = crate::W<PortbDoutSpec>;
#[doc = "Field `DOUT` reader - Data output"]
pub type DoutR = crate::FieldReader;
#[doc = "Field `DOUT` writer - Data output"]
pub type DoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Data output"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data output"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DoutW<PortbDoutSpec> {
        DoutW::new(self, 0)
    }
}
#[doc = "data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portb_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portb_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortbDoutSpec;
impl crate::RegisterSpec for PortbDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portb_dout::R`](R) reader structure"]
impl crate::Readable for PortbDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`portb_dout::W`](W) writer structure"]
impl crate::Writable for PortbDoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PORTB_DOUT to value 0"]
impl crate::Resettable for PortbDoutSpec {
    const RESET_VALUE: u32 = 0;
}
