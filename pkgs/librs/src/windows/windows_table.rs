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
        for (i, line) in lines.iter().enumerate() {
            dbg!(i);
            dbg!(line);
            // Handle blank lines at the start and end
            if line.is_empty() {
                continue;
            }

            // headers live on the first line
            if ! handled_headers {
                println!("headers line: {}", line);
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
    }

    /// None if idx is out of bounds
    pub fn get_row(&self, idx: usize) -> Option<&Vec<String>> {
        self.rows.get(idx)
    }

    pub fn rows_len(&self) -> usize {
        self.rows.len()
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

    #[test]
    fn it_fails_with_strange_input() {
        assert_eq!(WindowsTable::from_str("wooooooo").unwrap_err(), WindowsTableError::InvalidInput);
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
}
