use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command; 
use arboard::Clipboard;
use anyhow::{Context, Result};

#[test]
fn clipboard_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new().context("Failed to create clipboard")?;
    let _ = clipboard.set_text("");
    let mut cmd = Command::cargo_bin("materials")?;
    cmd.arg("-D").arg("7").arg("-C").arg("KUSW-P=2");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse decomposed list."));

    Ok(())
}

#[test]
fn output_solution_table() -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new().unwrap();
    let the_string = "ID	Names	Quantity	Valuation 
    1	Silicate Glass	1	1011.34 
    2	Smartfab Units	1	418.3 
    3	Liquid Ozone	1	166.13 
    4	Reactive Gas	1	195.65 
    5	Noble Gas	1	363.2 
    6	Industrial Fibers	1	1199.78 
    7	Supertensile Plastics	1	512.55 
    8	Polyaramids	1	102.93 
    9	Coolant	1	607.45 
    10	Condensates	1	346.7 
    11	Construction Blocks	1	381.78 
    12	Nanites	1	1448.58 
    ";
    let _ = clipboard.set_text(the_string);
    let mut cmd = Command::cargo_bin("material_lp")?;
    cmd.arg("-D").arg("7").arg("-C").arg("FY6-NK=3");
    let expected = "\
.----------------------------------------------------------------------------------------.
| constellation | system | planet | resource              | array_quantity | init_output |
| FY6-NK        | 2R-KLH | 5      | Coolant               | 49.16          | 20.82       |
| FY6-NK        | 2R-KLH | 8      | Noble Gas             | 66             | 41.89       |
| FY6-NK        | 2R-KLH | 9      | Supertensile Plastics | 66             | 25.43       |
| FY6-NK        | 6SB-BN | 4      | Coolant               | 66             | 24.02       |
| FY6-NK        | 6SB-BN | 6      | Coolant               | 66             | 21.36       |
| FY6-NK        | 6SB-BN | 7      | Coolant               | 66             | 21.31       |
| FY6-NK        | B1D-KU | 2      | Liquid Ozone          | 66             | 53.53       |
| FY6-NK        | B1D-KU | 3      | Silicate Glass        | 66             | 12.71       |
| FY6-NK        | KSM-1T | 1      | Noble Gas             | 66             | 36.39       |
| FY6-NK        | KSM-1T | 4      | Liquid Ozone          | 16.83          | 36.88       |
| FY6-NK        | KSM-1T | 7      | Coolant               | 66             | 22.13       |
| FY6-NK        | QFIU-K | 5      | Coolant               | 66             | 25.05       |
| FY6-NK        | YRV-MZ | 6      | Coolant               | 66             | 21.75       |
'----------------------------------------------------------------------------------------'";
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected));

    Ok(())
}
