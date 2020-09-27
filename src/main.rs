use std::io;

// s eeee fff
fn main() {

    let mut instr = String::new();
    io::stdin().read_line(&mut instr)
        .expect("Failed to read line");
    let fnum: f64 = instr.trim().parse()
        .expect("Please type a number!");

    println!("{}", fnum);


    // 符号(is_neg), frac, exponentに分解
    // 1. 符号
    let is_neg: bool = fnum < 0.0;  // sign
    let mut fnum_abs = fnum.abs();  // frac (最終的に0.0以上、1未満になる)
    let mut exponent = 0;           // 指数部
    let mut is_infinite: bool = false;  // 無限大
    let mut is_underflow: bool = false; // underflow

    // 2. 指数部
    if 1.0 <= fnum_abs {
        while 1.0 <= fnum_abs {
            exponent += 1;
            fnum_abs /= 2.0;
            if 7 <= exponent {   // eeee(-8(denormal), -7～6, 7(無限大))
                is_infinite = true;
                break;
            }
        }
    }

    if fnum_abs < 0.5 {
        while fnum_abs <= 0.5 {
            exponent -= 1;
            fnum_abs *= 2.0;
            if exponent <= -8 {
                is_infinite = true;
                break;
            }
        }
    }

    println!("exponent is {}", exponent);

    // 3. 無限大、非正規化数、正規化数のいずれか
    let mut bits = 0;
    println!("{} {}", is_neg, fnum_abs);
    // bit pattern of frac
    let mut bits = 0;
    if is_infinite {
        bits = 0;           // 無限大の場合、frac=0
    } else {
        fnum_abs -= 0.5;        // 0.1b は削除
        // 0x04
        fnum_abs *= 2.0;
        if fnum_abs >= 0.5 {
            bits += 0x04;
            fnum_abs -= 0.5;
        }
        // 0x02
        fnum_abs *= 2.0;
        if fnum_abs >= 0.5 {
            bits += 0x02;
            fnum_abs -= 0.5;
        }
        // 0x01
        fnum_abs *= 2.0;
        if fnum_abs >= 0.5 {
            bits += 0x01;
            fnum_abs -= 0.5;
        }
    }
    // 表示
    println!("{:#>1b} {:#>04b} {:#>03b}", is_neg as i32, (exponent + 8), bits);
}
