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
    let size = sizes[Number($size.val()) - 1];
    removeCSS('size');
    appendCSS('size', size);
}

function fontUpdate(event) {
    let font = fonts[Number($font.val())];
    removeCSS('font');
    appendCSS('font', font);
}

function strokeUpdate(event) {
    removeCSS('stroke');
    if ($stroke.val() == "0")
        return;
    else {
        let stroke = strokes[Number($stroke.val()) - 1];
        appendCSS('stroke', stroke);
    }
}

function shadowUpdate(event) {
    removeCSS('shadow');
    if ($shadow.val() == "0")
        return;
    else {
        let shadow = shadows[Number($shadow.val()) - 1];
        appendCSS('shadow', shadow);
    }
}

function badgesUpdate(event) {
    if ($badges.is(':checked')) {
        $('img[class="badge special"]').addClass('hidden');
    } else {
        $('img[class="badge special hidden"]').removeClass('hidden');
    }
}

function capsUpdate(event) {
    if ($small_caps.is(':checked')) {
        appendCSS('variant', 'SmallCaps');
    } else {
        removeCSS('variant');
    }
}

function generateURL(event) {
    event.preventDefault();

    const generatedUrl = 'https://www.giambaj.it/twitch/jchat/v2/?channel=' + $channel.val();

    let data = {
        size: $size.val(),
        font: $font.val(),
        stroke: ($stroke.val() != '0' ? $stroke.val() : false),
        shadow: ($shadow.val() != '0' ? $shadow.val() : false),
        bots: $bots.is(':checked'),
        hide_commands: $commands.is(':checked'),
        hide_badges: $badges.is(':checked'),
        animate: $animate.is(':checked'),
        fade: ($fade_bool.is(':checked') ? $fade.val() : false),
        small_caps: $small_caps.is(':checked'),
        inverted: $inverted.is(':checked')
    };

    const params = encodeQueryData(data);

    $url.val(generatedUrl + '&' + params);

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
    $channel.val('');
    $size.val('3');
    $font.val('0');
    $stroke.val('0');
    $shadow.val('0');
    $bots.prop('checked', false);
    $commands.prop('checked', false);
    $badges.prop('checked', false);
    $animate.prop('checked', false);
    $fade_bool.prop('checked', false);
    $fade.addClass('hidden');
    $fade_seconds.addClass('hidden');
    $fade.val("30");
    $small_caps.prop('checked', false);
    $inverted.prop('checked', false);

    sizeUpdate();
    fontUpdate();
    strokeUpdate();
    shadowUpdate();
    badgesUpdate();
    capsUpdate();
    if ($example.hasClass("white"))
        changePreview();

    $result.addClass('hidden');
    $generator.removeClass('hidden');
    showUrl();
}

const $generator = $("form[name='generator']");
const $channel = $('input[name="channel"]');
const $animate = $('input[name="animate"]');
const $bots = $('input[name="bots"]');
const $fade_bool = $("input[name='fade_bool']");
const $fade = $("input[name='fade']");
const $fade_seconds = $("#fade_seconds");
const $commands = $("input[name='commands']");
const $small_caps = $("input[name='small_caps']");
const $inverted = $("input[name='inverted']");
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