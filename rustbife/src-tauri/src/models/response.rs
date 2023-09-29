// use std::{collections::HashMap, net::IpAddr};
// use reqwest::ResponseBuilderExt;
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct Response {
//     args: HashMap<String, String>,
//     // I'm making headers into another struct here to show some possibilities. You can of course make it a HashMap like args
//     headers: ResponseHeaders,
//     origin: IpAddr,
//     url: url::Url,
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct ResponseHeaders {
//     accept: String,
//     host: String,
//     #[serde(flatten)]
//     dynamic: HashMap<String, String>,
// }