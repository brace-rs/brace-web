use crate::Element;

macro_rules! elements {
    ( $($name:ident)* ) => {
        $(
            #[cfg_attr(tarpaulin, skip)]
            pub fn $name() -> Element {
                Element::new(stringify!($name))
            }
        )*
    };
}

elements! {
    a abbr address area article aside audio b base bdi bdo blockquote body br button canvas caption
    cite code col colgroup data datalist dd del details dfn dialog div dl dt em embed fieldset
    figcaption figure footer form h1 h2 h3 h4 h5 h6 head header hgroup hr html i iframe img input
    ins kbd label legend li link main map mark menu menuitem meta meter nav noscript object ol
    optgroup option output p param picture pre progress q rb rp rt rtc ruby s samp script section
    select slot small source span strong style sub summary sup table tbody td template textarea
    tfoot th thead time title tr track u ul var video wbr
}

elements! {
    path circle ellipse line polygon polyline rect image
}

#[cfg_attr(tarpaulin, skip)]
pub fn svg() -> Element {
    Element::new("svg").with_attr("xmlns", "http://www.w3.org/2000/svg")
}
