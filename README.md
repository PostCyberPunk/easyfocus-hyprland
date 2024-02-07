# easyfocus-hyprland
a hyprland window switching helper forked from [sway-easyfocus](https://github.com/edzdez/sway-easyfocus)
### Installation
```bash
git clone --depth 1 https://github.com/PostCyberPunk/easyfocus-hyprland
cargo intall --path easyfocus-hyprland
```
### Usage
1. add $Home/.cargo/bin to your path
2. add a keybinding to your hyprland config
```
bind = $mainMod, P ,exec,easyfocus-hyprland
```
### Configuration
config file is located at $HOME/.config/easyfocus-hyprland/config.yaml
```yaml
chars: 'fjghdkslaemuvitywoqpcbnxz'

window_background_color: '1d1f21'
window_background_opacity: 0.2

label_background_color: '1d1f21'
label_background_opacity: 1.0
label_text_color: 'c5c8c6'

focused_background_color: '285577'
focused_background_opacity: 1.0
focused_text_color: 'ffffff'

font_family: monospace
font_weight: bold
font_size: medium

label_padding_x: 4
label_padding_y: 0
label_border_radius:4
label_margin_x: 4
label_margin_y: 2
```
### TODO
1. ( ) Multi-monitor support
2. ( ) Pinned window support
3. ( ) Better support for floating windows
4. ( ) add env support
5. ( ) pick across workspaces
6. ( ) use css instead of yaml
7. ( ) group window support
8. ( ) swap window support
