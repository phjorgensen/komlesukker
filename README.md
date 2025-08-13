# Komlesukker

The plan for this project is to create a CLI that can read from the Nightscout API and output the response. For now it always gets the first entry and writes it to stdout in the format that Waybar expects from their custom modules. I use this as a custom module in my own Waybar, but the plan is to make it more versatile over time.

For now, you can build the package and call the binary in the Waybar module as specified below.

## Build

Clone the repo and run:
```bash
cargo b --release
```

## Waybar module

I've put my binary in `~/.config/waybar/scripts/komlesukker`. If you put it somewhere else, you need to update the paths in the Waybar module.

```json
"custom/bs": {
  "return-type": "json",
  "exec-if": "which ~/.config/waybar/scripts/komlesukker",
  "exec": "~/.config/waybar/scripts/komlesukker",
  "on-click": "~/.config/waybar/scripts/komlesukker",
  "escape": false,
  "tooltip": true,
  "interval": 60,
}
```
