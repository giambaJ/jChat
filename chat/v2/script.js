(function ($) {
  // Thanks to BrunoLM (https://stackoverflow.com/a/3855394)
  $.QueryString = (function (paramsArray) {
    let params = {};

    for (let i = 0; i < paramsArray.length; ++i) {
      let param = paramsArray[i].split('=', 2);

      if (param.length !== 2) continue;

      params[param[0]] = decodeURIComponent(param[1].replace(/\+/g, ' '));
    }

    return params;
  })(window.location.search.substr(1).split('&'));
})(jQuery);

Chat = {
  info: {
    channel: null,
    animate:
      'animate' in $.QueryString
        ? $.QueryString.animate.toLowerCase() === 'true'
        : false,
    showBots:
      'bots' in $.QueryString
        ? $.QueryString.bots.toLowerCase() === 'true'
        : false,
    hideCommands:
      'hide_commands' in $.QueryString
        ? $.QueryString.hide_commands.toLowerCase() === 'true'
        : false,
    hideBadges:
      'hide_badges' in $.QueryString
        ? $.QueryString.hide_badges.toLowerCase() === 'true'
        : false,
    fade: 'fade' in $.QueryString ? parseInt($.QueryString.fade) : false,
    size: 'size' in $.QueryString ? parseInt($.QueryString.size) : 3,
    font: 'font' in $.QueryString ? parseInt($.QueryString.font) : 0,
    stroke: 'stroke' in $.QueryString ? parseInt($.QueryString.stroke) : false,
    shadow: 'shadow' in $.QueryString ? parseInt($.QueryString.shadow) : false,
    smallCaps:
      'small_caps' in $.QueryString
        ? $.QueryString.small_caps.toLowerCase() === 'true'
        : false,
    emotes: {},
    badges: {},
    userBadges: {},
    ffzapBadges: null,
    bttvBadges: null,
    seventvBadges: null,
    chatterinoBadges: null,
    cheers: {},
    lines: [],
    blockedUsers:
      'block' in $.QueryString
        ? $.QueryString.block.toLowerCase().split(',')
        : false,
    bots: ['streamelements', 'streamlabs', 'nightbot', 'moobot', 'fossabot'],
  },

  loadEmotes: function (channelID) {
    Chat.info.emotes = {};
    // Load BTTV, FFZ and 7TV emotes
    ['emotes/global', 'users/twitch/' + encodeURIComponent(channelID)].forEach(
      (endpoint) => {
        $.getJSON(
          'https://api.betterttv.net/3/cached/frankerfacez/' + endpoint,
        ).done(function (res) {
          res.forEach((emote) => {
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
            };
          });
        });
      },
    );

    ['emotes/global', 'users/twitch/' + encodeURIComponent(channelID)].forEach(
      (endpoint) => {
        $.getJSON('https://api.betterttv.net/3/cached/' + endpoint).done(
          function (res) {
            if (!Array.isArray(res)) {
              res = res.channelEmotes.concat(res.sharedEmotes);
            }
            res.forEach((emote) => {
              Chat.info.emotes[emote.code] = {
                id: emote.id,
                image: 'https://cdn.betterttv.net/emote/' + emote.id + '/3x',
                zeroWidth: [
                  '5e76d338d6581c3724c0f0b2',
                  '5e76d399d6581c3724c0f0b8',
                  '567b5b520e984428652809b6',
                  '5849c9a4f52be01a7ee5f79d',
                  '567b5c080e984428652809ba',
                  '567b5dc00e984428652809bd',
                  '58487cc6f52be01a7ee5f205',
                  '5849c9c8f52be01a7ee5f79e',
                ].includes(emote.id), // "5e76d338d6581c3724c0f0b2" => cvHazmat, "5e76d399d6581c3724c0f0b8" => cvMask, "567b5b520e984428652809b6" => SoSnowy, "5849c9a4f52be01a7ee5f79d" => IceCold, "567b5c080e984428652809ba" => CandyCane, "567b5dc00e984428652809bd" => ReinDeer, "58487cc6f52be01a7ee5f205" => SantaHat, "5849c9c8f52be01a7ee5f79e" => TopHat
              };
            });
          },
        );
      },
    );

    [
      'emotes/global',
      'users/' + encodeURIComponent(channelID) + '/emotes',
    ].forEach((endpoint) => {
      $.getJSON('https://api.7tv.app/v2/' + endpoint).done(function (res) {
        res.forEach((emote) => {
          Chat.info.emotes[emote.name] = {
            id: emote.id,
            image: emote.urls[emote.urls.length - 1][1],
            zeroWidth: emote.visibility_simple.includes('ZERO_WIDTH'),
          };
        });
      });
    });
  },

  load: function (callback) {
    TwitchAPI(
      'https://api.twitch.tv/helix/users?login=' + Chat.info.channel,
    ).done(function (res) {
      Chat.info.channelID = res.data[0].id;
      Chat.loadEmotes(Chat.info.channelID);

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

      // Load badges
      TwitchAPI('https://badges.twitch.tv/v1/badges/global/display').done(
        function (global) {
          Object.entries(global.badge_sets).forEach((badge) => {
            Object.entries(badge[1].versions).forEach((v) => {
              Chat.info.badges[badge[0] + ':' + v[0]] = v[1].image_url_4x;
            });
          });
          TwitchAPI(
            'https://badges.twitch.tv/v1/badges/channels/' +
              encodeURIComponent(Chat.info.channelID) +
              '/display',
          ).done(function (channel) {
            Object.entries(channel.badge_sets).forEach((badge) => {
              Object.entries(badge[1].versions).forEach((v) => {
                Chat.info.badges[badge[0] + ':' + v[0]] = v[1].image_url_4x;
              });
            });
            $.getJSON(
              'https://api.frankerfacez.com/v1/_room/id/' +
                encodeURIComponent(Chat.info.channelID),
            ).done(function (res) {
              if (res.room.moderator_badge) {
                Chat.info.badges['moderator:1'] =
                  'https://cdn.frankerfacez.com/room-badge/mod/' +
                  Chat.info.channel +
                  '/4/rounded';
              }
              if (res.room.vip_badge) {
                Chat.info.badges['vip:1'] =
                  'https://cdn.frankerfacez.com/room-badge/vip/' +
                  Chat.info.channel +
                  '/4';
              }
            });
          });
        },
      );

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

        $.getJSON('https://api.7tv.app/v2/badges?user_identifier=login')
          .done(function (res) {
            Chat.info.seventvBadges = res.badges;
          })
          .fail(function () {
            Chat.info.seventvBadges = [];
          });

        $.getJSON('https://api.chatterino.com/badges')
          .done(function (res) {
            Chat.info.chatterinoBadges = res.badges;
          })
          .fail(function () {
            Chat.info.chatterinoBadges = [];
          });
      }

      // Load cheers images
      TwitchAPI(
        'https://api.twitch.tv/helix/bits/cheermotes?broadcaster_id=' +
          Chat.info.channelId,
      ).done(function (res) {
        res.data.forEach((action) => {
          Chat.info.cheers[action.prefix] = {};
          action.tiers.forEach((tier) => {
            Chat.info.cheers[action.prefix][tier.min_bits] = {
              image: tier.images.dark.animated['4'],
              color: tier.color,
            };
          });
        });
      });

      callback(true);
    });
  },

  update: setInterval(function () {
    if (Chat.info.lines.length > 0) {
      var lines = Chat.info.lines.join('');

      if (Chat.info.animate) {
        var $auxDiv = $('<div></div>', { class: 'hidden' }).appendTo(
          '#chat_container',
        );
        $auxDiv.append(lines);
        var auxHeight = $auxDiv.height();
        $auxDiv.remove();

        var $animDiv = $('<div></div>');
        $('#chat_container').append($animDiv);
        $animDiv.animate({ height: auxHeight }, 150, function () {
          $(this).remove();
          $('#chat_container').append(lines);
        });
      } else {
        $('#chat_container').append(lines);
      }
      Chat.info.lines = [];
      var linesToDelete = $('.chat_line').length - 100;
      while (linesToDelete > 0) {
        $('.chat_line').eq(0).remove();
        linesToDelete--;
      }
    } else if (Chat.info.fade) {
      var messageTime = $('.chat_line').eq(0).data('time');
      if ((Date.now() - messageTime) / 1000 >= Chat.info.fade) {
        $('.chat_line')
          .eq(0)
          .fadeOut(function () {
            $(this).remove();
          });
      }
    }
  }, 200),

  loadUserBadges: function (nick, userId) {
    Chat.info.userBadges[nick] = [];
    $.getJSON('https://api.frankerfacez.com/v1/user/' + nick).always(function (
      res,
    ) {
      if (res.badges) {
        Object.entries(res.badges).forEach((badge) => {
          var userBadge = {
            description: badge[1].title,
            url: 'https:' + badge[1].urls['4'],
            color: badge[1].color,
          };
          if (!Chat.info.userBadges[nick].includes(userBadge))
            Chat.info.userBadges[nick].push(userBadge);
        });
      }
      Chat.info.ffzapBadges.forEach((user) => {
        if (user.id.toString() === userId) {
          var color = '#755000';
          if (user.tier == 2) color = user.badge_color || '#755000';
          else if (user.tier == 3) {
            if (user.badge_is_colored == 0)
              color = user.badge_color || '#755000';
            else color = false;
          }
          var userBadge = {
            description: 'FFZ:AP Badge',
            url: 'https://api.ffzap.com/v1/user/badge/' + userId + '/3',
            color: color,
          };
          if (!Chat.info.userBadges[nick].includes(userBadge))
            Chat.info.userBadges[nick].push(userBadge);
        }
      });
      Chat.info.bttvBadges.forEach((user) => {
        if (user.name === nick) {
          var userBadge = {
            description: user.badge.description,
            url: user.badge.svg,
          };
          if (!Chat.info.userBadges[nick].includes(userBadge))
            Chat.info.userBadges[nick].push(userBadge);
        }
      });
      Chat.info.seventvBadges.forEach((badge) => {
        badge.users.forEach((user) => {
          if (user === nick) {
            var userBadge = {
              description: badge.tooltip,
              url: badge.urls[2][1],
            };
            if (!Chat.info.userBadges[nick].includes(userBadge))
              Chat.info.userBadges[nick].push(userBadge);
          }
        });
      });
      Chat.info.chatterinoBadges.forEach((badge) => {
        badge.users.forEach((user) => {
          if (user === userId) {
            var userBadge = {
              description: badge.tooltip,
              url: badge.image3 || badge.image2 || badge.image1,
            };
            if (!Chat.info.userBadges[nick].includes(userBadge))
              Chat.info.userBadges[nick].push(userBadge);
          }
        });
      });
    });
  },

  write: function (nick, info, message) {
    if (info) {
      var $chatLine = $('<div></div>');
      $chatLine.addClass('chat_line');
      $chatLine.attr('data-nick', nick);
      $chatLine.attr('data-time', Date.now());
      $chatLine.attr('data-id', info.id);
      var $userInfo = $('<span></span>');
      $userInfo.addClass('user_info');

      // Writing badges
      if (Chat.info.hideBadges) {
        if (typeof info.badges === 'string') {
          info.badges.split(',').forEach((badge) => {
            var $badge = $('<img/>');
            $badge.addClass('badge');
            badge = badge.split('/');
            $badge.attr('src', Chat.info.badges[badge[0] + ':' + badge[1]]);
            $userInfo.append($badge);
          });
        }
      } else {
        var badges = [];
        const priorityBadges = [
          'predictions',
          'admin',
          'global_mod',
          'staff',
          'twitchbot',
          'broadcaster',
          'moderator',
          'vip',
        ];
        if (typeof info.badges === 'string') {
          info.badges.split(',').forEach((badge) => {
            badge = badge.split('/');
            var priority = priorityBadges.includes(badge[0]) ? true : false;
            badges.push({
              description: badge[0],
              url: Chat.info.badges[badge[0] + ':' + badge[1]],
              priority: priority,
            });
          });
        }
        var $modBadge;
        badges.forEach((badge) => {
          if (badge.priority) {
            var $badge = $('<img/>');
            $badge.addClass('badge');
            $badge.attr('src', badge.url);
            if (badge.description === 'moderator') $modBadge = $badge;
            $userInfo.append($badge);
          }
        });
        if (Chat.info.userBadges[nick]) {
          Chat.info.userBadges[nick].forEach((badge) => {
            var $badge = $('<img/>');
            $badge.addClass('badge');
            if (badge.color) $badge.css('background-color', badge.color);
            if (badge.description === 'Bot' && info.mod === '1') {
              $badge.css('background-color', 'rgb(0, 173, 3)');
              $modBadge.remove();
            }
            $badge.attr('src', badge.url);
            $userInfo.append($badge);
          });
        }
        badges.forEach((badge) => {
          if (!badge.priority) {
            var $badge = $('<img/>');
            $badge.addClass('badge');
            $badge.attr('src', badge.url);
            $userInfo.append($badge);
          }
        });
      }

      // Writing username
      var $username = $('<span></span>');
      $username.addClass('nick');
      if (typeof info.color === 'string') {
        if (tinycolor(info.color).getBrightness() <= 50)
          var color = tinycolor(info.color).lighten(30);
        else var color = info.color;
      } else {
        const twitchColors = [
          '#FF0000',
          '#0000FF',
          '#008000',
          '#B22222',
          '#FF7F50',
          '#9ACD32',
          '#FF4500',
          '#2E8B57',
          '#DAA520',
          '#D2691E',
          '#5F9EA0',
          '#1E90FF',
          '#FF69B4',
          '#8A2BE2',
          '#00FF7F',
        ];
        var color = twitchColors[nick.charCodeAt(0) % 15];
      }
      $username.css('color', color);
      $username.html(info['display-name'] ? info['display-name'] : nick);
      $userInfo.append($username);

      // Writing message
      var $message = $('<span></span>');
      $message.addClass('message');
      if (/^\x01ACTION.*\x01$/.test(message)) {
        $message.css('color', color);
        message = message
          .replace(/^\x01ACTION/, '')
          .replace(/\x01$/, '')
          .trim();
        $userInfo.append('<span>&nbsp;</span>');
      } else {
        $userInfo.append('<span class="colon">:</span>');
      }
      $chatLine.append($userInfo);

      // Replacing emotes and cheers
      var replacements = {};
      if (typeof info.emotes === 'string') {
        info.emotes.split('/').forEach((emoteData) => {
          var twitchEmote = emoteData.split(':');
          var indexes = twitchEmote[1].split(',')[0].split('-');
          var emojis = new RegExp('[\u1000-\uFFFF]+', 'g');
          var aux = message.replace(emojis, ' ');
          var emoteCode = aux.substr(indexes[0], indexes[1] - indexes[0] + 1);
          replacements[emoteCode] =
            '<img class="emote" src="https://static-cdn.jtvnw.net/emoticons/v2/' +
            twitchEmote[0] +
            '/default/dark/3.0" />';
        });
      }

      Object.entries(Chat.info.emotes).forEach((emote) => {
        if (message.search(escapeRegExp(emote[0])) > -1) {
          if (emote[1].upscale)
            replacements[emote[0]] =
              '<img class="emote upscale" src="' + emote[1].image + '" />';
          else if (emote[1].zeroWidth)
            replacements[emote[0]] =
              '<img class="emote" data-zw="true" src="' +
              emote[1].image +
              '" />';
          else
            replacements[emote[0]] =
              '<img class="emote" src="' + emote[1].image + '" />';
        }
      });

      message = escapeHtml(message);

      if (info.bits && parseInt(info.bits) > 0) {
        var bits = parseInt(info.bits);
        var parsed = false;
        for (cheerType of Object.entries(Chat.info.cheers)) {
          var regex = new RegExp(cheerType[0] + '\\d+\\s*', 'ig');
          if (message.search(regex) > -1) {
            message = message.replace(regex, '');

            if (!parsed) {
              var closest = 1;
              for (cheerTier of Object.keys(cheerType[1])
                .map(Number)
                .sort((a, b) => a - b)) {
                if (bits >= cheerTier) closest = cheerTier;
                else break;
              }
              message =
                '<img class="cheer_emote" src="' +
                cheerType[1][closest].image +
                '" /><span class="cheer_bits" style="color: ' +
                cheerType[1][closest].color +
                ';">' +
                bits +
                '</span> ' +
                message;
              parsed = true;
            }
          }
        }
      }

      var replacementKeys = Object.keys(replacements);
      replacementKeys.sort(function (a, b) {
        return b.length - a.length;
      });

      replacementKeys.forEach((replacementKey) => {
        var regex = new RegExp(
          '(?<!\\S)(' + escapeRegExp(replacementKey) + ')(?!\\S)',
          'g',
        );
        message = message.replace(regex, replacements[replacementKey]);
      });

      message = twemoji.parse(message);
      $message.html(message);

      // Writing zero-width emotes
      messageNodes = $message.children();
      messageNodes.each(function (i) {
        if (
          i != 0 &&
          $(this).data('zw') &&
          ($(messageNodes[i - 1]).hasClass('emote') ||
            $(messageNodes[i - 1]).hasClass('emoji')) &&
          !$(messageNodes[i - 1]).data('zw')
        ) {
          var $container = $('<span></span>');
          $container.addClass('zero-width_container');
          $(this).addClass('zero-width');
          $(this).before($container);
          $container.append(messageNodes[i - 1], this);
        }
      });
      $message.html($message.html().trim());
      $chatLine.append($message);
      Chat.info.lines.push($chatLine.wrap('<div>').parent().html());
    }
  },

  clearChat: function (nick) {
    setTimeout(function () {
      $('.chat_line[data-nick=' + nick + ']').remove();
    }, 200);
  },

  clearMessage: function (id) {
    setTimeout(function () {
      $('.chat_line[data-id=' + id + ']').remove();
    }, 200);
  },

  connect: function (channel) {
    Chat.info.channel = channel;
    var title = $(document).prop('title');
    $(document).prop('title', title + Chat.info.channel);

    Chat.load(function () {
      console.log('jChat: Connecting to IRC server...');
      var socket = new ReconnectingWebSocket('wss://127.0.0.1/ws/', 'irc', {
        reconnectInterval: 2000,
      });

      socket.onopen = function () {
        console.log('jChat: Connected');
        socket.send('PASS blah\r\n');
        socket.send(
          'NICK justinfan' + Math.floor(Math.random() * 99999) + '\r\n',
        );
        socket.send('CAP REQ :twitch.tv/commands twitch.tv/tags\r\n');
        socket.send('JOIN #' + Chat.info.channel + '\r\n');
      };

      socket.onclose = function () {
        console.log('jChat: Disconnected');
      };

      socket.onmessage = function (data) {
        // Temp for testing in order to figure out what the Twitch API sends so we can emulate it.
        console.log(JSON.stringify(data.data));

        data.data.split('\r\n').forEach((line) => {
          if (!line) return;
          var message = window.parseIRC(line);
          if (!message.command) return;

          switch (message.command) {
            case 'PING':
              socket.send('PONG ' + message.params[0]);
              return;
            case 'JOIN':
              console.log('jChat: Joined channel #' + Chat.info.channel);
              return;
            case 'CLEARMSG':
              if (message.tags)
                Chat.clearMessage(message.tags['target-msg-id']);
              return;
            case 'CLEARCHAT':
              if (message.params[1]) Chat.clearChat(message.params[1]);
              return;
            case 'PRIVMSG':
              if (message.params[0] !== '#' + channel || !message.params[1])
                return;
              var nick = message.prefix.split('@')[0].split('!')[0];

              if (
                message.params[1].toLowerCase() === '!refreshoverlay' &&
                typeof message.tags.badges === 'string'
              ) {
                var flag = false;
                message.tags.badges.split(',').forEach((badge) => {
                  badge = badge.split('/');
                  if (badge[0] === 'moderator' || badge[0] === 'broadcaster') {
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

              if (Chat.info.hideCommands) {
                if (/^!.+/.test(message.params[1])) return;
              }

              if (!Chat.info.showBots) {
                if (Chat.info.bots.includes(nick)) return;
              }

              if (Chat.info.blockedUsers) {
                if (Chat.info.blockedUsers.includes(nick)) return;
              }

              if (!Chat.info.hideBadges) {
                if (
                  Chat.info.bttvBadges &&
                  Chat.info.seventvBadges &&
                  Chat.info.chatterinoBadges &&
                  Chat.info.ffzapBadges &&
                  !Chat.info.userBadges[nick]
                )
                  Chat.loadUserBadges(nick, message.tags['user-id']);
              }

              Chat.write(nick, message.tags, message.params[1]);
              return;
          }
        });
      };
    });
  },
};

$(document).ready(function () {
  Chat.connect(
    $.QueryString.channel ? $.QueryString.channel.toLowerCase() : 'giambaj',
  );
});
