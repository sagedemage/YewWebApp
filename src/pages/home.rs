use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Home" }</h1>
                        <div class="content">
                            { self.view_content() }
                        </div>
                    </div>
                </div>
                // tile is-parent container
                <div class="tile container">
                </div>
            </div>
        }
    }
}
impl Home {
    fn view_content(&self) -> Html {
        html! {
            <div class="content">
                {" This is the home page. 
                There is not much to show.
                I am just kidding.
                It is a simple website created
                using the Yew framework with Rust.
                Yes, I am to used the beloved 
                Rust programming language
                to create web applications.
                I want my websites to
                be as fast and secure as possible.
                I do plan to use Rocket framework as well"}
            </div>
        }
    }
}
