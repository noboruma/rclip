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
function refreshLoginInputs(emailInput, passwdInput, button, loginConsole) {
    if (getCookie("namespace")) {
        emailInput.style.display = 'none';
        emailInput.required = false;
        passwdInput.style.display = 'none';
        passwdInput.required = false;
        toggleButton(true, button, "Log off");
        loginConsole.innerText = "Logged in";
    } else {
        emailInput.style.display = '';
        emailInput.required = false;
        passwdInput.style.display = '';
        passwdInput.required = false;
        toggleButton(true, button, "Login");
        loginConsole.innerText = "";
    }
}
function handleLogin(emailInput, passwdInput, button, loginConsole) {
    if (getCookie("namespace")) {
        setCookie('namespace', '');
        refreshLoginInputs(emailInput, passwdInput, button, loginConsole);
        return;
    }

    if (emailInput.value.length == 0) {
        emailInput.required = true;
        return;
    }

    if (passwdInput.value.length == 0) {
        passwdInput.required = true;
        return;
    }

    toggleButton(false, button, "Login...");
    rclip.handleLogin(emailInput.value, passwdInput.value,
        function (namespace) {
            if (namespace !== 'error') {
                setCookie("namespace", namespace);
                refreshLoginInputs(emailInput, passwdInput, button, loginConsole);
            } else {
                refreshLoginInputs(emailInput, passwdInput, button, loginConsole);
                loginConsole.innerText = 'Wrong email/password';
            }
    });
}
function handleCopy(token, button, content) {
    if (token.value.length == 0) {
        token.required = true;
        return;
    }

    if (content.value.length == 0) {
        content.required = true;
        return;
    }

    toggleButton(false, button, "Copying...");
    setCookie("token", token.value);
    namespace = getCookie('namespace');
    rclip.handleCopy(token.value, namespace, content.value, function() {
        toggleButton(true, button, "Copy");
    });
}
function handlePaste(token, button, pasteSection, pasteOutput) {
    if (token.value.length == 0) {
        token.required = true;
        return;
    }

    toggleButton(false, button, "Pasting...");

    setCookie("token", token.value);

    pasteSection.style.display = "block";
    namespace = getCookie('namespace');
    rclip.handlePaste(token.value, namespace, function (value) {
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

