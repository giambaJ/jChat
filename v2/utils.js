function appendCSS(type, name) {
    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: `chat_${type}`,
        href: `styles/${type}_${name}.css`
    }).appendTo("head");
}

function escapeRegExp(string) { // Thanks to coolaj86 and Darren Cook (https://stackoverflow.com/a/6969486)
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function escapeHtml(message) {
    return message
        .replace(/&/g, "&amp;")
        .replace(/(<)(?!3)/g, "&lt;")
        .replace(/(>)(?!\()/g, "&gt;");
}

function TwitchAPI(url) {
/*    return $.getJSON(url + (url.search(/\?/) > -1 ? '&' : '?') + 'client_id=' + client_id);*/
/* use only helix api oauth endpoint to get user_id and store the id# */
    return $.ajax({
        type: "GET", 
        url: "https://id.twitch.tv/oauth2/validate", 
        dataType: "json",
        headers: {'Authorization': 'Bearer ' + credentials},
/* not required client_id auth for running locally */
/*        headers: {'Client-Id': client_id},*/
        success : function(result) { 
            //set your variable to the result
            console.log('jChat: helix json aquired user_id');
        }
    });
}
