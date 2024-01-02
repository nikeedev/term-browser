use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal, QueueableCommand,
    style::Stylize,
};
use std::{env, process, fs, io::{stdout, Write, Result}};
use minidom::Element;

#[derive(Debug)]
struct TermElement<'a> {
    children: Vec<TermElement<'a>>,
    style: & 'a str,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading file: {}", x)
        }
    } else {
        println!("{}", "term-browser - by nikeedev - v0.1.0".blue());
        println!("{}", "\nUsage: term-browser <filename>.html or".cyan());
        println!("{}", "\n       term-browser <url>".cyan());
        process::exit(1);
    };
    
    println!("{}", file);

    let mut stdout = stdout();

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
 
    stdout.queue(terminal::SetTitle(format!("{} - term-browser", args[1].clone())))?;
    
    let root: Element = file.parse().unwrap();

    // let mut elements: Vec<TermElement> = Vec::new();

    // for child in root.children() {
        
    // }

    println!("{:#?}", root);

    // 'app_loop: loop {
    //     let event = read().unwrap();

    //     // if let Event::Key(event) = event {
    //     //     println!("{:?}", event.code);
    //     // };
    //     if let Event::Key(event) = event {
    //         if event.kind == KeyEventKind::Press {
    //             match event.code {
    //                 KeyCode::Char(character) => {
    //                     if character == 'q' && event.modifiers == KeyModifiers::CONTROL {
    //                         break 'app_loop;
    //                     };
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     let _ = stdout.write(file.as_bytes());
    //     stdout.flush().unwrap();
    // }

    Ok(())
}
