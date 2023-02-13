mod videos_list;
use videos_list::*;
//use videos_list::VideoDetails;
//use videos_list::VideosList;
use yew::prelude::*;

#[function_component(VideoDetails)]
pub fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}
#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {

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
#[function_component(App)]
pub fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and beaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];
    println!("spam?");
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |vidx: Video| selected_video.set(Some(vidx)))
    };

    let details = selected_video.as_ref().map(|video| html! {
+        <VideoDetails video={video.clone()} />
+    });

    html! {
                    <>
                        <h1>{ "Test" }</h1>
                        <div>
                            <h3>{"Videos to watch"}</h3>
    <VideosList videos={videos} on_click={on_video_select.clone()} />
                        </div>
            +            { for details }
                    </>
                }
}
