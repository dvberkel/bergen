(function(remark, haiku){
    let slideshow = remark.create({
        sourceUrl: 'presentation.md'
    });

    haiku.load("program/haiku.bergen", function(result){
        var h = document.getElementById('haiku');
        h.value = result;
    });
})(remark, haiku);
