mod formatter;
mod parser;
mod pattern;
mod task;
mod files;

use parser::Target;

fn main() {
    let f = formatter::Format::new(formatter::Format::TEXT);
    for item in files::list("./examples") {
        for kind in task::Type::list() {
            f.start_section(kind.target());

            let comment = parser::CommentStyle::from_file(&item.kind);
            let code = files::read(&item.path);
            let mut ext = pattern::Extractor::new(&kind, &comment, code);

            loop {
                match ext.get_task() {
                    Some(task) => {
                        f.delimiter(formatter::Delimiter::TASK, formatter::Point::START);
                        f.render(task);
                        f.delimiter(formatter::Delimiter::TASK, formatter::Point::END);
                    }
                    None => break,
                };
            }
            f.end_section();
        }
    }
}

// fn main2() {
//     let additional_types = vec![String::from("custom")];
//     let f = formatter::Format::new(formatter::Format::TEXT);
//     let comment = parser::CommentStyle::Slc;
//     for kind in task::Type::list_with_additional(Some(additional_types)) {
//         f.start_section(kind.target());

//         let mut ext = pattern::Extractor::new(&kind, &comment, CODE.to_string());

//         loop {
//             match ext.get_task() {
//                 Some(task) => {
//                     f.delimiter(formatter::Delimiter::TASK, formatter::Point::START);
//                     f.render(task);
//                     f.delimiter(formatter::Delimiter::TASK, formatter::Point::END);
//                 }
//                 None => break,
//             };
//         }
//         f.end_section();
//     }
// }
