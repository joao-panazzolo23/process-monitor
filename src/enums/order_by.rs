use std::str::FromStr;
#[derive(Debug)]
pub enum OrderBy {
    Cpu,
    Memory,
    Name,
    Pid,
}

impl FromStr for OrderBy {
    type Err = OrderBy;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mem" | "memory" | "m" => Ok(OrderBy::Memory),
            "name" | "n" => Ok(OrderBy::Name),
            "cpu" | "c" => Ok(OrderBy::Cpu),
            "pid" | "id" => Ok(OrderBy::Pid),
            _ => Err(OrderBy::Cpu),
        }
    }
}
