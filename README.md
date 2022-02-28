# [![](https://www.giambaj.it/twitch/jchat/img/peepoHappysmall.png)](#) jChat [![GitHub version](https://img.shields.io/badge/release-v2.3.4-blue)](#) [![Website giambaj.it](https://img.shields.io/website-up-down-green-red/https/giambaj.it.svg)](https://www.giambaj.it/twitch/jchat/) [![GitHub license](https://img.shields.io/github/license/giambaJ/jChat)](https://github.com/giambaJ/jChat/blob/main/LICENSE)

**jChat** is an overlay that allows you to show your Twitch chat on screen with OBS, XSplit, and any other streaming software that supports browser sources. It supports your [**BetterTTV**](https://betterttv.com/), [**FrankerFaceZ**](https://www.frankerfacez.com/) and [**7TV**](https://7tv.app/) emotes, always at the best available quality. You have many options to customize your chat, like enabling a smooth animation for new messages, or fading old ones after some time. If you have a chat full of !gamble addicts, you can choose to hide bots and commands messages. It also comes with many fonts and styling options that can be combined as desired.
### The app is up and running on the [**website**](https://www.giambaj.it/twitch/jchat/).

**Local jChat** running jChat locally is supported.  Download the files and point to index.html in https://github.com/giambaJ/jChat/tree/main/v2 - a Twitch Helix API oauth code will be required in credentials.js. (fixme: how to efficiently get helix api oauth tokens)
* example: `file://v2/index.html?channel=yourusername&fade=10&hide_commands=true&size=1&shadow=0`

## Features
- 7TV, BTTV and FFZ emotes support
- Custom channel badges
- Lots of fonts and styling options
- Twitter emojis
- 7TV, BTTV, FFZ, FFZ:AP and Chatterino user badges (on/off)
- Smooth animation (on/off)
- Fade old messages (on/off)
- Hide bots messages (on/off)
- Hide commands messages (on/off)
- !refreshoverlay to make newly added emotes appear (mods only)
