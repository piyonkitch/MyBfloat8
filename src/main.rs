use std::io;

fn main() {

    let mut instr = String::new();
    io::stdin().read_line(&mut instr)
        .expect("Failed to read line");
    let fnum: f64 = instr.trim().parse()
        .expect("Please type a number!");

    println!("{}", fnum);


    // sign(is_neg), frac, exponentに分解
    // 1. sign
    let is_neg: bool = fnum < 0.0;  // sign
    let mut fnum_abs = fnum.abs();  // frac (最終的に0.0以上、1未満になる)
    // 2. frac and exponent
    if (0.5 <= fnum_abs) && (fnum_abs < 1.0) {
        let ishift = 0;
        println!("{} {} {}", is_neg, fnum_abs, ishift);

        // bit pattern of frac
        let mut bits = 0;
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
        println!("{}", bits);
    }
}
