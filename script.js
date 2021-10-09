function fadeOption(event) {
    if ($fade_bool.is(':checked')) {
        $fade.removeClass('hidden');
        $fade_seconds.removeClass('hidden');
    } else {
        $fade.addClass('hidden');
        $fade_seconds.addClass('hidden');
    }
}

function sizeUpdate(event) {
    $('link[class="size"]').remove();

    const size = sizes[Number($size.val())]

    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: "size",
        href: `styles/size_${size}.css`
    }).appendTo("head");
}

function fontUpdate(event) {
    $('link[class="font"]').remove();

    const font = fonts[Number($font.val())]

    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: "font",
        href: `styles/font_${font}.css`
    }).appendTo("head");

}

function strokeUpdate(event) {
    $('link[class="stroke"]').remove();

    if ($stroke.val() == "0") return // if "off is selected"

    const stroke = strokes[Number($stroke.val()) - 1]

    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: "stroke",
        href: `styles/stroke_${stroke}.css`
    }).appendTo("head");
}

function shadowUpdate(event) {
    $('link[class="shadow"]').remove();

    if ($shadow.val() == "0") return // if "off is selected"

    const shadow =  shadows[Number($shadow.val()) -1]

    $("<link/>", {
        rel: "stylesheet",
        type: "text/css",
        class: "shadow",
        href:  `styles/shadow_${shadow}.css`
    }).appendTo("head");
}

function capsUpdate(event) {
    if ($small_caps.is(':checked')) {
        $("<link/>", {
            rel: "stylesheet",
            type: "text/css",
            class: "small_caps",
            href: "styles/variant_SmallCaps.css"
        }).appendTo("head");
    } else {
        $('link[class="small_caps"]').remove();
    }
}

function badgesUpdate(event) {
    if ($badges.is(':checked')) {
        $('img[class="badge special"]').addClass('hidden');
    } else {
        $('img[class="badge special hidden"]').removeClass('hidden');
    }
}

function generateURL(event) {
    event.preventDefault();

    var generatedUrl = 'https://www.giambaj.it/twitch/jchat/v2/?channel=' + $channel.val();
    if ($animate.is(':checked')) generatedUrl += '&animate=true';
    if ($bots.is(':checked')) generatedUrl += '&bots=true';
    if ($fade_bool.is(':checked')) generatedUrl += '&fade=' + $fade.val();
    if ($commands.is(':checked')) generatedUrl += '&hide_commands=true';
    if ($badges.is(':checked')) generatedUrl += '&hide_badges=true';
    generatedUrl += '&size=' + $size.val();
    generatedUrl += '&font=' + $font.val();
    if ($stroke.val() != '0') generatedUrl += '&stroke=' + $stroke.val();
    if ($shadow.val() != '0') generatedUrl += '&shadow=' + $shadow.val();
    if ($small_caps.is(':checked')) generatedUrl += '&small_caps=true';

    $url.val(generatedUrl);
    $generator.addClass('hidden');
    $result.removeClass('hidden');
}

function changePreview(event) {
    if ($example.hasClass("white")) {
        $example.removeClass("white");
        $brightness.attr('src', "img/light.png");
    } else {
        $example.addClass("white");
        $brightness.attr('src', "img/dark.png");
    }
}

function copyUrl(event) {
    navigator.clipboard.writeText($url.val());

    $alert.css('visibility', 'visible');
    $alert.css('opacity', '1');
}

function showUrl(event) {
    $alert.css('opacity', '0');
    setTimeout(function() {
        $alert.css('visibility', 'hidden');
    }, 200);
}

function resetForm(event) {
    $channel.val("");
    $animate.prop('checked', false);
    $bots.prop('checked', false);
    $fade_bool.prop('checked', false);
    $fade.addClass('hidden');
    $fade_seconds.addClass('hidden');
    $fade.val("30");
    $commands.prop('checked', false);
    $small_caps.prop('checked', false);
    $badges.prop('checked', false);
    $('link[class="small_caps"]').remove();
    $('img[class="badge special hidden"]').removeClass('hidden');
    $result.addClass('hidden');
    $generator.removeClass('hidden');
    showUrl();
}

const fonts = [ 'BalooTammudu', 'SegoeUI', 'Roboto', 'Lato', 'NotoSans', 'SourceCodePro',
'Impact', 'Comfortaa', 'DancingScript', 'IndieFlower', 'PressStart2P', 'Wallpoet']
const sizes = ['small', 'medium', 'large']
const strokes = ['thin', 'medium', 'thick', 'thicker']
const shadows = ['small', 'medium', 'large']

const $generator = $("form[name='generator']");
const $channel = $('input[name="channel"]');
const $animate = $('input[name="animate"]');
const $bots = $('input[name="bots"]');
const $fade_bool = $("input[name='fade_bool']");
const $fade = $("input[name='fade']");
const $fade_seconds = $("#fade_seconds");
const $commands = $("input[name='commands']");
const $small_caps = $("input[name='small_caps']");
const $badges = $("input[name='badges']");
const $size = $("select[name='size']");
const $font = $("select[name='font']");
const $stroke = $("select[name='stroke']");
const $shadow = $("select[name='shadow']");
const $brightness = $("#brightness");
const $example = $('#example');
const $result = $("#result");
const $url = $('#url');
const $alert = $("#alert");
const $reset = $("#reset");

$fade_bool.change(fadeOption);
$size.change(sizeUpdate);
$font.change(fontUpdate);
$stroke.change(strokeUpdate);
$shadow.change(shadowUpdate);
$small_caps.change(capsUpdate);
$badges.change(badgesUpdate);
$generator.submit(generateURL);
$brightness.click(changePreview);
$url.click(copyUrl);
$alert.click(showUrl);
$reset.click(resetForm);