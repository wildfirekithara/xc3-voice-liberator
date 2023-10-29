# Voice Liberator mod for Xenoblade 3
This mod loads Future Redeemed character VOs in the Base Game, and vice versa.
This works with any and no DLC packs installed/loaded, but is really only useful with DLC4 at the very least.

> **Important**: While this mod does not deal with persistent data, you are still using it at your own risk. I am
not responsible for anything that could happen to your saves, game, console, account, etc.

## Usage

#### Switch
1. Download the latest version of the mod from the [Releases](https://github.com/wildfirekithara/xc3-voice-liberator/releases/latest) page.
2. Extract the archive to root of your SD card.

#### Ryujinx
1. Download the latest version of the mod from the [Releases](https://github.com/wildfirekithara/xc3-voice-liberator/releases/latest) page.
2. Open Ryujinx, then right-click on the game and select "Open Atmosphere Mods Directory".
3. From the archive, extract the `exefs` and `romfs` directory into the folder you opened.

#### Replacing files

After installing the mod, you can place your custom files in the `/atmosphere/contents/010074f013262000/romfs/` folder.  
This mod enables loading files that would normally be overwritten by the game's archives.

## Build instructions
To build the project, install [Rust](https://rustup.rs/) and run
```sh
./build.sh
```

## Credits
The scaffolding, build script, and skyline configuration for this project was borrowed from [xc3-file-loader](https://github.com/RoccoDev/xc3-file-loader).

## License
This mod is distributed under the terms of the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html). See [COPYING](COPYING) for details.
