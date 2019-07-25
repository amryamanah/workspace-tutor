use chap15_smart_pointer;
use chap16_concurrency;
use chap17_oop;
use blog::postv1 as StatePost;
use blog::postv2 as TypePost;
use chap18_pattern;
use chap20_webserver;

pub fn main_chap15() {
    chap15_smart_pointer::main_box();
}

pub fn main_chap16() {
    chap16_concurrency::main();
}

pub fn main_chap17() { chap17_oop::main(); }

pub fn main_blog() {
    let mut post = StatePost::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = TypePost::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

pub fn main_chap18() { chap18_pattern::main() }

pub fn main_chap20() {
//    chap20_webserver::single::main();
    chap20_webserver::multithreaded::main();
}

fn main() {
    main_chap20();
}
