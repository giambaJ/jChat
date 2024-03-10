function appendCSS(type, name) {
	$("<link/>", {
		rel: "stylesheet",
		type: "text/css",
		class: `chat_${type}`,
		href: `styles/${type}_${name}.css`
	}).appendTo("head");
}

function escapeRegExp(string) {
	return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function escapeHtml(message) {
	return message.replace(/&/g, "&amp;").replace(/(<)(?!3)/g, "&lt;").replace(/(>)(?!\()/g, "&gt;");
}

function calculateHTMLForEmote(emote) {
	var html = "";
	if (emote.upscale) {
		html = '<img class="emote upscale" src="' + emote.image + '" />';
	} else {
		if (emote.zeroWidth) {
			html = '<img class="emote" data-zw="true" src="' + emote.image + '" />';
		} else {
			html = '<img class="emote" src="' + emote.image + '" />';
		}
	}
	return html;
}

function TwitchOAuth() {
	return $.ajax({
		type: "GET",
		url: "https://id.twitch.tv/oauth2/validate",
		dataType: "json",
		headers: { 'Authorization': 'Bearer ' + credentials },
		success: function (result) {
			//set your variable to the result
			console.log('jChat: helix json aquired user_id');
			// console.log(url)
			// console.log(result)
		},
		error: function (result) {
			// this *should* show up when the token expires
			var $chatLine = $('<div style="color: red;">Twitch OAuth invalid</div>');
			Chat.info.lines.push($chatLine.wrap('<div>').parent().html());
		}
	});
}

function TwitchAPI(url) {
	return $.ajax({
		type: "GET",
		url: "https://api.twitch.tv/helix" + url,
		dataType: "json",
		headers: {
			'Authorization': 'Bearer ' + credentials,
			'Client-Id': client_id
		},
		success: function () {
			console.log('jChat: GET ' + url);
		},
		error: function (result) {
			var $chatLine = $('<div style="color: red;">Twitch API Error</div>');
			console.log(result)
			Chat.info.lines.push($chatLine.wrap('<div>').parent().html());
		}
	});
}

function loadChannelInformation() {
	Chat.loadEmotes(Chat.info.channelID);

	const twitchBadgesPromise = TwitchAPI('/chat/badges?broadcaster_id=' + Chat.info.channelID).done(function (res) {
		res?.data.forEach(badge => {
			badge?.versions.forEach(version => {
				Chat.info.badges[badge.set_id + ':' + version.id] = version.image_url_4x;
			});
		});
	});

	const frankerfacesPromise = $.getJSON('https://api.frankerfacez.com/v1/_room/id/' + encodeURIComponent(Chat.info.channelID)).done(function (res) {
		if (res.room.moderator_badge) {
			Chat.info.badges['moderator:1'] = 'https://cdn.frankerfacez.com/room-badge/mod/' + Chat.info.channel + '/4/rounded';
		}
		if (res.room.vip_badge) {
			Chat.info.badges['vip:1'] = 'https://cdn.frankerfacez.com/room-badge/vip/' + Chat.info.channel + '/4';
		}
	});

	// Load cheers images
	const twitchCheersPromise = TwitchAPI("/bits/cheermotes?broadcaster_id=" + Chat.info.channelID).done(function (res) {
		res = res.data
		res.forEach(action => {
			Chat.info.cheers[action.prefix] = {}
			action.tiers.forEach(tier => {
				Chat.info.cheers[action.prefix][tier.min_bits] = {
					image: tier.images.dark.animated['4'],
					color: tier.color
				};
			});
		});
	});

	return Promise.all([twitchBadgesPromise, frankerfacesPromise, twitchCheersPromise])
		.then(() => {
			console.log('All data loaded successfully internal.');
			// Perform further actions with loaded data here
		})
		.catch(error => {
			console.error('An error occurred:', error);
		});
}