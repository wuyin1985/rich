use hashtoollib::HashTool;

fn main() {
    let mut h = HashTool::new();
    h.hash("test");
    h.save_reverse_dict_2_file("d:/test.txt");
    println!("end!");
}
