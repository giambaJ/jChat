(function ($) {
	$.QueryString = (function (paramsArray) {
		let params = {};
		for (let i = 0; i < paramsArray.length; ++i) {
			let param = paramsArray[i].split('=', 2);
			if (param.length !== 2)
				continue;
			params[param[0]] = decodeURIComponent(param[1].replace(/\+/g, " "));
		}
		return params;
	})(window.location.search.substr(1).split('&'))
})(jQuery);

Chat = {
	info: {
		channel: null,
		animate: ('animate' in $.QueryString ? ($.QueryString.animate.toLowerCase() === 'true') : false),
		showBots: ('bots' in $.QueryString ? ($.QueryString.bots.toLowerCase() === 'true') : false),
		hideCommands: ('hide_commands' in $.QueryString ? ($.QueryString.hide_commands.toLowerCase() === 'true') : false),
		hideBadges: ('hide_badges' in $.QueryString ? ($.QueryString.hide_badges.toLowerCase() === 'true') : false),
		fade: ('fade' in $.QueryString ? parseInt($.QueryString.fade) : false),
		size: ('size' in $.QueryString ? parseInt($.QueryString.size) : 3),
		font: ('font' in $.QueryString ? parseInt($.QueryString.font) : 0),
		stroke: ('stroke' in $.QueryString ? parseInt($.QueryString.stroke) : false),
		shadow: ('shadow' in $.QueryString ? parseInt($.QueryString.shadow) : false),
		smallCaps: ('small_caps' in $.QueryString ? ($.QueryString.small_caps.toLowerCase() === 'true') : false),
		emotes: {},
		badges: {},
		userBadges: {},
		ffzapBadges: null,
		bttvBadges: null,
		seventvBadges: [],
		chatterinoBadges: null,
		cheers: {},
		blockedUsers: ('block' in $.QueryString ? $.QueryString.block.toLowerCase().split(',') : false),
		bots: ['streamelements', 'streamlabs', 'nightbot', 'moobot', 'fossabot'],
		nicknameColor: ('cN' in $.QueryString ? $.QueryString.cN : false)
	},
	loadEmotes: function (channelID) {
		Chat.info.emotes = {};
		['emotes/global', 'users/twitch/' + encodeURIComponent(channelID)].forEach(endpoint => {
			$.getJSON('https://api.betterttv.net/3/cached/frankerfacez/' + endpoint).done(function (res) {
				res.forEach(emote => {
					if (emote.images['4x']) {
						var imageUrl = emote.images['4x'];
						var upscale = false;
					} else {
						var imageUrl = emote.images['2x'] || emote.images['1x'];
						var upscale = true;
					}
					Chat.info.emotes[emote.code] = {
						id: emote.id,
						image: imageUrl,
						upscale: upscale,
						regex: new RegExp("(?<!\\S)(" + escapeRegExp(emote.code) + ")(?!\\S)", 'g')
					};
					Chat.info.emotes[emote.code].html = calculateHTMLForEmote(Chat.info.emotes[emote.code]);
				});
			});
		});
		['emotes/global', 'users/twitch/' + encodeURIComponent(channelID)].forEach(endpoint => {
			$.getJSON('https://api.betterttv.net/3/cached/' + endpoint).done(function (res) {
				if (!Array.isArray(res)) {
					res = res.channelEmotes.concat(res.sharedEmotes);
				}
				res.forEach(emote => {
					Chat.info.emotes[emote.code] = {
						id: emote.id,
						image: 'https://cdn.betterttv.net/emote/' + emote.id + '/3x',
						regex: new RegExp("(?<!\\S)(" + escapeRegExp(emote.code) + ")(?!\\S)", 'g'),
						zeroWidth: ["5e76d338d6581c3724c0f0b2", "5e76d399d6581c3724c0f0b8", "567b5b520e984428652809b6", "5849c9a4f52be01a7ee5f79d", "567b5c080e984428652809ba", "567b5dc00e984428652809bd", "58487cc6f52be01a7ee5f205", "5849c9c8f52be01a7ee5f79e"].includes(emote.id)
					};
					Chat.info.emotes[emote.code].html = calculateHTMLForEmote(Chat.info.emotes[emote.code]);
				});
			});
		});
		$.getJSON('https://7tv.io/v3/emote-sets/global').done((res) => {
			res?.emotes?.forEach(emote => {
				const emoteData = emote.data.host.files.pop();
				Chat.info.emotes[emote.name] = {
					id: emote.id,
					regex: new RegExp("(?<!\\S)(" + escapeRegExp(emote.name) + ")(?!\\S)", 'g'),
					image: `https:${emote.data.host.url}/${emoteData.name}`,
					zeroWidth: emote.data.flags == 256
				}
				Chat.info.emotes[emote.name].html = calculateHTMLForEmote(Chat.info.emotes[emote.name]);
			})
		})
		$.getJSON('https://7tv.io/v3/users/twitch/' + encodeURIComponent(channelID)).done((res) => {
			res?.emote_set?.emotes?.forEach(emote => {
				const emoteData = emote.data.host.files.pop();
				Chat.info.emotes[emote.name] = {
					id: emote.id,
					image: `https:${emote.data.host.url}/${emoteData.name}`,
					regex: new RegExp("(?<!\\S)(" + escapeRegExp(emote.name) + ")(?!\\S)", 'g'),
					zeroWidth: emote.data.flags == 256
				}
				Chat.info.emotes[emote.name].html = calculateHTMLForEmote(Chat.info.emotes[emote.name]);
			})
		})
	},
	load: function (callback) {
		// Load CSS
		let size = sizes[Chat.info.size - 1];
		let font = fonts[Chat.info.font];

		appendCSS('size', size);
		appendCSS('font', font);

		if (Chat.info.stroke && Chat.info.stroke > 0) {
			let stroke = strokes[Chat.info.stroke - 1];
			appendCSS('stroke', stroke);
		}
		if (Chat.info.shadow && Chat.info.shadow > 0) {
			let shadow = shadows[Chat.info.shadow - 1];
			appendCSS('shadow', shadow);
		}
		if (Chat.info.smallCaps) {
			appendCSS('variant', 'SmallCaps');
		}
		if (Chat.info.invert) {
			appendCSS('variant', 'invert');
		}

		TwitchOAuth().done(function (res) {

			// Set the global client_id to be used in future requests
			client_id = res.client_id;

			const channelIDPromise = TwitchAPI("/users?login=" + Chat.info.channel).then(function (res) {
				Chat.info.channelID = res.data[0].id;
				return loadChannelInformation();
			});

			// Load badges
			const twitchGlobalBadgesPromise = TwitchAPI('/chat/badges/global').done(function (res) {
				res?.data.forEach(badge => {
					badge?.versions.forEach(version => {
						Chat.info.badges[badge.set_id + ':' + version.id] = version.image_url_4x;
					});
				});
			});

			if (!Chat.info.hideBadges) {
				$.getJSON('https://api.ffzap.com/v1/supporters')
					.done(function (res) {
						Chat.info.ffzapBadges = res;
					})
					.fail(function () {
						Chat.info.ffzapBadges = [];
					});
				$.getJSON('https://api.betterttv.net/3/cached/badges')
					.done(function (res) {
						Chat.info.bttvBadges = res;
					})
					.fail(function () {
						Chat.info.bttvBadges = [];
					});

				$.getJSON('https://api.chatterino.com/badges')
					.done(function (res) {
						Chat.info.chatterinoBadges = res.badges;
					})
					.fail(function () {
						Chat.info.chatterinoBadges = [];
					});
			}

			Promise.all([channelIDPromise, twitchGlobalBadgesPromise])
				.then(() => {
					console.log('All data loaded successfully.');
					callback(true);
				})
				.catch(error => {
					console.error('An error occurred:', error);
				});
		});
	},
	loadUserBadges: function (nick, userId) {
		Chat.info.userBadges[nick] = [];
	},
	write: function (nick, info_new_message, new_message) {
		if (info_new_message) {
			var $chatLine = $('<div></div>');
			$chatLine.addClass('chat_line');
			$chatLine.attr('data-nick', nick);
			$chatLine.attr('data-time', Date.now());
			$chatLine.attr('data-id', info_new_message.id);
			var $userInfo = $('<span></span>');
			$userInfo.addClass('user_info');
			if (Chat.info.hideBadges) {
				if (typeof (info_new_message.badges) === 'string') {
					info_new_message.badges.split(',').forEach(badge => {
						var $badge = $('<img/>'); $badge.addClass('badge');
						badge = badge.split('/');
						$badge.attr('src', Chat.info.badges[badge[0] + ':' + badge[1]]);
						$userInfo.append($badge);
					});
				}
			} else {
				var badges = [];
				const priorityBadges = ['predictions', 'admin', 'global_mod', 'staff', 'twitchbot', 'broadcaster', 'moderator', 'vip'];
				if (typeof (info_new_message.badges) === 'string') {
					info_new_message.badges.split(',').forEach(badge => {
						badge = badge.split('/');
						var priority = (priorityBadges.includes(badge[0]) ? true : false);
						badges.push({ description: badge[0], url: Chat.info.badges[badge[0] + ':' + badge[1]], priority: priority });
					});
				}
				var $modBadge;
				badges.forEach(badge => {
					if (badge.priority) {
						var $badge = $('<img/>');
						$badge.addClass('badge');
						$badge.attr('src', badge.url);
						if (badge.description === 'moderator')
							$modBadge = $badge;
						$userInfo.append($badge);
					}
				});
				if (Chat.info.userBadges[nick]) {
					Chat.info.userBadges[nick].forEach(badge => {
						var $badge = $('<img/>');
						$badge.addClass('badge');
						if (badge.color)
							$badge.css('background-color', badge.color);
						if (badge.description === 'Bot' && info_new_message.mod === '1') {
							$badge.css('background-color', 'rgb(0, 173, 3)');
							$modBadge.remove();
						}
						$badge.attr('src', badge.url);
						$userInfo.append($badge);
					});
				}
				badges.forEach(badge => {
					if (!badge.priority) {
						var $badge = $('<img/>');
						$badge.addClass('badge');
						$badge.attr('src', badge.url);
						$userInfo.append($badge);
					}
				});
			}
			var $username = $('<span></span>');
			$username.addClass('nick');
			if (Chat.info.nicknameColor)
				var color = Chat.info.nicknameColor;
			else {
				if (typeof (info_new_message.color) === 'string') {
					if (tinycolor(info_new_message.color).getBrightness() <= 50)
						var color = tinycolor(info_new_message.color).lighten(30);
					else
						var color = info_new_message.color;
				} else {
					const twitchColors = ["#FF0000", "#0000FF", "#008000", "#B22222", "#FF7F50", "#9ACD32", "#FF4500", "#2E8B57", "#DAA520", "#D2691E", "#5F9EA0", "#1E90FF", "#FF69B4", "#8A2BE2", "#00FF7F"]; var color = twitchColors[nick.charCodeAt(0) % 15];
				}
			}
			$username.css('color', color);
			$username.html(info_new_message['display-name'] ? info_new_message['display-name'] : nick);
			$userInfo.append($username);
			var $message = $('<span></span>');
			$message.addClass('message');
			if (/^\x01ACTION.*\x01$/.test(new_message)) {
				$message.css('color', color);
				new_message = new_message.replace(/^\x01ACTION/, '').replace(/\x01$/, '').trim();
				$userInfo.append('<span>&nbsp;</span>');
			} else {
				$userInfo.append('<span class="colon">:</span>');
			}
			$chatLine.append($userInfo);
			var replacements = {};
			if (typeof (info_new_message.emotes) === 'string') {
				info_new_message.emotes.split('/').forEach(emoteData => {
					var twitchEmote = emoteData.split(':');
					var indexes = twitchEmote[1].split(',')[0].split('-');
					var emojis = new RegExp('[\u1000-\uFFFF]+', 'g');
					var aux = new_message.replace(emojis, ' ');
					var emoteCode = aux.substr(indexes[0], indexes[1] - indexes[0] + 1);
					replacements[emoteCode] = {
						html: '<img class="emote" src="https://static-cdn.jtvnw.net/emoticons/v2/' + twitchEmote[0] + '/default/dark/3.0" />',
						regex: new RegExp("(?<!\\S)(" + escapeRegExp(emoteCode) + ")(?!\\S)", 'g')
					};
				});
			}

			Object.entries(Chat.info.emotes).forEach(emote => {
				if (emote[1].regex.test(new_message)) {
					replacements[emote[0]] = emote[1];
				}
			});

			new_message = escapeHtml(new_message);
			if (info_new_message.bits && parseInt(info_new_message.bits) > 0) {
				var bits = parseInt(info_new_message.bits);
				var parsed = false;
				for (cheerType of Object.entries(Chat.info.cheers)) {
					var regex = new RegExp(cheerType[0] + "\\d+\\s*", 'ig');
					if (regex.test(new_message)) {
						new_message = new_message.replace(regex, '');
						if (!parsed) {
							var closest = 1;
							for (cheerTier of Object.keys(cheerType[1]).map(Number).sort((a, b) => a - b)) {
								if (bits >= cheerTier)
									closest = cheerTier;
								else
									break;
							}
							new_message = '<img class="cheer_emote" src="' + cheerType[1][closest].image + '" /><span class="cheer_bits" style="color: ' + cheerType[1][closest].color + ';">' + bits + '</span> ' + new_message;
							parsed = true;
						}
					}
				}
			}

			if (Object.keys(replacements).length > 0) {
				// Iterate over the replacements object
				Object.entries(replacements).forEach(([_, value]) => {
					new_message = new_message.replace(value.regex, value.html);
				});
			}

			//new_message = twemoji.parse(new_message);
			$message.html(new_message);
			messageNodes = $message.children();
			messageNodes.each(function (i) {
				if (i != 0 && $(this).data('zw') && ($(messageNodes[i - 1]).hasClass('emote') || $(messageNodes[i - 1]).hasClass('emoji')) && !$(messageNodes[i - 1]).data('zw')) {
					var $container = $('<span></span>');
					$container.addClass('zero-width_container');
					$(this).addClass('zero-width');
					$(this).before($container);
					$container.append(messageNodes[i - 1], this);
				}
			});
			$message.html($message.html().trim());
			$chatLine.append($message);
			$('#chat_container').append($chatLine.wrap('<div>').parent().html());

			// Checking if chat is longer than viewport
			var divHeight = $("#chat_container").height();
			while (divHeight > 570) {
				$('.chat_line').eq(0).remove();
				divHeight = $("#chat_container").height();
			}
		}
	},
	clearChat: function (nick) {
		$('.chat_line[data-nick=' + nick + ']').remove();
	},
	clearMessage: function (id) {
		$('.chat_line[data-id=' + id + ']').remove();
	},
	connect: function (channel) {
		Chat.info.channel = channel;
		var title = $(document).prop('title');
		$(document).prop('title', title + Chat.info.channel);
		Chat.load(function () {

			if (Chat.info.fade) {
				setInterval(function () {
					var messageTime = $('.chat_line').eq(0).data('time');
					if ((Date.now() - messageTime) / 1000 >= Chat.info.fade) {
						$('.chat_line').eq(0).fadeOut(function () {
							$(this).remove();
						});
					}
				}, 5000)
			}

			console.log('jChat: Connecting to IRC server...');
			var socket = new ReconnectingWebSocket('wss://irc-ws.chat.twitch.tv', 'irc', { reconnectInterval: 2000 });
			socket.onopen = function () {
				console.log('jChat: Connected');
				socket.send('PASS blah\r\n');
				socket.send('NICK justinfan' + Math.floor(Math.random() * 99999) + '\r\n');
				socket.send('CAP REQ :twitch.tv/commands twitch.tv/tags\r\n');
				socket.send('JOIN #' + Chat.info.channel + '\r\n');
			};
			socket.onclose = function () {
				console.log('jChat: Disconnected');
			};
			socket.onmessage = function (data) {
				data.data.split('\r\n').forEach(line => {
					if (!line) return;
					var message = window.parseIRC(line);
					if (!message.command) return;
					switch (message.command) {
						case "PING": socket.send('PONG ' + message.params[0]);
							return;
						case "JOIN": console.log('jChat: Joined channel #' + Chat.info.channel);
							return;
						case "CLEARMSG":
							if (message.tags)
								Chat.clearMessage(message.tags['target-msg-id']);
							return;
						case "CLEARCHAT":
							if (message.params[1])
								Chat.clearChat(message.params[1]);
							return;
						case "PRIVMSG": if (message.params[0] !== '#' + channel || !message.params[1]) return;
							var nick = message.prefix.split('@')[0].split('!')[0];
							if (message.params[1].toLowerCase() === "!refreshoverlay" && typeof (message.tags.badges) === 'string') {
								var flag = false;
								message.tags.badges.split(',').forEach(badge => {
									badge = badge.split('/');
									if (badge[0] === "moderator" || badge[0] === "broadcaster") {
										flag = true;
										return;
									}
								});
								if (flag) {
									Chat.loadEmotes(Chat.info.channelID);
									console.log('jChat: Refreshing emotes...');
									return;
								}
							}
							if (message.params[1].toLowerCase() === "!reloadchat" && typeof (message.tags.badges) === 'string') {
								var flag = false;
								message.tags.badges.split(',').forEach(badge => {
									badge = badge.split('/');
									if (badge[0] === "moderator" || badge[0] === "broadcaster") {
										flag = true;
										return;
									}
								});
								if (flag) {
									location.reload();
								}
							}
							if (Chat.info.hideCommands) {
								if (/^!.+/.test(message.params[1]))
									return;
							}
							if (!Chat.info.showBots) {
								if (Chat.info.bots.includes(nick))
									return;
							}
							if (Chat.info.blockedUsers) {
								if (Chat.info.blockedUsers.includes(nick))
									return;
							}
							if (!Chat.info.hideBadges) {
								if (Chat.info.bttvBadges && Chat.info.seventvBadges && Chat.info.chatterinoBadges && Chat.info.ffzapBadges && !Chat.info.userBadges[nick])
									Chat.loadUserBadges(nick, message.tags['user-id']);
							}

							Chat.write(nick, message.tags, message.params[1]);
							return;
					}
				});
			};
		});
	}
};

$(document).ready(function () {
	Chat.connect($.QueryString.channel ? $.QueryString.channel.toLowerCase() : 'gioffyna');
});