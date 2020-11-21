# Absolute Price Oscillator (APO)
```
use ta_common::traits::Indicator;
use apo_rs::APO;
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
```
### Calculation
ema<sub>n</sub>=EMA<sub>n</sub>(input)  
ema<sub>m</sub>=EMA<sub>m</sub>(input)  
APO=ema<sub>n</sub> - ema<sub>m</sub>  
where  
n=Short Period  
m= Long Period