use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn search_in_file(filename: &Path, search_text: &str) -> io::Result<()> {
    // Проверка на существование файла
    if !filename.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Файл не найден: {:?}", filename)));
    }

    // Открытие файла
    let file = fs::File::open(filename).map_err(|e| {
        io::Error::new(e.kind(), format!("Ошибка при открытии файла '{}': {}", filename.display(), e))
    })?;

    let reader = io::BufReader::new(file);

    // Поиск текста в каждой строке файла
    for line in reader.lines() {
        let line = line?;
        if line.contains(search_text) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    // Получение аргументов командной строки
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Использование: {} <имя_файла> <текст_для_поиска>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let search_text = &args[2];

    // Выполнение поиска
    if let Err(e) = search_in_file(Path::new(filename), search_text) {
        eprintln!("Ошибка: {}", e);
        std::process::exit(1);
    }
}
