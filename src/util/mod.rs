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

    pub fn print_2d_vec_with_indexes_d(grid: &Vec<Vec<char>>) {
        println!();
        if grid.is_empty() || grid[0].is_empty() {
            return;
        }

        print!("  ");
        for j in 0..grid[0].len() {
            print!("{:8} ", j);
        }
        println!();

        let mut c = 0;
        for (i, row) in grid.iter().enumerate() {
            print!("{:3} ", i);
            for &val in row {
                print!("{:3}({:3}) ", val,c);
                c += 1;
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
                if val {print!("T   ");}
                else {print!("F   ");}
            }
            println!();
        }
    }
}
