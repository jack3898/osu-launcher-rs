# Osu Launcher Rs

_rs just means Rust_

## About

Instead of launching Osu! with its native executable, launch Osu! with this executable and have a handful of useful tools auto-load alongside your Osu! client.

Using a simple JSON config that is auto-generated, you can specify various useful Osu! related tools to launch with Osu! like:

| Application                                                                       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Osu!                                                                              | Need I say more?                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [Rewind](https://github.com/abstrakt8/rewind)                                     | Rewind is a great tool to watch for new replays. It needs to be open with Osu! to detect new replays so you can see why you missed that pesky jump or tricky pattern.                                                                                                                                                                                                                                                                                                                              |
| [Danser](https://github.com/Wieku/danser-go)                                      | Using your own Danser config, render mp4 videos on replay create. Osu Launcher Rs will watch your replays folder for any new replays and auto-render an mp4 video locally using Danser for you to share online. Gone are the days of having to navigate to your replays directory, to load into the Danser GUI, or even worse, manually record your Osu! client. 🤢<br /><br />_Make sure to hold the `R` key while you save your replay for this render process to kick in! It won't by default._ |
| [Open Tablet Driver](https://github.com/OpenTabletDriver/OpenTabletDriver) daemon | Open Tablet Driver does not load when your machine boots, unless you go out of your way to configure it to do that. This launcher can auto-boot your Open Tablet Driver daemon.                                                                                                                                                                                                                                                                                                                    |
| [FunOrange Osu! Trainer](https://github.com/FunOrange/osu-trainer)                | Need to generate various difficulties of maps to help you train and get better at Osu!? FunOrange Osu! Trainer needs to be open with Osu to inspect the songs you have selected, of course, this launcher will do that for you!                                                                                                                                                                                                                                                                    |

Really Osu Launcher Rs is just a lightweight executable that opens more executables for you!

_Note: Windows is only supported_

## Configuration

When you run the launcher for the first time a new JSON file will be created next to the executable. This JSON file contains common configuration options.

Short explanation of the options:

- `path`: The path to the area where the files for the application live.
- `executable_name`: The name of the executable to launch.<br /><br />If you pass an arbitrary executable, the launcher will run it passing in options specific to that program! So be careful.
- `download`: For some applications you can auto-download them. Set this to `false` if you do not want to auto-download the application or set it to `true` if you do want to auto-download the application.<br /><br />When an application is downloaded, it will use the `path` option to extract the download to. Should the application already exist in the `path` directory, it will not be downloaded again.<br /><br />The applications are fetched directly from the official sources linked in the table above and are also included in the config.
- `source`: The source of the application. This is used to download the application.<br /><br />_Make sure to backup any configs you have for the application before updating then delete the existing application folder for Osu! Launcher Rs to re-download._
- `enabled`: Set this to `false` if you do not want to launch the application or set it to `true` if you do want to launch the application.

By default all features are disabled (except for launching Osu!). You will need to enable them one-by-one in the config to your liking.

## Build

1. Install Rust
2. Clone this repo
3. run `cargo build --release`
4. Navigate to your `target/release` directory for the executable.

_I will setup a pipeline to auto-build and publish binaries for download in the future._

## WIP features

- GUI. A GUI would be cool to change the config. But I will look into this last as I would like all the core features to be implemented first.
