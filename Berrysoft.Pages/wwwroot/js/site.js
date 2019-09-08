function changeStyle(name, url) {
    var link = document.getElementById(name);
    link.href = url;
}

function highlight(language, code) {
    return hljs.highlight(language, code).value;
}
