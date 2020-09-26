function changeStyle(name, url) {
    var link = document.getElementById(name);
    link.href = url;
}

function focusElement(element) {
    element.focus();
}

function highlight(language, code) {
    return hljs.highlight(language, code).value;
}

function katexRender(code, display) {
    return katex.renderToString(code, {
        throwOnError: false,
        displayMode: display
    });
}
