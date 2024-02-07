use hyprland::data::Client;
use hyprland::shared::Address;
#[derive(Debug, Clone)]
pub struct HyprWin {
    pub address: Address,
    pub at: (i16, i16),
    pub size: (i16, i16),
}
impl From<Client> for HyprWin {
    fn from(client: Client) -> Self {
        Self {
            address: client.address,
            at: client.at,
            size: client.size,
        }
    }
}
