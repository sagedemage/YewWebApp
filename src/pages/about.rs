use yew::prelude::*;

pub struct About;
impl Component for About {
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
                        <h1 class="title is-1">{ "About" }</h1>
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
impl About {
    fn view_content(&self) -> Html {
        html! {
            <div class="content">
                {" I am a web developer and information security nerd.
                The operating system I use is Linux. I use Linux since
                it is great for doing programming imo. I have access to
                the terminal. I can set keyboard shortcuts in order
                to make Linux usage as fast as possible. I feel like a 
                ninja using Linux."}
            </div>
        }
    }
}