pub mod util {
    pub fn print_2d_vec_with_indexes(grid: &Vec<Vec<char>>) {
        println!();
        if grid.is_empty() || grid[0].is_empty() {
            return;
        }

        print!("  ");
        for j in 0..grid[0].len() {
            print!("{:3} ", j);
        }
        println!();

        for (i, row) in grid.iter().enumerate() {
            print!("{:3} ", i);
            for &val in row {
                print!("{:3} ", val);
            }

            println!();
        }
        println!();
    }

    pub fn print_2d_vec_with_indexes_bool(grid: &Vec<Vec<bool>>) {
        if grid.is_empty() || grid[0].is_empty() {
            return;
        }

        print!("  ");
        for j in 0..grid[0].len() {
            print!("{:3} ", j);
        }
        println!();

        for (i, row) in grid.iter().enumerate() {
            print!("{:3} ", i);
            for &val in row {
                print!("{:3} ", val);
            }
            println!();
        }
    }
}
