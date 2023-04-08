#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use rocket::serde::json::{json, Value, Json};
use rocket::serde::{Deserialize, Serialize};
use tch::jit::CModule;
use tch::vision::imagenet;
use tch::Kind;
use tch::Tensor;
use crate::utils::download_util::download_util::download_file_text_async;
pub mod utils;


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct InferenceRequest<'r> {
    url: &'r str,
}


#[derive(Serialize)]
struct InferenceResponse {
    probability: f64,
    class: String,
}

struct Model {
    module: CModule,
}

impl Model {
    fn new(model_path: &str) -> Self {
        let model = tch::CModule::load(model_path).expect("Couldn't load model");
        Self { module: model }
    }

    fn infer(&self, input: &Tensor) -> Vec<InferenceResponse> {
        let inference_res = self
            .module
            .forward_ts(&[input.unsqueeze(0)])
            .unwrap()
            .softmax(-1, Kind::Float);

        imagenet::top(&inference_res, 5)
            .into_iter()
            .map(|(probability, class)| InferenceResponse {probability, class })
            .collect()

    }
}

#[get("/")]
async fn inference_handler(model: &State<Model>) -> Value {
    let img_file = download_file_text_async("http://t3.gstatic.com/licensed-image?q=tbn:ANd9GcRv7Ev1T8O6as52YDwz3YDa9ya3-xv5SpMw3Lk_mZHqxwWvGDCw47ZaixFiTefWHF_dHHqDFFSYk2ZRKfU").await.expect("download failed");
    let tensor = imagenet::load_image_and_resize_from_memory(&img_file, 224, 224).unwrap();
    let output = model.infer(&tensor);
    json!(output)
}

#[post("/infer", format = "json", data="<req>")]
async fn post_inference_handler(req: Json<InferenceRequest<'_>>, model: &State<Model>) -> Value {
    let img_file = download_file_text_async(&(req.url[..])).await.expect("download failed");
    let tensor = imagenet::load_image_and_resize_from_memory(&img_file, 224, 224).unwrap();
    let output = model.infer(&tensor);
    json!(output)
}

#[launch]
fn rocket() -> _ {
    // let model_path = "/Users/gill/Documents/rust-learning/serve_model/model_load/src/models/mobile_net_v2.pt";
    let model_path = "/app/artifacts/resnet18.pt";

    let model = Model::new(model_path);

    rocket::build()
        .mount("/", routes![inference_handler, post_inference_handler])
        .manage(model)
}
