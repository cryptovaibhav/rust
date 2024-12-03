// Rules for conversion to postfix - 
// 1. Priorities of operators 
//      + , - = 1
//      * , / = 2
//      ^ = 3
// 2. If scanned operator is <= priority of operator on top of stack then pop operators until we have a low / equal priority. Add 
//      the poped elements to postfix 
// 3. If "(", push it to the stack 
// 4. If ")", pop elements from stack until "(" is found. Add poped elements to postfix 
// 5. If operand then just add to postfix 

fn main() {
    let expression: String = String::from("( 33 + 45 / 3 * ( 2 + 9 ) - 50 )");
    println!("Expression is: {}", expression);

    let postfix = convert_to_postfix(&expression);

    println!("Final postfix is: {:?}", postfix);

    let result = evaluate(postfix);
    println!("Result is: {}", result);
}

fn convert_to_postfix(expression: &str) -> Vec<&str> {
    let mut stack: Vec<&str> = Vec::new();
    let mut postfix: Vec<&str> = Vec::new();

    let array: Vec<&str> = expression.split(" ").collect();
    // println!("{:?}", array);

    for i in array {
        // println!("Current value is {}", i);

        if is_operator(i) {
            if i == "(" {
                stack.push(i);
            } else if i == ")" {
                loop {
                    let popped_item = stack.pop().unwrap();
                    if popped_item == "(" {
                        break;
                    } else if popped_item != ")" {
                        postfix.push(popped_item);
                    }
                }
            } else {
                let mut top_of_stack = stack[stack.len() - 1];
                // println!("Top of stack is: {}", top_of_stack);
                let current_priority = get_priority(i);
                // println!("Current priority is: {}", current_priority);
                let mut top_priority = get_priority(top_of_stack);
                // println!("Top priority is: {}", top_priority);
                if current_priority > top_priority {
                    stack.push(i);
                } else {                    
                    while current_priority <= top_priority {
                        let popped_item = stack.pop().unwrap();
                        postfix.push(popped_item);
                        top_of_stack = stack[stack.len() - 1];      // we can use the stack.last() to get the last element as well                   
                        top_priority = get_priority(top_of_stack); 
                        
                        if stack.last() == None { // if stack becomes empty 
                            break;
                        }                                              
                    }
                    stack.push(i);
                }
            }
            
            // stack.push(i);
        } else {
            postfix.push(i);
        }

        // println!("Stack is: {:?}", stack);
        // println!("Postfix is: {:?}", postfix);
    }

    postfix
}

fn is_operator(item: &str) -> bool {
    let result = match item {
        "+" | "-" | "/" | "*" | "(" | ")" => true,
        _ => false
    };
    
    result
}

fn get_priority(operator: &str) -> u8 {
    let priority: u8 = match operator {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        _ => 0
    };

    // println!("Current operator is {} and priority is {}", operator, priority);
    priority
}


// RULES FOR POSTFIX EVALUATION 
// 1. If operand -> push to stack 
// 2. If operation -> pop two elements, perform the operation and push the result to stack

// ["33", "45", "3", "/", "2", "9", "+", "*", "+", "50", "-"]
fn evaluate(postfix: Vec<&str>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    
    for i in postfix {
        if is_operator(i) {
            let first_popped_item: i32 = stack.pop().unwrap();
            let second_popped_item: i32 = stack.pop().unwrap();
            let result = perform_operation(i, first_popped_item, second_popped_item);
            stack.push(result);
        } else {
            let item: i32 = i.parse().expect("Failed to parse");
            stack.push(item);
        }
    }

    stack[0]
}

fn perform_operation(operator: &str, item_1: i32, item_2: i32) -> i32 {    
    let result = match operator {
        "+" => item_1 + item_2,
        "-" => item_2 - item_1,
        "*" => item_1 * item_2,
        "/" => item_2 / item_1,
        _ => 0
    };

    result
}