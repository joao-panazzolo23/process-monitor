pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub cpu_usage: f32,
    pub memory_usage: f64,
    pub father_pid: Option<u32>,
    pub status: String,
}
