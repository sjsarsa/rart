use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

use rart::art::fscs_art::FscsArt;
use rart::art::kdfc_art::KdfcArt;
use rart::art::lhs_art::LhsArt;

const N: usize = 1003;

fn main() -> std::io::Result<()> {
    let num = [100, 200, 500, 1000, 2000, 5000, 10000, 15000, 20000];

    let bd2 = [vec![-5000, 5000], vec![-5000, 5000]];
    let bd3 = [vec![-5000, 5000], vec![-5000, 5000], vec![-5000, 5000]];
    let bd4 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];
    let bd5 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];
    let bd6 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];
    let bd7 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];
    let bd8 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];
    let bd9 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];
    let bd10 = [
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
        vec![-5000, 5000],
    ];

    for n in num {
        let test_result_dir = "test-results";
        fs::create_dir_all(test_result_dir)?;

        let s0 = format!("{test_result_dir}/2d-LHS-{n}.txt");
        let s1 = format!("{test_result_dir}/2d-FscsART-{n}.txt");
        let s2 = format!("{test_result_dir}/2d-NaiveKDFC-{n}.txt");
        let s3 = format!("{test_result_dir}/2d-SemiBalKDFC-{n}.txt");
        let s4 = format!("{test_result_dir}/2d-LimBalKDFC-{n}.txt");

        test_lhs(&s0, &bd2, n)?;
        test_fscs_art(&s1, &bd2, n)?;
        test_naive_kdfc(&s2, &bd2, n)?;
        test_semi_bal_kdfc(&s3, &bd2, n)?;
        test_lim_bal_kdfc(&s4, &bd2, n)?;
    }

    Ok(())
}

fn test_fscs_art(file: &str, bd: &[Vec<i32>], point_num: i32) -> std::io::Result<()> {
    let f1 = File::create(file)?;
    let mut out = std::io::BufWriter::new(&f1);

    let mut sum = 0.0;
    for i in 0..N {
        let n1 = Instant::now();
        let mut fscs = FscsArt::new(); // Or FSCS_ART::with_n(10); if you have that constructor
        fscs.test_fscs_art_efficiency(point_num, bd);
        let n2 = Instant::now();
        if i > 2 {
            sum += n2.duration_since(n1).as_nanos() as f64;
            writeln!(out, "{}", n2.duration_since(n1).as_nanos() as f64 / 1e6)?;
        }
    }

    let num = 1000.0;
    let avg_time = sum / num / 1e6;
    println!("fscs {:.6}\t", avg_time);
    write!(out, "{:.6}\t", avg_time)?;
    out.flush()?;
    Ok(())
}

fn test_naive_kdfc(file: &str, bd: &[Vec<i32>], point_num: i32) -> std::io::Result<()> {
    let f1 = File::create(file)?;
    let mut out = std::io::BufWriter::new(&f1);

    let mut sum = 0.0;
    for i in 0..N {
        let n1 = Instant::now();
        let mut kdfc = KdfcArt::with_bound(bd); // Assuming you have a constructor like this
        kdfc.test_naive_kdfc_efficiency(point_num);
        let n2 = Instant::now();
        if i > 2 {
            sum += n2.duration_since(n1).as_nanos() as f64;
            writeln!(out, "{}", n2.duration_since(n1).as_nanos() as f64 / 1e6)?;
        }
    }

    let num = 1000.0;
    let avg_time = sum / num / 1e6;
    println!("kdfc naive {:.6}\t", avg_time);
    write!(out, "{:.6}\t", avg_time)?;
    out.flush()?;
    Ok(())
}

fn test_semi_bal_kdfc(file: &str, bd: &[Vec<i32>], point_num: i32) -> std::io::Result<()> {
    let f1 = File::create(file)?;
    let mut out = std::io::BufWriter::new(&f1);

    let mut sum = 0.0;
    for i in 0..N {
        let n1 = Instant::now();
        let mut kdfc = KdfcArt::with_bound(bd);
        kdfc.test_semi_bal_kdfc_efficiency(point_num);
        let n2 = Instant::now();
        if i > 2 {
            sum += n2.duration_since(n1).as_nanos() as f64;
            writeln!(out, "{}", n2.duration_since(n1).as_nanos() as f64 / 1e6)?;
        }
    }

    let num = 1000.0;
    let avg_time = sum / num / 1e6;
    println!("kdfc semi bal {:.6}\t", avg_time);
    write!(out, "{:.6}\t", avg_time)?;
    out.flush()?;
    Ok(())
}

fn test_lim_bal_kdfc(file: &str, bd: &[Vec<i32>], point_num: i32) -> std::io::Result<()> {
    let f1 = File::create(file)?;
    let mut out = std::io::BufWriter::new(&f1);

    let d = bd.len() as f64;
    let mut back_num = vec![0; point_num as usize];
    back_num[1] = 1;
    (2..point_num as usize).for_each(|i| {
        back_num[i] =
            (1.0 / 2.0 * (d + 1.0 / d).powi(2) * ((i as f64).ln() / 2.0f64.ln())).ceil() as i32;
    });

    let mut sum = 0.0;
    for i in 0..N {
        let n1 = Instant::now();
        let mut kdfc = KdfcArt::with_bound(bd);
        kdfc.test_lim_bal_kdfc_efficiency(point_num, &back_num);
        let n2 = Instant::now();
        if i > 2 {
            sum += n2.duration_since(n1).as_nanos() as f64;
            writeln!(out, "{}", n2.duration_since(n1).as_nanos() as f64 / 1e6)?;
        }
    }

    let num = 1000.0;
    let avg_time = sum / num / 1e6;
    println!("kdfc lim bal {:.6}\t", avg_time);
    write!(out, "{:.6}\t", avg_time)?;
    out.flush()?;
    Ok(())
}

fn test_lhs(file: &str, bd: &[Vec<i32>], point_num: i32) -> std::io::Result<()> {
    let f1 = File::create(file)?;
    let mut out = std::io::BufWriter::new(&f1);

    let mut sum = 0.0;
    for i in 0..N {
        let n1 = Instant::now();
        let mut lhs = LhsArt::new();
        lhs.test_lhs_art_efficiency(point_num as usize, bd);
        let n2 = Instant::now();
        if i > 2 {
            sum += n2.duration_since(n1).as_nanos() as f64;
            writeln!(out, "{}", n2.duration_since(n1).as_nanos() as f64 / 1e6)?;
        }
    }

    let num = 1000.0;
    let avg_time = sum / num / 1e6;
    println!("lhs {:.6}\t", avg_time);
    write!(out, "{:.6}\t", avg_time)?;
    out.flush()?;
    Ok(())
}
