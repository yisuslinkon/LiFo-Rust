use lifo::Stack;

fn main() {
    let mut stack1: Stack<u16> = Stack::new();
    let mut stack2: Stack<u16> = Stack::new();
    stack2.push(1);

    println!("The first stack: {:?}", stack1);
    println!("The second stack: {:?}", stack2);

    assert!(stack1.is_empty());
    assert!(!stack2.is_empty());

    stack1.push(1);
    stack2.push(2);

    println!("The first stack: {:?}", stack1);
    println!("The second stack: {:?}", stack2);

    stack1.pop();
    stack2.pop();

    assert!(stack1.is_empty());
    assert!(!stack2.is_empty());

    stack2.push(2);
    stack2.push(3);
    stack2.push(4);
    stack2.push(5);

    println!("The last one number is on the Stack 1: {:?}", stack1.peek());
    println!("The last one number is on the Stack 2: {:?}", stack2.peek());

    // println!("The first stack: {:?}", &stack1);
    // println!("The second stack: {:?}", &stack2);
}
