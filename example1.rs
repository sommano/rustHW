estern mod extra;
use extra::arc::Arc;

fn main() {
    let numbers = [1,2,3];

    let numbers_arc = Arc::new(numbers);

    for num in range(0, 3) {
        let local_arc = port.recv();
        let task_numbers = local_arc.get();
        println!("{:d}", task_numbers[num]);
    }
}