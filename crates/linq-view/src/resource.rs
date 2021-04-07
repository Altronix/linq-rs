pub enum Html<'a> {
    Url(&'a str),
    File(&'a str),
}

pub enum Js<'a> {
    Url(&'a str),
    File(&'a str),
}

pub enum Resource<'a> {
    Html(Html<'a>),
    Js(Js<'a>),
}

impl<'a> From<Html<'a>> for Resource<'a> {
    fn from(t: Html<'a>) -> Resource<'a> {
        match t {
            Html::File(s) => Resource::Html(Html::File(s)),
            Html::Url(s) => Resource::Html(Html::Url(s)),
        }
    }
}

impl<'a> From<Js<'a>> for Resource<'a> {
    fn from(t: Js<'a>) -> Resource<'a> {
        match t {
            Js::File(s) => Resource::Js(Js::File(s)),
            Js::Url(s) => Resource::Js(Js::Url(s)),
        }
    }
}
