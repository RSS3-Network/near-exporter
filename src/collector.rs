use std::vec;

use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    Gauge, Opts,
};

use crate::near;

pub struct ViewAccountMetricsCollector {
    desc: Desc,
    url: String,
    account_id: String,
    result: Option<near_primitives::views::AccountView>,
}

impl ViewAccountMetricsCollector {
    pub fn new(desc: Desc, url: String, account_id: String) -> Self {
        ViewAccountMetricsCollector {
            desc,
            url,
            account_id,
            result: None,
        }
    }

    pub async fn fetch(&mut self) {
        let result = near::fetch_data(self.url.clone(), self.account_id.clone()).await;
        match result {
            Ok(account) => {
                self.result = Some(account);
            }
            Err(e) => {
                println!("Failed to fetch data: {}", e);
            }
        }
    }
}

impl Collector for ViewAccountMetricsCollector {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.desc]
    }

    fn collect(&self) -> Vec<MetricFamily> {
        let mut families = vec![];

        let account = match &self.result {
            Some(account) => account,
            None => return families,
        };

        let opts = Opts::new("balance", "NEAR Protocol account balance")
            .namespace(self.desc.fq_name.clone())
            .subsystem("account");

        let g = Gauge::with_opts(opts).unwrap();
        g.set(account.amount as f64);

        families.extend(g.collect().iter().cloned());

        families
    }
}
