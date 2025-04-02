use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use rart::art::fscs_art::FscsArt;
use rart::art::kdfc_art::KdfcArt;
use rart::art::lhs_art::LhsArt;
use rart::art::rt::Rt;
use rart::fault::fault_zone::FaultZone;
use rart::fault::fault_zone_block::FaultZoneBlock;
use rart::fault::fault_zone_point_square::FaultZonePointSquare;
use rart::fault::fault_zone_strip::FaultZoneStrip;

fn fix_rate_test(
    result_summary_csv: &mut ResultCsvWriter,
    area_size: f32,
    space_bounds: &[Vec<i32>],
    shape: &str,
    n_iter: u16,
    n_repeat_fault_zone: u16,
) -> std::io::Result<()> {
    let mut fzb: FaultZone;

    let mut random: Rt;
    let mut lhs: LhsArt;
    let mut kdfc: KdfcArt;
    let mut fscs: FscsArt;

    let mut back_num = vec![0; (100.0 * (1.0 / area_size) as f64) as usize]; // Corrected size calculation
                                                                             // let mut back_num = vec![0; 100 * (1.0 / area_size) as usize];
    back_num[0] = 1;
    back_num[1] = 1;
    let d = space_bounds.len() as f64;
    (2..back_num.len()).for_each(|i| {
        back_num[i] =
            (1.0 / 2.0 * (d + 1.0 / d).powi(2) * ((i as f64).ln() / 2.0f64.ln())).ceil() as i32;
    });

    let lhs_partitions = 1000.0f64.powf(1.0 / space_bounds.len() as f64) as usize;

    let mut test_case_counts = BTreeMap::from([
        ("random (uniform)", 0.0),
        ("lhs", 0.0),
        ("fscs", 0.0),
        ("naive kdfc", 0.0),
        ("semi-bal kdfc", 0.0),
        ("lim-bal kdfc", 0.0),
    ]);

    for _i in 0..n_iter {
        // println!("generating {shape} {_i}");
        fzb = match shape {
            "block" => FaultZone::Block(FaultZoneBlock::new(space_bounds, area_size as f64)),
            "strip" => FaultZone::Strip(FaultZoneStrip::new(space_bounds, area_size as f64, 0.9)),
            "point" => {
                FaultZone::PointSquare(FaultZonePointSquare::new(space_bounds, area_size as f64))
            }
            _ => panic!("Invalid shape"),
        };

        for _j in 0..n_repeat_fault_zone {
            // println!("random (uniform)");
            random = Rt::new();
            let test_cases = random.test_rt_effectiveness(space_bounds, &fzb);
            *test_case_counts.entry("random (uniform)").or_default() += test_cases as f64;
            // write!(result_raw_buf, "{}\t", test_cases)?;

            // println!("lhs");
            lhs = LhsArt::default();
            lhs.n_partitions = lhs_partitions;
            lhs.exhaustive = true;
            lhs.input_domain = space_bounds;
            let test_cases = lhs.test_lhs_art_effectiveness(&fzb);
            *test_case_counts.entry("lhs").or_default() += test_cases as f64;
            // write!(result_raw_buf, "{}\t", test_cases)?;

            // println!("fscs");
            // fscs = FscsArt::with_n(10);
            // let test_cases = fscs.test_fscs_art_effectiveness(space_bounds, &fzb);
            // *test_case_counts.entry("fscs").or_default() += test_cases as f64;
            // write!(result_raw_buf, "{}\t", test_cases)?;

            // // println!("naive kdfc");
            // kdfc = KdfcArt::with_bound(space_bounds);
            // kdfc.test_naive_kdfc_effectiveness(&fzb);
            // *test_case_counts.entry("naive kdfc").or_default() += kdfc.size as f64;
            // // write!(result_raw_buf, "{}\t", kdfc.size)?;

            // // println!("semi-bal kdfc");
            // kdfc = KdfcArt::with_bound(space_bounds);
            // kdfc.test_semi_bal_kdfc_effectiveness(&fzb);
            // *test_case_counts.entry("semi-bal kdfc").or_default() += kdfc.size as f64;
            // // write!(result_raw_buf, "{}\t", kdfc.size)?;

            // // println!("lim-bal kdfc");
            // kdfc = KdfcArt::with_bound(space_bounds);
            // kdfc.test_lim_bal_kdfc_effectiveness(&fzb, &back_num);
            // *test_case_counts.entry("lim-bal kdfc").or_default() += kdfc.size as f64;
            // // write!(result_raw_buf, "{}\t", kdfc.size)?;

            // writeln!(result_raw_buf)?;
            // println!("{} {} {} {}", num1, num2, num3, num4);
            // result_raw_buf.flush()?;
            // println!("{shape} {_i}, {_j}\r");
        }
    }

    let n = (n_iter * n_repeat_fault_zone) as f64;
    let s = 1.0 / area_size as f64 / 100.0; // Corrected calculation

    test_case_counts
        .iter()
        .map(|(k, v)| (k, v / n / s))
        .for_each(|(k, v)| {
            println!("  {k:20}{v:.4}");
            let _ = result_summary_csv.write(k, v, shape, area_size as f64, space_bounds.len() as u32);
        });

    result_summary_csv.buf.flush()?;
    Ok(())
}

/// Returns a 2d vector where the inner elements are bounds for space
/// dimensions. The bounds are fixed to [-5000, 5000] for all dimensions.
/// # Arguments
///
/// * `n_dims` - Number of dimensions for the space
///
/// # Examples
///
/// ```
/// let bounds = generate_bounds(2);
/// assert_eq!(bounds, vec![vec![-5000, 5000], vec![-5000, 5000]]);
fn generate_bounds(n_dims: u32) -> Vec<Vec<i32>> {
    let mut bounds = Vec::new();
    for _ in 0..n_dims {
        bounds.push(vec![-5000, 5000]);
    }
    bounds
}

struct ResultCsvWriter<'this> {
    buf: BufWriter<&'this File>,
}

impl<'this> ResultCsvWriter<'this> {
    fn new(file: &'this File) -> std::io::Result<Self> {
        let buf = BufWriter::new(file);
        Ok(Self { buf })
    }

    fn init(&mut self) -> std::io::Result<()> {
        writeln!(
            self.buf,
            "algorithm,efficiency_mean,shape,area_size,space_dim"
        )?;
        Ok(())
    }

    fn write(
        &mut self,
        algorithm: &str,
        efficiency_mean: f64,
        shape: &str,
        area_size: f64,
        space_dim: u32,
    ) -> std::io::Result<()> {
        writeln!(
            self.buf,
            "{algorithm},{efficiency_mean},{shape},{area_size},{space_dim}",
        )?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let area_sizes = vec![
        0.01f32, 0.005f32, 0.002f32, 0.001f32, 0.0005f32, 0.0002f32, 0.0001f32,
    ];

    let space_dims = 6;

    let n_iter = 1000;
    let n_repeat_fault_zone = 10;

    let shapes = ["block", "strip", "point"];

    let cur_time = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let test_result_dir = Path::new("test-results/efficiency/");
    fs::create_dir_all(test_result_dir)?;
    let result_file_summary =
        File::create(test_result_dir.join(format!("summary-{}.csv", cur_time)))?;
    let mut result_csv_writer = ResultCsvWriter::new(&result_file_summary)?;
    result_csv_writer.init()?;

    for area_size in area_sizes {
        println!("{area_size}");

        for shape in shapes.iter() {
            println!("{shape}");
            // let result_file_raw  = File::create(format!("{result_dir}/raw-{shape}-{area_size}-{space_bounds:?}.txt"))?;
            // let mut result_raw_buf = BufWriter::new(&result_file_raw);
            fix_rate_test(
                &mut result_csv_writer,
                area_size,
                &generate_bounds(space_dims),
                shape,
                n_iter,
                n_repeat_fault_zone,
            )?;
        }
    }

    Ok(())
}
