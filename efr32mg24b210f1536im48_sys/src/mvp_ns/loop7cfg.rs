#[doc = "Register `LOOP7CFG` reader"]
pub type R = crate::R<Loop7cfgSpec>;
#[doc = "Register `LOOP7CFG` writer"]
pub type W = crate::W<Loop7cfgSpec>;
#[doc = "Field `NUMITERS` reader - Number of Iterations"]
pub type NumitersR = crate::FieldReader<u16>;
#[doc = "Field `NUMITERS` writer - Number of Iterations"]
pub type NumitersW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ARRAY0INCRDIM0` reader - Increment Dimension 0"]
pub type Array0incrdim0R = crate::BitReader;
#[doc = "Field `ARRAY0INCRDIM0` writer - Increment Dimension 0"]
pub type Array0incrdim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY0INCRDIM1` reader - Increment Dimension 1"]
pub type Array0incrdim1R = crate::BitReader;
#[doc = "Field `ARRAY0INCRDIM1` writer - Increment Dimension 1"]
pub type Array0incrdim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY0INCRDIM2` reader - Increment Dimension 2"]
pub type Array0incrdim2R = crate::BitReader;
#[doc = "Field `ARRAY0INCRDIM2` writer - Increment Dimension 2"]
pub type Array0incrdim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY1INCRDIM0` reader - Increment Dimension 0"]
pub type Array1incrdim0R = crate::BitReader;
#[doc = "Field `ARRAY1INCRDIM0` writer - Increment Dimension 0"]
pub type Array1incrdim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY1INCRDIM1` reader - Increment Dimension 1"]
pub type Array1incrdim1R = crate::BitReader;
#[doc = "Field `ARRAY1INCRDIM1` writer - Increment Dimension 1"]
pub type Array1incrdim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY1INCRDIM2` reader - Increment Dimension 2"]
pub type Array1incrdim2R = crate::BitReader;
#[doc = "Field `ARRAY1INCRDIM2` writer - Increment Dimension 2"]
pub type Array1incrdim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY2INCRDIM0` reader - Increment Dimension 0"]
pub type Array2incrdim0R = crate::BitReader;
#[doc = "Field `ARRAY2INCRDIM0` writer - Increment Dimension 0"]
pub type Array2incrdim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY2INCRDIM1` reader - Increment Dimension 1"]
pub type Array2incrdim1R = crate::BitReader;
#[doc = "Field `ARRAY2INCRDIM1` writer - Increment Dimension 1"]
pub type Array2incrdim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY2INCRDIM2` reader - Increment Dimension 2"]
pub type Array2incrdim2R = crate::BitReader;
#[doc = "Field `ARRAY2INCRDIM2` writer - Increment Dimension 2"]
pub type Array2incrdim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY3INCRDIM0` reader - Increment Dimension 0"]
pub type Array3incrdim0R = crate::BitReader;
#[doc = "Field `ARRAY3INCRDIM0` writer - Increment Dimension 0"]
pub type Array3incrdim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY3INCRDIM1` reader - Increment Dimension 1"]
pub type Array3incrdim1R = crate::BitReader;
#[doc = "Field `ARRAY3INCRDIM1` writer - Increment Dimension 1"]
pub type Array3incrdim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY3INCRDIM2` reader - Increment Dimension 2"]
pub type Array3incrdim2R = crate::BitReader;
#[doc = "Field `ARRAY3INCRDIM2` writer - Increment Dimension 2"]
pub type Array3incrdim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY4INCRDIM0` reader - Increment Dimension 0"]
pub type Array4incrdim0R = crate::BitReader;
#[doc = "Field `ARRAY4INCRDIM0` writer - Increment Dimension 0"]
pub type Array4incrdim0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY4INCRDIM1` reader - Increment Dimension 1"]
pub type Array4incrdim1R = crate::BitReader;
#[doc = "Field `ARRAY4INCRDIM1` writer - Increment Dimension 1"]
pub type Array4incrdim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRAY4INCRDIM2` reader - Increment Dimension 2"]
pub type Array4incrdim2R = crate::BitReader;
#[doc = "Field `ARRAY4INCRDIM2` writer - Increment Dimension 2"]
pub type Array4incrdim2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Number of Iterations"]
    #[inline(always)]
    pub fn numiters(&self) -> NumitersR {
        NumitersR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Increment Dimension 0"]
    #[inline(always)]
    pub fn array0incrdim0(&self) -> Array0incrdim0R {
        Array0incrdim0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Increment Dimension 1"]
    #[inline(always)]
    pub fn array0incrdim1(&self) -> Array0incrdim1R {
        Array0incrdim1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Increment Dimension 2"]
    #[inline(always)]
    pub fn array0incrdim2(&self) -> Array0incrdim2R {
        Array0incrdim2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Increment Dimension 0"]
    #[inline(always)]
    pub fn array1incrdim0(&self) -> Array1incrdim0R {
        Array1incrdim0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Increment Dimension 1"]
    #[inline(always)]
    pub fn array1incrdim1(&self) -> Array1incrdim1R {
        Array1incrdim1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Increment Dimension 2"]
    #[inline(always)]
    pub fn array1incrdim2(&self) -> Array1incrdim2R {
        Array1incrdim2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Increment Dimension 0"]
    #[inline(always)]
    pub fn array2incrdim0(&self) -> Array2incrdim0R {
        Array2incrdim0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Increment Dimension 1"]
    #[inline(always)]
    pub fn array2incrdim1(&self) -> Array2incrdim1R {
        Array2incrdim1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Increment Dimension 2"]
    #[inline(always)]
    pub fn array2incrdim2(&self) -> Array2incrdim2R {
        Array2incrdim2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Increment Dimension 0"]
    #[inline(always)]
    pub fn array3incrdim0(&self) -> Array3incrdim0R {
        Array3incrdim0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Increment Dimension 1"]
    #[inline(always)]
    pub fn array3incrdim1(&self) -> Array3incrdim1R {
        Array3incrdim1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Increment Dimension 2"]
    #[inline(always)]
    pub fn array3incrdim2(&self) -> Array3incrdim2R {
        Array3incrdim2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Increment Dimension 0"]
    #[inline(always)]
    pub fn array4incrdim0(&self) -> Array4incrdim0R {
        Array4incrdim0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Increment Dimension 1"]
    #[inline(always)]
    pub fn array4incrdim1(&self) -> Array4incrdim1R {
        Array4incrdim1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Increment Dimension 2"]
    #[inline(always)]
    pub fn array4incrdim2(&self) -> Array4incrdim2R {
        Array4incrdim2R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number of Iterations"]
    #[inline(always)]
    #[must_use]
    pub fn numiters(&mut self) -> NumitersW<Loop7cfgSpec> {
        NumitersW::new(self, 0)
    }
    #[doc = "Bit 12 - Increment Dimension 0"]
    #[inline(always)]
    #[must_use]
    pub fn array0incrdim0(&mut self) -> Array0incrdim0W<Loop7cfgSpec> {
        Array0incrdim0W::new(self, 12)
    }
    #[doc = "Bit 13 - Increment Dimension 1"]
    #[inline(always)]
    #[must_use]
    pub fn array0incrdim1(&mut self) -> Array0incrdim1W<Loop7cfgSpec> {
        Array0incrdim1W::new(self, 13)
    }
    #[doc = "Bit 14 - Increment Dimension 2"]
    #[inline(always)]
    #[must_use]
    pub fn array0incrdim2(&mut self) -> Array0incrdim2W<Loop7cfgSpec> {
        Array0incrdim2W::new(self, 14)
    }
    #[doc = "Bit 16 - Increment Dimension 0"]
    #[inline(always)]
    #[must_use]
    pub fn array1incrdim0(&mut self) -> Array1incrdim0W<Loop7cfgSpec> {
        Array1incrdim0W::new(self, 16)
    }
    #[doc = "Bit 17 - Increment Dimension 1"]
    #[inline(always)]
    #[must_use]
    pub fn array1incrdim1(&mut self) -> Array1incrdim1W<Loop7cfgSpec> {
        Array1incrdim1W::new(self, 17)
    }
    #[doc = "Bit 18 - Increment Dimension 2"]
    #[inline(always)]
    #[must_use]
    pub fn array1incrdim2(&mut self) -> Array1incrdim2W<Loop7cfgSpec> {
        Array1incrdim2W::new(self, 18)
    }
    #[doc = "Bit 20 - Increment Dimension 0"]
    #[inline(always)]
    #[must_use]
    pub fn array2incrdim0(&mut self) -> Array2incrdim0W<Loop7cfgSpec> {
        Array2incrdim0W::new(self, 20)
    }
    #[doc = "Bit 21 - Increment Dimension 1"]
    #[inline(always)]
    #[must_use]
    pub fn array2incrdim1(&mut self) -> Array2incrdim1W<Loop7cfgSpec> {
        Array2incrdim1W::new(self, 21)
    }
    #[doc = "Bit 22 - Increment Dimension 2"]
    #[inline(always)]
    #[must_use]
    pub fn array2incrdim2(&mut self) -> Array2incrdim2W<Loop7cfgSpec> {
        Array2incrdim2W::new(self, 22)
    }
    #[doc = "Bit 24 - Increment Dimension 0"]
    #[inline(always)]
    #[must_use]
    pub fn array3incrdim0(&mut self) -> Array3incrdim0W<Loop7cfgSpec> {
        Array3incrdim0W::new(self, 24)
    }
    #[doc = "Bit 25 - Increment Dimension 1"]
    #[inline(always)]
    #[must_use]
    pub fn array3incrdim1(&mut self) -> Array3incrdim1W<Loop7cfgSpec> {
        Array3incrdim1W::new(self, 25)
    }
    #[doc = "Bit 26 - Increment Dimension 2"]
    #[inline(always)]
    #[must_use]
    pub fn array3incrdim2(&mut self) -> Array3incrdim2W<Loop7cfgSpec> {
        Array3incrdim2W::new(self, 26)
    }
    #[doc = "Bit 28 - Increment Dimension 0"]
    #[inline(always)]
    #[must_use]
    pub fn array4incrdim0(&mut self) -> Array4incrdim0W<Loop7cfgSpec> {
        Array4incrdim0W::new(self, 28)
    }
    #[doc = "Bit 29 - Increment Dimension 1"]
    #[inline(always)]
    #[must_use]
    pub fn array4incrdim1(&mut self) -> Array4incrdim1W<Loop7cfgSpec> {
        Array4incrdim1W::new(self, 29)
    }
    #[doc = "Bit 30 - Increment Dimension 2"]
    #[inline(always)]
    #[must_use]
    pub fn array4incrdim2(&mut self) -> Array4incrdim2W<Loop7cfgSpec> {
        Array4incrdim2W::new(self, 30)
    }
}
#[doc = "Loop N Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`loop7cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop7cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Loop7cfgSpec;
impl crate::RegisterSpec for Loop7cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loop7cfg::R`](R) reader structure"]
impl crate::Readable for Loop7cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`loop7cfg::W`](W) writer structure"]
impl crate::Writable for Loop7cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOP7CFG to value 0"]
impl crate::Resettable for Loop7cfgSpec {
    const RESET_VALUE: u32 = 0;
}
