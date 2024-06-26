use hyprland::data::{Client, Monitor};
use hyprland::shared::{Address, HyprDataActive};
#[derive(Debug, Clone)]
pub struct HyprWin {
    pub address: Address,
    pub at: (i32, i32),
    pub size: (i32, i32),
    pub focused: bool,
}
impl From<Client> for HyprWin {
    fn from(client: Client) -> Self {
        let monitor = Monitor::get_active().unwrap();
        Self {
            address: client.address,
            at: (
                (client.at.0 - (monitor.x as i16)) as i32,
                (client.at.1 - (monitor.y as i16)) as i32,
            ),
            size: (client.size.0 as i32, client.size.1 as i32),
            focused: client.focus_history_id == 0,
        }
    }
}
