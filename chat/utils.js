function encodeQueryData(data) { // https://stackoverflow.com/questions/111529/how-to-create-query-parameters-in-javascript
    const ret = [];
    for (let d in data) {
        if (data[d])
            ret.push(encodeURIComponent(d) + '=' + encodeURIComponent(data[d]));
    }
    return ret.join('&');
}

function appendCSS(type, name) {
    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: `preview_${type}`,
        href: `styles/${type}_${name}.css`
    }).appendTo("head");
}

function removeCSS(type) {
    $(`link[class="preview_${type}"]`).remove();
}