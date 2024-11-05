pub fn format_irating(irating: u32) -> String {
    format!("{:.1}k", irating as f32 / 1000.0)
}
