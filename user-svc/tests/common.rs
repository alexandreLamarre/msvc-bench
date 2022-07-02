use docker_client::container::Config;
use docker_client::DockerClient;
use docker_client::DockerError;

fn setup_test_env() -> Result<(), Box<dyn std::error::Error>> {
    let client = DockerClient::connect("/var/run/docker.sock");
    let config = Config::with_image("mongo").name("mongo-test").build();
    let container = match client.create_container(config) {
        Err(e) => return e,
        Ok(cont) => cont,
    };
    client.start_container(container, "-d");
    Ok(())
}
