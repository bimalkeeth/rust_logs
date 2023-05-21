#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused)]

pub fn expression() {
    let precompute = {
        let a = (-34i64).abs();
        let b = 345i64.pow(3);
        let c = 3;

        a + b + c
    };
}

#[derive(Debug)]
struct Items(u32);

pub fn expression2() {
    let items = Items(2);
    let items_ptr = &items;
    let ref items_ref = items;

    assert_eq!(items_ptr as *const Items, items_ref as *const Items);

    let mut a = Items(20);
    {
        let ref mut b = a;
        b.0 += 25;
    }

    println!("{:?}", items);
    println!("{:?}", a);
}

enum Food {
    Pizza,
    Salad,
}

enum PaymentMode {
    BitCoin,
    Credit,
}

struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode,
}

pub fn expression_3() {
    let food_order = Order {
        count: 2,
        item: Food::Salad,
        payment: PaymentMode::Credit,
    };

    let Order { count, item, .. } = food_order;
    println!("hi")
}

struct Person(String);

pub fn expression_4() {
    let a = Person("richard fynman".to_string());

    match a {
        Person(ref name) => println!("{} was a great physicist", name),
        _ => panic!("oh no")
    }

    let b = a;
}

struct Container {
    items_count: u32,
}

fn increment_item(Container { ref mut items_count }: &mut Container) {
    *items_count += 1;

    println!("{}",items_count);
}

fn calculate_cost(Container { items_count }: &Container) -> u32 {
    let rate = 7;
    rate * items_count
}

pub fn expression_5() {
    let mut container = Container {
        items_count: 0
    };

    increment_item( &mut container);
    let total_cost = calculate_cost(&container);

    println!("total cost {}",total_cost)
}

pub struct ParsedPayload<T> {
    inner: T
}
pub struct ParseError<E> {
    inner: E
}

type ParserResult<T,E>=Result<ParsedPayload<T>,ParseError<E>>;

pub fn parse_payload<T,E>(stream:&[u8])->ParserResult<T,E>{
    unimplemented!();
}