
mod phenotypes;

pub async fn run() {
  phenotypes::run_phenotypes().await;
}