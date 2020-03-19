use futures::Future;
use seed::{prelude::*, *};

use mytodo::{JsonApiResponse, Task};

struct Model {
    tasks: Vec<Task>,
}

impl Default for Model {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(fetch::ResponseDataResult<JsonApiResponse>),
}

fn fetch_data() -> impl Future<Output = Result<Msg, Msg>> {
    let url = "http://localhost:8000/tasks/";
    Request::new(url).fetch_json_data(Msg::FetchedTasks)
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
            orders.skip();
        }
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| li![{ t.title.clone() }])
        .collect();

    vec![h1![{ "Tasks" }], ul![tasks]]
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(fetch_data());
    AfterMount::default()
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();

    //app.update(Msg::FetchedTasks);
}
