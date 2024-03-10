# Forked from [jChat](https://github.com/giambaJ/jChat). Used [jChat-Hyped](https://github.com/ThatHypedPerson/jChat-hyped) to handle new Twitch API login.

To use, create a file called `credentials.js` inside the v2 folder and place inside your Twitch Oauth Token like this:
```js
const credentials = 'YOUR_OAUTH_HERE';
```

To obtain a token, either register a custom application inside Twitch or use one of the many services that provide you with an Oauth token like [https://twitchapps.com/tmi/](https://twitchapps.com/tmi/).

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
