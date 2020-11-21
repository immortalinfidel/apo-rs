use ema_rs::EMA;
use ta_common::traits::Indicator;

pub struct APO {
    long_ema: EMA,
    short_ema: EMA,
    short_period: u32,
    index: u32,
}


impl APO {
    pub fn new(short_period: u32,long_period: u32) -> APO {
        Self {
            long_ema: EMA::new(long_period),
            short_ema: EMA::new(short_period),
            short_period,
            index: 0,
        }
    }
}


impl Indicator<f64, Option<f64>> for APO {
    fn next(&mut self, input: f64) -> Option<f64> {
        let short = self.short_ema.next(input);
        let long = self.long_ema.next(input);
        return if self.index >= self.short_period - 1 {
            let apo = short - long;
            Some(apo)
        } else {
            self.index = self.index + 1;
            None
        };


    }

    fn reset(&mut self) {
        self.short_ema.reset();
        self.long_ema.reset();
        self.index = 0;
    }
}


#[cfg(test)]
mod tests {
    use crate::APO;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut apo = APO::new(2, 5);
        assert_eq!(apo.next(81.59), None);
        assert_eq!(apo.next(81.06), Some(-0.1766666666666623));
        assert_eq!(apo.next(82.87), Some(0.4266666666666623));
        assert_eq!(apo.next(83.00), Some(0.5092592592592524));
        assert_eq!(apo.next(83.61), Some(0.6177777777777749));
        assert_eq!(apo.next(83.15), Some(0.3512757201646082));
        assert_eq!(apo.next(82.84), Some(0.11065843621399551));
        assert_eq!(apo.next(83.99), Some(0.41593049839961793));
        assert_eq!(apo.next(84.55), Some(0.5780064014631705));
        assert_eq!(apo.next(84.36), Some(0.4222440684854689));
        assert_eq!(apo.next(85.53), Some(0.6837982014936586));
        assert_eq!(apo.next(86.54), Some(0.9266328529413386));
        assert_eq!(apo.next(86.89), Some(0.8913443637205063));
        assert_eq!(apo.next(87.77), Some(0.9787592852891009));
        assert_eq!(apo.next(87.29), Some(0.6206827600178713));
    }
}
