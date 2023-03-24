use std::env;

fn client_speak_fifo(command: &str, id: Option<&str>) {
    match id {
        Some(id) => println!("Command received: {} with id {}", command, id),
        None => println!("Command received: {}", command),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide an argument");
        return;
    }
    match args[1].as_str() {
        "print" => client_speak_fifo("print", None),
        "info" => {
            if args.len() < 3 {
                println!("Please provide an id for the info command");
                return;
            }
            let id = &args[2];
            client_speak_fifo("info", Some(id));
        },
        "copy" => {
            if args.len() < 3 {
                println!("Please provide an id for the copy command");
                return;
            }
            let id = &args[2];
            client_speak_fifo("copy", Some(id));
        },
        "delete" => {
            if args.len() < 3 {
                println!("Please provide an id for the delete command");
                return;
            }
            let id = &args[2];
            client_speak_fifo("delete", Some(id));
        },
        "save" => client_speak_fifo("save", None),
        "daemon" => println!("Daemon selected"),
        "help" => println!("Help selected"),
        _ => println!("Invalid argument"),
    }
}
