// HTTP Endpoint routes
use crate::database::db::DBhandler;
use crate::database::decode::decompress_string;
use crate::database::types::{
    job_start, BroadcastInput, BroadcastStatus, ScenarioInfoOut, GenericOut, GetUrlResponse, ScenarioInfo,
    UrlResponse, Urldata,
};
use crate::jobs::threads::{thread_status, ThreadManager}; // ThreadInfo
use crate::scenarios::scenario_parse::{scenario_information, multi_scenario_info};
use crate::scenarios::scenario_types::Graph;
use actix_web::{get, post, web, HttpResponse, Result};
use std::sync::Arc;

#[get("/")]
pub async fn info() -> HttpResponse {
    HttpResponse::Ok().body("Documentation: https://xcmsend.github.io/api/index.html")
}

// open channels, list open ingoing and outgoing hrmp channels for paraid
#[post("/polkadot/openchannels")]
pub async fn dot_openchannels() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

// broadcast input: {chain: 'hydradx', tx: ''}
#[post("/broadcast")]
pub async fn broadcast_tx(data: web::Json<BroadcastInput>) -> web::Json<BroadcastStatus> {
    web::Json(BroadcastStatus {
        status: "fail".to_string(),
        hash: "not found".to_string(),
    })
}

#[post("/saveUrl")]
pub async fn save_url(
    data: web::Json<Urldata>,
    db: web::Data<DBhandler>,
) -> web::Json<UrlResponse> {
    println!("saveurl: {:?}", data);
    let shortid = db.saveurl(data.into_inner()).expect("Could not save to db");
    println!("Data saved!");
    println!("Short id generated: {:?}", shortid);

    web::Json(UrlResponse {
        success: true,
        shortUrl: shortid.to_owned(),
    })
}

#[get("/getUrl/{name}")]
pub async fn get_url(
    name: web::Path<String>,
    db: web::Data<DBhandler>,
) -> web::Json<GetUrlResponse> {
    let fluff = format!("Todo {name}!");
    println!("{:?}", fluff);

    match db.get_entry(name.to_string()) {
        Ok(out) => {
            println!("Output: {:?}", out);
            return web::Json(GetUrlResponse {
                success: true,
                longUrl: out.to_owned(),
            });
            // return HttpResponse::Ok().body("Found entry!");
        }
        Err(err) => web::Json(GetUrlResponse {
            success: false,
            longUrl: "not found".to_string(),
        }),
    };

    web::Json(GetUrlResponse {
        success: false,
        longUrl: "not found".to_string(),
    })
}

#[post("/xcm-asset-transfer")]
pub async fn xcm_asset_transfer() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

#[post("/job/start")]
pub async fn start_job(
    data: web::Json<job_start>,
    db: web::Data<DBhandler>,
) -> web::Json<GenericOut> {
    let my_data: job_start = data.into_inner();

    return web::Json(GenericOut {
        success: true,
        result: "Job started".to_string(),
    });
}



/*
curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWvg"}' http://localhost:8081/scenario/info -v
{"success":true,"result":[{"source_chain":"polkadot","source_address":"5GdvmQtUwByTt6Vkx41vtWvg5guyaH3BL2yn6iamg1RViiKD","dest_chain":"assetHub","dest_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","assetid":"0","amount":"1","txtype":"swap","tx":"not set"},{"source_chain":"assetHub","source_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","dest_chain":"hydraDx","dest_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","assetid":"3","amount":"2","txtype":"swap","tx":"not set"},{"source_chain":"hydraDx","source_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","dest_chain":"hydraDx","dest_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","assetid":"5","amount":"2","txtype":"swap","tx":"not set"}]}
*/

#[post("/scenario/info")]
pub async fn scenario_info(
    data: web::Json<ScenarioInfo>,
    db: web::Data<DBhandler>,
) -> web::Json<ScenarioInfoOut> {
    println!("scenario info got input: {:?}", data);
    let name = data.into_inner().id;
    // geturl
    match db.get_entry(name.to_string()) {
        Ok(out) => {
            println!("Output: {:?}", out);

            // decode blob
            let decoded = decompress_string(out)
                .await
                .expect("Failed to decompress string, invalid value");
            println!("decoded ok");
            println!("Decoded as: {}", decoded);
            // Decoded diagram data json
            let graph: Graph =
                serde_json::from_str(decoded.as_str()).expect("Failed to parse JSON");

            println!("decoded okay");
            // parse scenario
            println!("parsing scenario_information");
            let output_string = scenario_information(graph.clone()).expect("could not parse scenario");
            println!("parsing scenario_information ok");
            println!("parsing multi_scenario_info");
            let o2 = multi_scenario_info(graph.clone());
            println!("parsing multi_scenario_info ok");
            println!("parsing multi_scenario_info: {:?}", o2);
            return web::Json(ScenarioInfoOut {
                success: true,
                result: Some(o2),
            });
        }
        Err(err) => {
            return web::Json(ScenarioInfoOut {
                success: false,
                result:  None,
            })
        }
    };

    return web::Json(ScenarioInfoOut {
        success: false,
        result: None, 
       // result: Vec::new(),
    });
    //HttpResponse::Ok().body("wip")
}

// scenario workers
#[get("/scenario/all_workers")]
pub async fn list_all_threads(data: web::Data<Arc<ThreadManager>>) -> HttpResponse {
    let active_threads = data.get_active_threads();
    println!("listning threads!");
    HttpResponse::Ok().json(active_threads)
}

/// query single scenario worker
#[post("/scenario/worker/")]
pub async fn list_single_thread(
    postdata: web::Json<ScenarioInfo>,
    data: web::Data<Arc<ThreadManager>>,
) -> web::Json<thread_status> {
    let scenario_id = postdata.into_inner().id;
    let thread_info = data.get_thread_status(scenario_id);

    return web::Json(thread_info);
}

// get execution logs | get the history of the executed scenario
#[post("/scenario/worker/logs")]
pub async fn get_logs() -> web::Json<GenericOut> {
    return web::Json(GenericOut {
        success: false,
        result: "not found".to_string(),
    });
}

// test a http action
#[post("/action/http/dry_run")]
pub async fn dry_run_http() -> web::Json<GenericOut> {
    return web::Json(GenericOut {
        success: false,
        result: "not found".to_string(),
    });
}
