use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::{
    About,
    Base,
    CatDetail,
    CatList,
    Cats,
    Home,
    PageNotFound,
};

#[rustfmt::skip]
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Routes {
    #[layout(Base)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[nest("/cats")]
            #[layout(Cats)]
                #[route("/")]
                CatList {},
                #[route("/:id")]
                CatDetail { id: Uuid },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
