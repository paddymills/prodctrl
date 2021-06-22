
use TypeMap;
use XlParser;

use calamine::{DataType, Range, String};
use std::collections::HashMap;

struct Name<'a>(&'a str);
struct Qty(u32);

struct WorkOrder<'a> {
    header_map: HashMap<, i32>
    data: HashMap<Name, TypeMap>,
}

impl XlParser for WorkOrder {
    fn new(xl_file: &str) -> Self {

        // open workbook
        let mut workbook: Xlsx<_> = open_workbook(xl_file).expect("Cannot open file");

        // Read whole worksheet
        if let Some(Ok(range)) = workbook.worksheet_range("SigmaNest Work Order") {
            let mut iter = range.rows();
            let header = iter.next();
            println!("{:?}", header);

            for row in iter {
                let parsed = self.parse_row(row);

                let name = self.data.get(parsed.get::<Name>())
                match self.data.get_mut(name) {
                    Some(&mut val) => val.update::<Qty>(|&mut v| v.0 += parsed.get::<Qty>().0 ),
                    None => self.data.insert(name, parsed)
                };
            }
        }

        self
    }

    fn parse_header(&mut self, rng: &[DataType]) {
        let header = HashMap<&str, i32>::new();

        let _headervals = Some([
            String("Order No"),
            String("Item Name"),
            String("Qty"),
            String("Material"),
            String("Due Date"),
            String("Customer"),
            String("Dwg Number"),
            String("Priority"),
            String("File Name"),
            String("Remark"),
            String("Item Data1"),
            String("Item Data2"),
            String("Item Data4"),
            String("Process"),
            String("ChargeRefNumber"),
            String("Operation2"),
            String("Operation3"),
            String("Operation4"),
            String("Operation5"),
            String("Operation6"),
            String("Operation7"),
            String("Operation8"),
            String("Operation9"),
            String("Operation10")
        ])

        match rng {
            Some(cols) => {
                let mut index = 0;
                while let String(val) = cols.next() {
                    match val {
                        "Item Name" => header.insert("Name", index),

                        _ => ()
                    };

                    index += 1;
                }
            },
            None => ()
        };
    }

    fn parse_row(&mut self, ) -> TypeMap {

    }

}