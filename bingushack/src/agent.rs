// atm taken from the example agent
// https://github.com/robberphex/jvmti-rust/blob/de8b7b139ea43fea58b41a7ce7447a65442e636e/sample/src/agent.rs#L1
/*
 * TODO The functions below are essentially parts of an actual client implementation. Because this
 * implementation is highly experimental and incomplete they shall remain here for a while but
 * they will have to find a new home, eventually
 */

use jvmti::{runtime::{MethodInvocationEvent, ObjectAllocationEvent, ClassFileLoadEvent}, context::static_context, thread::Thread, instrumentation::asm::transformer::Transformer, bytecode::{Constant, printer::ClassfilePrinter}};

pub fn on_method_entry(event: MethodInvocationEvent) {
    let shall_record = match static_context().config.read() {
        Ok(cfg) => (*cfg).entry_points.iter().any(|item| *item == format!("{}.{}.{}", event.class_sig.package, event.class_sig.name, event.method_sig.name) ), //event.class_name.as_str() == item),
        _ => false
    };

    if !shall_record {
        println!("[M-{}.{}{}]", event.class_sig.package, event.class_sig.name, event.method_sig.name);
    }

    static_context().method_enter(&event.thread.id);
}

pub fn on_method_exit(event: MethodInvocationEvent) {
    match static_context().method_exit(&event.thread.id) {
        //Some(_) => (),
        Some(duration) => println!("Method {} exited after {}", event.method_sig.name, duration),
        None => println!("Method has no start: {}", event.method_sig.name)
    }
}

pub fn on_thread_start(thread: Thread) {
    println!("[TS-{}]", thread.name);

    static_context().thread_start(&thread.id);
}

pub fn on_thread_end(thread: Thread) {
    println!("[TE-{}]", thread.name);

    match static_context().thread_end(&thread.id) {
        Some(duration) => println!("Thread {} lived {}", thread.name, duration),
        None => println!("Thread {} has no start", thread.name)
    }
}

pub fn on_monitor_wait(thread: Thread) {
    println!("[W1-{}]", thread.name);
}

pub fn on_monitor_waited(thread: Thread) {
    println!("[W2-{}]", thread.name);
}

pub fn on_monitor_contended_enter(thread: Thread) {
    println!("[C1-{}]", thread.name);

    static_context().monitor_enter(&thread.id);
}

pub fn on_monitor_contended_entered(thread: Thread) {
    println!("[C2-{}]", thread.name);

    match static_context().monitor_entered(&thread.id) {
        Some(duration) => println!("Thread {} waited {}", thread.name, duration),
        None => println!("Thread {} has never waited", thread.name)
    }
}

pub fn on_class_file_load(mut event: ClassFileLoadEvent) -> Option<Vec<u8>> {
    let shall_transform = match static_context().config.read() {
        Ok(cfg) => (*cfg).entry_points.iter().any(|item| item.starts_with(event.class_name.as_str())), //event.class_name.as_str() == item),
        _ => false
    };

    if shall_transform {
        {
            let mut transformer = Transformer::new(&mut event.class);
            let result = transformer.ensure_constant(Constant::Utf8(String::from("Cde").into_bytes()));

            println!("Result: {:?}", result);
        }
        let _: Vec<()> = ClassfilePrinter::render_lines(&event.class).iter().map(|line| println!("{}", line)).collect();
    }
/*
    let output_class: Vec<u8> = vec![];
    let mut write_cursor = Cursor::new(output_class);

    let mut new_class = event.class;

    new_class.constant_pool.constants = new_class.constant_pool.constants.into_iter().map(|constant| {
        match constant {
            Constant::Utf8(bytes) => String::from_utf8(bytes.clone()).map(|string| match string.as_str() {
                "Hello World" => Constant::Utf8(String::from("Lofasz").into_bytes()),
                _ => Constant::Utf8(string.into_bytes())
            }).unwrap_or(Constant::Utf8(bytes)),
            other @ _ => other
        }
    }).collect();

    let result = {
        let mut writer = ClassWriter::new(&mut write_cursor);
        writer.write_class(&new_class)
    };

    if let Ok(_) = result {
        Some(write_cursor.into_inner())
    } else {
        None
    }
    */
    None
}

pub fn on_garbage_collection_start() {
    println!("GC Start: {:?}", std::time::Instant::now());
}

pub fn on_garbage_collection_finish() {
    println!("GC Finish: {:?}", std::time::Instant::now());
}

pub fn on_object_alloc(event: ObjectAllocationEvent) {
    println!("Object allocation: (size: {})", event.size);
}

pub fn on_object_free() {
    println!("Object free");
}