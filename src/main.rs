use std::io;
use std::fmt;       // format in print
use rand::Rng;      // random number
use getopt::prelude::*;

// s eeee fff
struct MyBfloat8 {
    is_neg: bool,   // s
    exponent: i32,  // eeee -8(denormal), -7～6, 7(無限大)
    frac: i32,      // fff  1.fff(normalの場合) or 0.fff(denormalの場合)
}

impl fmt::Display for MyBfloat8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.exponent == 0) && (self.frac == 0) {
            write!(f, "(s:{:#>1b} expo:{:#>04b} fra:{:#>03b})=(-1^{} * 0)",
                self.is_neg as i32, self.exponent, self.frac,
                self.is_neg as i32, 
            )
        } else if self.exponent == 0 {
            write!(f, "(s:{:#>1b} expo:{:#>04b} fra:{:#>03b})=(-1^{} * 0.{:#>03b}b * 2^{})",
                self.is_neg as i32, self.exponent, self.frac,
                self.is_neg as i32, self.frac, (self.exponent - 8 + 1 /* 非正規化数 */), 
            )
        } else if (self.exponent == 7 + 8) && (self.frac == 0) {
            write!(f, "(s:{:#>1b} expo:{:#>04b} fra:{:#>03b})=(-1^{} * Infinite)",
                self.is_neg as i32, self.exponent, self.frac,
                self.is_neg as i32, 
            )
        } else if self.exponent == 7 + 8 {
            write!(f, "(s:{:#>1b} expo:{:#>04b} fra:{:#>03b})=NaN",
                self.is_neg as i32, self.exponent, self.frac,
            )
        } else {
            write!(f, "(s:{:#>1b} expo:{:#>04b} fra:{:#>03b})=(-1^{} * 1.{:#>03b}b * 2^{})",
                self.is_neg as i32, self.exponent, self.frac,
                self.is_neg as i32, self.frac, (self.exponent - 8), 
            )
        }
    }
}

impl Into<f64> for MyBfloat8 {
    fn into(self) -> f64 {
        let mut frac;
        let mut exponent;

        if (self.exponent == 0) && (self.frac == 0) {
            frac = 0.0;
            if self.is_neg {
                -frac
            } else {
                frac
            }
        } else if self.exponent == 0 {
            frac = self.frac as f64 / 8.0;

            exponent = self.exponent - 8 + 1 /* 非正規化数 */;
            while exponent < 0 {
                frac /= 2.0;
                exponent += 1;
            }

            if self.is_neg {
                -frac
            } else {
                frac
            }
        } else if (self.exponent == 7 + 8) && (self.frac == 0) {
            if self.is_neg {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            }
        } else if self.exponent == 7 + 8 {
            f64::NAN
        } else {
            frac = 1.0 + (self.frac as f64 / 8.0);

            exponent = self.exponent - 8;
            while exponent < 0 {
                frac /= 2.0;
                exponent += 1;
            }
            while exponent > 0 {
                frac *= 2.0;
                exponent -= 1;
            }

            if self.is_neg {
                -frac
            } else {
                frac
            }
        }
    }
}

impl From<f64> for MyBfloat8 {
    fn from(fnum: f64) -> Self {
        // 符号(is_neg), frac, exponentに分解
        // 1. 符号
        let is_neg: bool = fnum < 0.0;  // sign
        let mut fnum_abs = fnum.abs();  // frac (最終的に0.0以上、1未満になる)
        let mut exponent = 0;           // 指数部
        let mut is_infinite: bool = false;  // 無限大
        let mut is_underflow: bool = false; // underflow

        // 2. 指数部
        if 2.0 <= fnum_abs {
            while 2.0 <= fnum_abs {
                exponent += 1;
                fnum_abs /= 2.0;
                if 7 <= exponent {   // eeee(-8(denormal), -7～6, 7(無限大))
                    is_infinite = true;
                    break;
                }
            }
        }

        if fnum_abs < 1.0 {
            while fnum_abs < 1.0 {
                if exponent <= -7 {
                    is_underflow = true;    // 0.fff x 2^-7の形式
                    break;
                }
                exponent -= 1;
                fnum_abs *= 2.0;
            }
        }

        // 3. 無限大、非正規化数、正規化数のいずれか
        // bit pattern of frac
        let mut bits = 0;
        if is_infinite {
            bits = 0;               // 無限大の場合、frac=0 (補足：frac≠0はNaN)
        } else {
            if is_underflow == false {
                fnum_abs -= 1.0;    // 正規化数の場合は、整数部1を削除
            }

            // 0x04
            fnum_abs *= 2.0;
            if fnum_abs >= 1.0 {
                bits += 0x04;
                fnum_abs -= 1.0;
            }
            // 0x02
            fnum_abs *= 2.0;
            if fnum_abs >= 1.0 {
                bits += 0x02;
                fnum_abs -= 1.0;
            }
            // 0x01
            fnum_abs *= 2.0;
            if fnum_abs >= 1.0 {
                bits += 0x01;
                fnum_abs -= 1.0;
            }
        }
        if is_underflow {
            exponent = -8;
        }

        // 構造体にセットする
        Self {
            is_neg: is_neg,
            exponent: exponent + 8,
            frac: bits,
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {       // getopt用

    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = Parser::new(&args, "ir:");

    let mut i_flag = false;
    let mut r_flag: f64 = 0.0;
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('i', None) => i_flag = true,
                Opt('r', Some(string)) => r_flag = string.clone().trim().parse()
                    .expect("Please enter a numer after -r"),
                _ => unreachable!(),
            }
        }
    }

    let _args = args.split_off(opts.index());

    if i_flag
    {
        let mut instr = String::new();
        io::stdin().read_line(&mut instr)
            .expect("Failed to read line");
        let fnum: f64 = instr.trim().parse()
            .expect("Please type a number!");

        let bf8 = MyBfloat8::from(fnum);
        println!("{}", bf8);

        let fnum2: f64 = MyBfloat8::into(bf8);
        println!("This bfloat8 is {} as f64", fnum2);
    }
    else if r_flag != 0.0
    {
        let mut rng = rand::thread_rng();
        let mut _cnt: i32;
        let mut rnd_fnum: f64;
        let mut mybfloat8num: MyBfloat8;
        let mut fnum_from_mybload8: f64;

        for _cnt in 1..1000
        {
            rnd_fnum = rng.gen::<f64>() * r_flag;
            mybfloat8num = MyBfloat8::from(rnd_fnum);
            fnum_from_mybload8 = MyBfloat8::into(mybfloat8num);
            println!("{}\t{}", rnd_fnum, fnum_from_mybload8);
        }
    }

    Ok(())
}
