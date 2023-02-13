use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>,
}
#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {

    let on_click = on_click.clone();
    videos
        .iter()
        .map(|vid| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = vid.clone();
                Callback::from(move |_| {
                    on_click.emit(video.clone())
                })
            };
            html! {
                <p key={vid.id} onclick={on_video_select}> {format!("{}: {}",vid.speaker, vid.title)}</p>
            }
        })
        .collect()
}
