use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use bollard::exec::StartExecResults;
use bollard::models::{ContainerCreateBody, ExecConfig};
use bollard::query_parameters::{
    CreateContainerOptions, CreateImageOptions, StartContainerOptions,
};
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::env::args;
use std::str::FromStr;
use tokio::sync::OnceCell;

static DOCKER: OnceCell<Docker> = OnceCell::const_new();
static CONTAINER: OnceCell<String> = OnceCell::const_new();

#[post("/sh")]
async fn sh(body: String) -> impl Responder {
    let docker = DOCKER.get().unwrap();
    let container = CONTAINER.get().unwrap();

    let exec = match docker
        .create_exec(
            container,
            ExecConfig {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["/bin/sh".to_string(), "-c".to_string(), body]),
                ..Default::default()
            },
        )
        .await
    {
        Ok(exec) => exec,
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };

    let result = match docker.start_exec(&exec.id, None).await {
        Ok(exec) => exec,
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };

    match result {
        StartExecResults::Attached { mut output, .. } => {
            let mut buffer: Vec<u8> = Vec::new();
            while let Some(Ok(msg)) = output.next().await {
                buffer.extend_from_slice(msg.as_ref());
            }

            HttpResponse::Ok().body(buffer)
        }
        StartExecResults::Detached => HttpResponse::Created().body("Detached"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::connect_with_defaults()?;

    let [ref image, ref tag, ref port] = args().collect::<Vec<String>>()[..3] else {
        eprintln!("insufficient arguments");
        return Ok(());
    };

    let port = u16::from_str(&port)?;

    let mut image_stream = docker.create_image(
        Some(CreateImageOptions {
            from_image: Some(image.clone()),
            tag: Some(tag.clone()),
            ..Default::default()
        }),
        None,
        None,
    );

    while let Some(result) = image_stream.next().await {
        if let Some(status) = result?.status {
            eprintln!("{}", status);
        }
    }

    let container = docker
        .create_container(
            Some(CreateContainerOptions {
                name: None,
                ..Default::default()
            }),
            ContainerCreateBody {
                image: Some(format!("{}:{}", image, tag)),
                cmd: Some(vec!["sleep".to_string(), "infinity".to_string()]),
                ..Default::default()
            },
        )
        .await?;

    for warning in container.warnings {
        eprintln!("{}", warning);
    }

    docker
        .start_container(&container.id, None::<StartContainerOptions>)
        .await?;

    println!("{}", &container.id);

    DOCKER.set(docker).unwrap();
    CONTAINER.set(container.id).unwrap();

    HttpServer::new(|| App::new().service(sh))
        .bind(("127.0.0.1", port))?
        .run()
        .await?;

    Ok(())
}
