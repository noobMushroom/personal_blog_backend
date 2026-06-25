use personal_blog::{configuration::get_configuration, startup::Application, telemetry};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = telemetry::get_subscriber("Personal Blog".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let configuration = get_configuration().expect("failed to get configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
