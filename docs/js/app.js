(function(remark, haiku, Elm){
    let slideshow = remark.create({
        sourceUrl: 'presentation.md'
    });

    haiku.load("program/haiku.bergen", function(result){
        var h = document.getElementById('haiku');
        h.value = result;
    });

    Elm.Main.init({ node: document.getElementById('brnfck-container') });
})(remark, haiku, Elm);
