use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct WindowsTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl WindowsTable {
    pub fn from_str(input: &str) -> Result<WindowsTable, WindowsTableError> {
        let lines: Vec<&str> = input.split('\n').collect();
        let mut headers: Vec<String> = Vec::new();
        let mut rows: Vec<Vec<String>> = Vec::new();

        if lines.len() <= 2 {
            return Err(WindowsTableError::InvalidInput);
        }

        let mut handled_headers = false;
        let mut handled_dividers = false;
        for line in lines.iter() {
            // Handle blank lines at the start and end
            if line.is_empty() {
                continue;
            }

            // headers live on the first line
            if ! handled_headers {
                headers = parse_row(line);
                handled_headers = true;
                continue;
            }

            // the second line is a divider
            if ! handled_dividers {
                handled_dividers = true;
                continue;
            }

            rows.push(parse_row(line));
        }

        Ok(WindowsTable {
            headers,
            rows,
        })
    }

    pub fn headers_len(&self) -> usize {
        self.headers.len()
    }

    /// Get the index of the given header, case insensitive
    pub fn header_idx(&self, name: &str) -> Option<usize> {
        for (idx, header_name) in self.headers.iter().enumerate() {
            if header_name.to_lowercase() == name.to_lowercase() {
                return Some(idx);
            }
        }

        None
    }

    /// None if idx is out of bounds
    pub fn get_row(&self, idx: usize) -> Option<&Vec<String>> {
        self.rows.get(idx)
    }

    pub fn rows_len(&self) -> usize {
        self.rows.len()
    }

    /// Get the contents of a cell, based on a header name
    pub fn get_cell(&self, header: &str, row: usize) -> Option<String> {
        let header_idx = self.header_idx(header);

        if header_idx == None {
            return None;
        }

        let the_row = self.rows.get(row);

        if the_row == None {
            return None;
        }

        let the_cell = the_row.unwrap().get(header_idx.unwrap());

        if the_cell == None {
            return None;
        }

        the_cell.cloned()
    }

    pub fn print(&self) -> () {
        println!("{:#?}", self.headers);
        println!("{:#?}", self.rows);
    }
}

fn parse_row(input: &str) -> Vec<String> {
    let split: Vec<&str> = input.split_whitespace().collect();
    let mut items: Vec<String> = Vec::new();

    for item in split {
        items.push(item.to_string());
    }

    items
}

#[derive(Debug, PartialEq, Eq)]
pub enum WindowsTableError {
    InvalidInput,
}

impl Error for WindowsTableError { }

impl Display for WindowsTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            WindowsTableError::InvalidInput => "Invalid input: doesn't look like a windows table",
        };
        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `Get-Command ipconfig`
    static GET_COMMAND_ONE: &str = "\
CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Application     ipconfig.exe                                       10.0.2262… C:\\windows\\system32\\ipconfig.exe
";

    // `Get-Command -Module Microsoft.PowerShell.Core`
    static GET_COMMAND_MODULE_CORE: &str = "

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Cmdlet          Add-History                                        7.4.3.500  Microsoft.PowerShell.Core
Cmdlet          Clear-History                                      7.4.3.500  Microsoft.PowerShell.Core
Cmdlet          Connect-PSSession                                  7.4.3.500  Microsoft.PowerShell.Core
Cmdlet          Debug-Job                                          7.4.3.500  Microsoft.PowerShell.Core
Cmdlet          Disable-ExperimentalFeature                        7.4.3.500  Microsoft.PowerShell.Core
Cmdlet          Disable-PSRemoting                                 7.4.3.500  Microsoft.PowerShell.Core
";

    static GET_COMMAND_NO_ROWS: &str = "
CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
";

    static GET_COMMAND_WEIRD_ROWS: &str = "

CommandType     Name                                               Version    Source
-----------     ----                                               -------    ------
Cmdlet          Add-History                                                   Microsoft.PowerShell.Core
Cmdlet          Clear-History                                      7.4.3.500  Microsoft.PowerShell.Core
Cmdlet          Connect-PSSession                                  7.4.3.500  Microsoft.PowerShell.Core
";

    #[test]
    fn it_fails_with_strange_input() {
        assert_eq!(WindowsTable::from_str("wooooooo").unwrap_err(), WindowsTableError::InvalidInput);
    }

    #[test]
    fn it_parses_a_table_with_no_rows() {
        let table = WindowsTable::from_str(GET_COMMAND_NO_ROWS).unwrap();
        assert_eq!(table.headers_len(), 4);
        assert_eq!(table.rows_len(), 0);
    }

    #[test]
    fn it_parses_a_table_with_one_row() {
        let table = WindowsTable::from_str(GET_COMMAND_ONE).unwrap();
        assert_eq!(
            table.get_row(0),
            Some(
                &vec![
                    "Application".to_string(),
                    "ipconfig.exe".to_string(),
                    "10.0.2262…".to_string(),
                    "C:\\windows\\system32\\ipconfig.exe".to_string(),
                ]
            )
        );
    }

    #[test]
    fn it_parses_a_table_with_a_few_rows() {
        let table = WindowsTable::from_str(GET_COMMAND_MODULE_CORE).unwrap();
        assert_eq!(table.rows_len(), 6);
    }

    #[test]
    fn it_gracefully_fails_to_find_a_header_index() {
        let table = WindowsTable::from_str(GET_COMMAND_ONE).unwrap();
        assert_eq!(table.header_idx("NonExistent"), None);
    }

    #[test]
    fn it_finds_the_index_of_a_header() {
        let table = WindowsTable::from_str(GET_COMMAND_ONE).unwrap();
        assert_eq!(table.header_idx("CommandType"), Some(0));
        assert_eq!(table.header_idx("Name"), Some(1));
        assert_eq!(table.header_idx("Version"), Some(2));
        assert_eq!(table.header_idx("Source"), Some(3));
    }

    #[test]
    fn it_gracefully_fails_to_find_a_cell_non_existent_header() {
        let table = WindowsTable::from_str(GET_COMMAND_MODULE_CORE).unwrap();
        assert_eq!(table.get_cell("NonExistent", 0), None);
    }

    #[test]
    fn it_gracefully_fails_to_find_a_cell_row_out_of_bounds() {
        let table = WindowsTable::from_str(GET_COMMAND_MODULE_CORE).unwrap();
        assert_eq!(table.get_cell("Name", 12), None);
    }

    #[test]
    fn it_gracefully_fails_to_find_a_cell_inconsistent_input() {
        let table = WindowsTable::from_str(GET_COMMAND_WEIRD_ROWS).unwrap();
        assert_eq!(table.get_cell("Source", 0), None);
    }

    #[test]
    fn it_finds_a_cell() {
        let table = WindowsTable::from_str(GET_COMMAND_MODULE_CORE).unwrap();
        assert_eq!(table.get_cell("CommandType", 0), Some("Cmdlet".to_string()));
        assert_eq!(table.get_cell("Name", 0), Some("Add-History".to_string()));
        assert_eq!(table.get_cell("Version", 0), Some("7.4.3.500".to_string()));
        assert_eq!(table.get_cell("Source", 0), Some("Microsoft.PowerShell.Core".to_string()));

        assert_eq!(table.get_cell("CommandType", 1), Some("Cmdlet".to_string()));
        assert_eq!(table.get_cell("Name", 1), Some("Clear-History".to_string()));
        assert_eq!(table.get_cell("Version", 1), Some("7.4.3.500".to_string()));
        assert_eq!(table.get_cell("Source", 1), Some("Microsoft.PowerShell.Core".to_string()));

        assert_eq!(table.get_cell("CommandType", 2), Some("Cmdlet".to_string()));
        assert_eq!(table.get_cell("Name", 2), Some("Connect-PSSession".to_string()));
        assert_eq!(table.get_cell("Version", 2), Some("7.4.3.500".to_string()));
        assert_eq!(table.get_cell("Source", 2), Some("Microsoft.PowerShell.Core".to_string()));

        assert_eq!(table.get_cell("CommandType", 3), Some("Cmdlet".to_string()));
        assert_eq!(table.get_cell("Name", 3), Some("Debug-Job".to_string()));
        assert_eq!(table.get_cell("Version", 3), Some("7.4.3.500".to_string()));
        assert_eq!(table.get_cell("Source", 3), Some("Microsoft.PowerShell.Core".to_string()));

        assert_eq!(table.get_cell("CommandType", 4), Some("Cmdlet".to_string()));
        assert_eq!(table.get_cell("Name", 4), Some("Disable-ExperimentalFeature".to_string()));
        assert_eq!(table.get_cell("Version", 4), Some("7.4.3.500".to_string()));
        assert_eq!(table.get_cell("Source", 4), Some("Microsoft.PowerShell.Core".to_string()));

        assert_eq!(table.get_cell("CommandType", 5), Some("Cmdlet".to_string()));
        assert_eq!(table.get_cell("Name", 5), Some("Disable-PSRemoting".to_string()));
        assert_eq!(table.get_cell("Version", 5), Some("7.4.3.500".to_string()));
        assert_eq!(table.get_cell("Source", 5), Some("Microsoft.PowerShell.Core".to_string()));
    }
}
