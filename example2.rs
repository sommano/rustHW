estern mod extra;
use extra::arc::RWArc;

fn main() {
    let numbers =[1,2,3];

    let numbers_arc = RWArc::new(numbers);

    for num in range(0, 3) {
        let (port, chan) = Chan::new();

        local_arc.write(|nums| {
            nums[num] += 1
        });
        local_arc.read(|nums| {
            println!("{:d}", nums[num]);
        })
    }
}