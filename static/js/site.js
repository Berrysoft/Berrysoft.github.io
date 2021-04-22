function collapseNav() {
    $('#navbarSupportedContent').collapse('hide');
}

function katexRender(code, display) {
    return katex.renderToString(code, {
        throwOnError: false,
        displayMode: display
    });
}
