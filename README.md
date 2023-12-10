# Osu Launcher Rs

_rs just means Rust_

## About

Instead of launching Osu! with its native executable, launch Osu! with this executable and have a handful of useful tools auto-load alongside your Osu! client.

Using a simple JSON config that is auto-generated, you can specify various useful Osu! related tools to launch with Osu! like:

| Application               | Description                                                                                                                                                                                                                                                                                                                                                         |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Osu!                      | Need I say more?                                                                                                                                                                                                                                                                                                                                                    |
| Rewind                    | Rewind is a great tool to watch for new replays. It needs to be open with Osu! to detect new replays so you can see why you missed that pesky jump or tricky pattern.                                                                                                                                                                                               |
| Danser                    | Using your own Danser config, render mp4 videos on replay create. Osu Launcher Rs will watch your replays folder for any new replays and auto-render an mp4 video locally using Danser for you to share online. Gone are the days of having to navigate to your replays directory, to load into the Danser GUI, or even worse, manually record your Osu! client. 🤢 |
| Open Tablet Driver daemon | Open Tablet Driver does not load when your machine boots, unless you go out of your way to configure it to do that. This launcher can auto-boot your Open Tablet Driver daemon.                                                                                                                                                                                     |
| FunOrange Osu! Trainer    | Need to generate various difficulties of maps to help you train and get better at Osu!? FunOrange Osu! Trainer needs to be open with Osu to inspect the songs you have selected, of course, this launcher will do that for you!                                                                                                                                     |

Really Osu Launcher Rs is just a lightweight executable that opens more executables for you!

_Note: Windows is only supported_

## WIP features

-   Auto download apps. This launcher has some sensible defaults for config and can try to work out on its own the likely location of where some applications live. However, some apps like Osu! Trainer and Danser can live as simple executable anywhere on your machine, so by default the config path for this is `null`. I will work on a feature that will allow an auto-download of these applications to a directory that sits next to the launcher, so you can worry less about configuration!
-   GUI. A GUI would be cool to change the config. But I will look into this last as I would like all the core features to be implemented first.
