
use prodctrl::dialog::critical;

use calamine::{Reader, open_workbook, Xlsx};
use dirs::download_dir;

fn main() -> Result<(), &'static str> {
    let export_file = match download_dir() {
        Some(mut dir) => {
            dir.push("SigmaNest Work Order.xlsx");
            
            if !dir.exists() {
                critical( format!("Could not find export file: {:?}", dir).as_str() );
            }
            
            dir
        },
        None => panic!("Failed to get user's downloads directory")
    };


    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(export_file).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("SigmaNest Work Order") {
        let mut iter = range.rows();
        let header = iter.next();
        println!("{:?}", header);

        for row in iter {
            let name = &row[1];
            let qty = &row[2];
            let grade = &row[3];

            println!("{:>20} | {:>5} | {:}", name, qty, grade);
        }
    }

    Ok(())
}