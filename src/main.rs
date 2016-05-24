extern crate tree;
extern crate time;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use tree::istree;

fn read_adj_matrix() -> (Vec< Vec<usize> >,usize) {
        /*
        Input Is given As Adjacency List Style
        eg:
        5
        1 2 3
        2 1
        3 1
        this will be converted into adjacency Matrix by the method
        */
        let path = Path::new("W_input_not_tree_7.txt");
        let display = path.display();
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       Error::description(&why)),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       Error::description(&why)),
            Ok(_) => println!("Input read..{}",display),// print!("{} contains:\n{}", display, s),
        }

        //------------------------Adjoint List to Adjoint Matrix---------------------

        let mut v: Vec<&str> = s.rsplit('\n').collect();
        let nodes: usize = v.pop().unwrap().to_string().trim().parse().unwrap();
        let mut adj_list: Vec< Vec<usize> > = vec![vec![0;nodes];nodes];
        v.reverse();
        let mut count_r = 0;
        let mut first_row = 0;
            for i in v.iter(){
                for num in i.split_whitespace() {
                    let n: usize = num.trim().parse().expect("invalid input");
                    if first_row == 0{
                        count_r = n;
                        first_row += 1;
                        continue;
                    }
                    adj_list[count_r - 1][n - 1] = 1;
                 }
                 first_row = 0;
            }
         return (adj_list,nodes);
}


fn main(){

        let ( mut adj_list,nodes) = read_adj_matrix();
    //    println!("{:?}",adj_list );
        println!("Nodes     :   {}", nodes );

        let p_start = time::get_time();

        istree::check_tree(&mut adj_list,nodes);

        let p_stop = time::get_time();
        let dur_sec = p_stop.sec - p_start.sec;
        let mut duration: f64 = 0.0;
        if dur_sec == 0{
            let dur_nsec = p_stop.nsec - p_start.nsec;
            duration = (dur_nsec as f64)/100_000_00000.0; // + 0
        }else if dur_sec >= 1 {
            let diff_from_1_now : i64= 1000_000_00_000 as i64- p_start.nsec as i64;
            let duration_sec = dur_sec - 1;
            let calc_p = (duration_sec * 1000_0000_0) + diff_from_1_now as i64 + p_stop.nsec as i64; // - 0
            duration = (calc_p as f64)/1000_000_0000.0; // + 0
        }
        println!("Parallel Running Time     : {} Seconds",duration);


}
