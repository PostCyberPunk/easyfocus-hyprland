use hyprland::data::Client;
use hyprland::shared::Address;
#[derive(Debug, Clone)]
pub struct HyprWin {
    pub address: Address,
    pub at: (i32, i32),
    pub size: (i32, i32),
}
impl From<Client> for HyprWin {
    fn from(client: Client) -> Self {
        Self {
            address: client.address,
            at: (client.at.0 as i32, client.at.1 as i32),
            size: (client.size.0 as i32, client.size.1 as i32),
        }
    }
}
