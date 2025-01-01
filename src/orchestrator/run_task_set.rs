use std::collections::HashMap;
use std::fs;
use std::error::Error;
use serde::Deserialize;
use runautils::file_utils::get_tmp_file_path;
use runautils::bash_util::run_bash_script;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use actix::prelude::*;
use crate::orchestrator::ws_handle_task_request::WebSocketActor;
use actix_web_actors::ws;

#[derive(Deserialize, Debug)]
struct TaskNode {
    label: String,
    script: String,
    #[serde(rename = "type")]
    node_type: String,
    category: String,
    children: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct TaskSet {
    task_set_nodes: Vec<TaskNode>,
}

pub fn process_run_task_set(task_set_json_str: String,
                            ctx: &mut ws::WebsocketContext<WebSocketActor>) -> Result <String, Box<dyn Error>> {

    println!("task_set_json_str : {:?}", task_set_json_str);

    let task_list :TaskSet =  serde_json::from_str(task_set_json_str.as_str())?;
    for task_node in task_list.task_set_nodes {
        println!("task_set_node : {:?}", task_node.node_type);
        match task_node.node_type.as_str() {
            "bash" => {
                match process_bash_command_with_outpu_streaming(&task_node, ctx) {
                    Ok(msg) => {},
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            _ => {
                println!("Unknown task node type: {}", task_node.node_type);
            }
        }
    }
    Ok("success".to_string())
}

fn process_bash_command_with_outpu_streaming(
    task_node: &TaskNode,
    ctx: &mut ws::WebsocketContext<WebSocketActor>,
) -> Result<String, Box<dyn Error>> {
    if task_node.script.is_empty() {
        return Err("empty script".into());
    }

    let tmp_file_name = get_tmp_file_path("/tmp");
    fs::write(&tmp_file_name, &task_node.script)?;
    println!("Script written to temporary file: {:?}", tmp_file_name);
    let mut env_vars = HashMap::<String, String>::new();

    let tmp_file_str = tmp_file_name
        .to_str()
        .ok_or_else(|| "Could not convert temporary file path to string")?;

    let mut command = Command::new("bash")
        .arg(tmp_file_str)
        .current_dir("/tmp")
        .envs(&env_vars)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Capture stdout and stderr
    if let Some(stdout) = command.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let line = line?;
            println!("stdout: {}", line); // Log locally
            ctx.text(format!("{}", line)); // Send to WebSocket
        }
    }

    if let Some(stderr) = command.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            let line = line?;
            eprintln!("stderr: {}", line); // Log locally
            ctx.text(format!("{}", line)); // Send to WebSocket
        }
    }

    let status = command.wait()?;
    if !status.success() {
        return Err(format!("Script failed with exit code: {}", status).into());
    }

    Ok("ok".to_string())
}

//
//
