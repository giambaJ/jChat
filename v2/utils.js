function appendCSS(file){
    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: "size",
        href: `styles/${file}.css`
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
    return $.getJSON(url + (url.search(/\?/) > -1 ? '&' : '?') + 'client_id=' + client_id);
}
