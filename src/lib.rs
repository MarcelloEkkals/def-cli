pub use serde_json;
pub use reqwest;

pub mod defender {

    use reqwest;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use serde_json;
    use serde_json::{Value};
    use std::fs::File;
    use std::io::Read;
    use std::{thread, time};

    pub struct Api {
        token: &'static str,
    }

    impl Api {
        pub async fn scan(&self, file: &str) {
            let url = "https://api.metadefender.com/v4/file/";
            let mut file = File::open(file).unwrap();
            let mut vec = Vec::new();
            let client = reqwest::Client::new();

            file.read_to_end(&mut vec).unwrap();
            let res = client.post(url)
                      .headers(self.construct_headers())
                      .body(vec)
                      .send()
                      .await
                      .unwrap();
            let resp = res.text().await.unwrap();
            let resp_data: Value = serde_json::from_str(&resp).unwrap();
            thread::sleep(time::Duration::from_secs(5));
            let respo = client.get(format!("{}{}", url, resp_data["data_id"]).replace("\"", ""))
                        .headers(self.construct_headers())
                        .send()
                        .await
                        .unwrap();
            let txt = respo
                       .text()
                       .await
                       .unwrap();
            let parsed: Value = serde_json::from_str(&txt).unwrap();
            let all_av = [
                "Lavasoft",
                "StopZilla",
                "Zillya",
                "VirusBlokAda",
                "TrendMicro",
                "SUPERAntiSpyware",
                "NProtect",
                "Nanoav",
                "FSecure",
                "Eset",
                "BitDefender",
                "Baidu",
                "Ahnlab",
                "AegisLab",
                "Zoner",
                "Sophos",
                "Preventon",
                "McAfee",
                "K7",
                "Jiangmin",
                "Hauri",
                "FProt",
                "Fortinet",
                "Filseclab",
                "Emsisoft",
                "ClamAV",
                "Avira",
                "Avg",
                "Agnitum",
                "Ikarus",
                "Cyren",
                "MicrosoftSecurityEssentials",
                "QuickHeal",
                "TotalDefense",
                "TrendMicroHouseCall",
                "XvirusPersonalGuard",
                "DrWebGateway",
                "VirITEXplorer"
            ];
            for element in &all_av {
                println!("{} founded {} thread({})", element, parsed["scan_results"]["scan_details"][element]["scan_result_i"], parsed["scan_results"]["scan_details"][element]["threat_found"]);
            }
        }
        pub fn new() -> Self {
            Self {
                token: "d472816094e275309553b3d9c9d4b42d",
            }
        }
        fn construct_headers(&self) -> HeaderMap {
            let mut header = HeaderMap::new();
            header.insert("apikey", HeaderValue::from_static(self.token));
            header.insert(CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"));
            
            header
        }
    }

}
