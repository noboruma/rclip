function copyContent(textarea, button) {
    toggleButton(false, button, "Copied!");

    textarea.select();
    document.execCommand("copy");

    setTimeout(function() {
        toggleButton(true, button, "Copy to local clipboard");
    }, 500);
}
function setCookie(name, value) {
    var today = new Date();
    var expiry = new Date(today.getTime() + 30 * 24 * 3600 * 1000);
    document.cookie=name + "=" + escape(value) + ";expires=" + expiry.toGMTString() + ";path=/";
}
function getCookie(name) {
    if (document.cookie != ""){
        cookies = document.cookie.split(";");
        for (var i = 0; i < cookies.length; i++) {
            cookie = cookies[i].trim().split("=");
            if (cookie[0] == name) {
                return cookie[1];
            }
        }
    }
    return '';
}
function toggleButton(enable, button, text) {
    if (enable) {
        button.innerText= text;
        button.className = "";
        button.disabled = false;
    } else {
        button.innerText= text;
        button.className = "make-background-grey";
        button.disabled = true;
    }
}
function handleGenerate(tokenInput, button) {
    tokenInput.required = false;
    if (tokenInput.value.length != 0) return;

    toggleButton(false, button, "Generating...");
    rclip.handleGenerate(function (token) {
        tokenInput.value = token;
        toggleButton(true, button, "Get token");
        setCookie("token", token);
        tokenInput.required = true;
    });
}
function handleCopy(token, button, content) {
    if (token.length == 0) return;

    if (content.value.length == 0) {
        content.required = true;
        return;
    }

    toggleButton(false, button, "Copying...");
    setCookie("token", token);
    rclip.handleCopy(token, content.value, function() {
        toggleButton(true, button, "Copy");
    });
}
function handlePaste(token, button, pasteSection, pasteOutput) {
    if (token.length == 0) return;

    toggleButton(false, button, "Pasting...");

    setCookie("token", token);

    pasteSection.style.display = "block";
    rclip.handlePaste(token, function (value) {
        pasteOutput.value = value;
        toggleButton(true, button, "Paste");
    });
}
function shareClipboard(token, button) {
    if (token.length == 0) return;

    toggleButton(false, button, "Copied!");

    const tmp = document.createElement('textarea');
    tmp.value = window.location.href.split('?')[0] + '?token=' + token;
    document.body.appendChild(tmp);
    tmp.select();
    document.execCommand('copy');
    document.body.removeChild(tmp);

    setTimeout(function() {
        toggleButton(true, button, "Share");
    }, 500);
}

