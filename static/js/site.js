function collapseNav() {
    $('#navbarSupportedContent').collapse('hide');
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
