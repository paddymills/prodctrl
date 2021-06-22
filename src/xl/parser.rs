
use TypeMap;

use calamine::DataType;

trait XlParser {
    fn new(xl_file: &str) -> Self;

    fn parse_header(&mut self, cells: &[DataType]);
    fn parse_row(&mut self, row: &[DataType]) -> TypeMap;
}