MathJax = {
    tex: {
        inlineMath: [['$', '$'], ['\\(', '\\)']]
    }
};

$(document).ready(function () {
    $("#navbarSupportedContent div a[href=\"" + document.location.pathname + "\"]").addClass("active");
});
