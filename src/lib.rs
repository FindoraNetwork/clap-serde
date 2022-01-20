use clap::{App, Arg, ArgGroup};

#[macro_use]
mod de;

#[cfg(test)]
mod tests;

pub struct AppWrap<'a> {
    app: App<'a>,
}

impl<'a> From<AppWrap<'a>> for App<'a> {
    fn from(a: AppWrap<'a>) -> Self {
        a.app
    }
}

impl<'a> From<App<'a>> for AppWrap<'a> {
    fn from(app: App<'a>) -> Self {
        AppWrap { app }
    }
}

pub struct ArgWrap<'a> {
    arg: Arg<'a>,
}

impl<'a> From<ArgWrap<'a>> for Arg<'a> {
    fn from(arg: ArgWrap<'a>) -> Self {
        arg.arg
    }
}

impl<'a> From<Arg<'a>> for ArgWrap<'a> {
    fn from(arg: Arg<'a>) -> Self {
        ArgWrap { arg }
    }
}

struct ArgGroupWrap<'a> {
    group: ArgGroup<'a>,
}

impl<'a> From<ArgGroupWrap<'a>> for ArgGroup<'a> {
    fn from(group: ArgGroupWrap<'a>) -> Self {
        group.group
    }
}

impl<'a> From<ArgGroup<'a>> for ArgGroupWrap<'a> {
    fn from(group: ArgGroup<'a>) -> Self {
        ArgGroupWrap { group }
    }
}
