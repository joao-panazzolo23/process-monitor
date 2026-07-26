use crate::OrderByUi;

pub enum OrderBy {
    Cpu,
    Memory,
    Name,
    Pid,
}

impl TryFrom<i32> for OrderBy {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OrderBy::Cpu),
            1 => Ok(OrderBy::Memory),
            2 => Ok(OrderBy::Name),
            3 => Ok(OrderBy::Pid),
            _ => Err(()),
        }
    }
}

impl From<OrderByUi> for crate::OrderBy {
    fn from(value: OrderByUi) -> Self {
        match value {
            OrderByUi::Cpu => Self::Cpu,
            OrderByUi::Memory => Self::Memory,
            OrderByUi::Name => Self::Name,
            OrderByUi::Pid => Self::Pid,
        }
    }
}
