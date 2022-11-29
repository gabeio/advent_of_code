use std::io::{self, Read};
use std::sync::Arc;

fn main() -> io::Result<()> {
    let buffer = readin();
    //println!("{:?}", buffer);
    let mut vstr: Vec<&str> = buffer.split('\n').collect();
    vstr.pop();
    let vstr: Vec<&str> = vstr;
    //println!("{:?}", vstr);
    //let convert = |x: &str| String::from(x).parse().unwrap();
    //let vint: Vec<u32> = vstr.to_vec().into_iter().map(convert).collect();
    //println!("{:?}", vint);
    println!("{:?}", part1(&vstr));
    println!("{:?}", part2(&vstr));
    Ok(())
}

#[derive(Debug,Clone,Copy)]
struct Number {
    value: u32,
    called: bool
}

impl Number {
    fn was_called(&mut self) {
        self.called = true;
    }
}

fn readin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _result = handle.read_to_string(&mut buffer);
    return buffer;
}

fn convert(x: &str) -> Arc<Number> {
    //println!("{:?}", x);
    return Arc::new(Number {
        value: x.parse().unwrap(),
        called: false
    });
}

fn pop_empty(vstr: &mut Vec<&str>) {
    let mut i = 0;
    while vstr.len() > 0 && i < vstr.len(){
        if vstr[i] == "" {
            vstr.remove(i);
        }
        i += 1;
    }
}

fn parse_boards(input: Vec<&str>) -> Arc<Vec<Arc<Vec<Arc<Vec<Arc<Number>>>>>>> {
    let mut boards: Arc<Vec<Arc<Vec<Arc<Vec<Arc<Number>>>>>>> = Arc::new(Vec::new());
    //println!("boards {:?}", boards);
    //println!("{:?}", input);
    let mut row_cd = 5;
    //println!("while");
    let mut i = 0;
    let mut boardi = 0;
    while input.len() > i {
        if row_cd == 5 {
            // add new board
            Arc::make_mut(&mut boards).push(Arc::new(Vec::new()));
            //println!("boards {:?}", boards);
        }
        let line = input[i];
        //println!("line {:?}", line);
        if line.len() > 0 {
            let mut vstr: Vec<&str> = line.split(' ').collect();
            pop_empty(&mut vstr);
            //println!("vstr {:?}", vstr);
            let row: Vec<Arc<Number>> = vstr.to_vec().into_iter().map(convert).collect();
            //println!("boardi {:?}", boardi);
            //println!("row_cd {:?}", row_cd);
            Arc::make_mut(&mut Arc::make_mut(&mut boards)[boardi]).insert(5-row_cd, Arc::new(row));
            //println!("boards {:?}", boards);
            row_cd -= 1;
        } else {
            boardi += 1;
            row_cd = 5;
        }
        i += 1;
    }
    //println!("boards: {:?}", boards);
    return boards;
}

fn bingo(called: Vec<Arc<Number>>, mut boards: Arc<Vec<Arc<Vec<Arc<Vec<Arc<Number>>>>>>>) -> u32 {
    for i in called { // for each called
        for mut board in Arc::make_mut(&mut boards) {
            for mut row in Arc::make_mut(&mut board) {
                for mut num in Arc::make_mut(&mut row) {
                    if num.value == i.value {
                        Arc::make_mut(&mut num).was_called();
                    }
                }
            }
            let val = check(board.clone());
            if val {
                println!("for i in called: i = {:?}", i);
                let answer = summation(board.clone()) * i.value;
                println!("answer: {}", &answer);
                return answer;
            }
        }
    }
    println!("failed to find a winning board");
    // returns winning board
    return 0;
}

fn check(board: Arc<Vec<Arc<Vec<Arc<Number>>>>>) -> bool {
    // for each board
    // check rows for all true
    let mut rows = false;
    for i in Arc::as_ref(&board) { // columns
        let mut row = true; // default to winner
        for j in Arc::as_ref(i) { // rows
            if j.called != true { // if no winner
                row = false; // this row is not a winner
                break; // break and continue
            }
        }
        if row == true { // if the row is a winner
            rows = true; // the board then is a winner
            break; // and we can stop looking
        }
    }
    // check columns for all true
    let mut columns = false;
    let rboard = Arc::as_ref(&board);
    for i in 0..rboard.len() {
        let mut column = true;
        for j in 0..rboard[i].len() {
            if rboard[j][i].called != true {
                column = false;
                break;
            }
        }
        if column == true {
            columns = true;
            break;
        }
    }
    return rows || columns;
}

fn summation(board: Arc<Vec<Arc<Vec<Arc<Number>>>>>) -> u32 {
    let mut sum = 0;
    for i in Arc::as_ref(&board) { // columns
        for j in Arc::as_ref(i) { // rows
            if j.called != true {
                sum += j.value;
            }
        }
    }
    return sum;
}

fn bingo2(called: Vec<Arc<Number>>, mut boards: Arc<Vec<Arc<Vec<Arc<Vec<Arc<Number>>>>>>>) -> u32 {
    let mut ilist: Vec<usize> = vec![];
    for i in called { // for each called
        let mut iboard: usize = 0;
        for mut board in Arc::make_mut(&mut boards) {
            for mut row in Arc::make_mut(&mut board) {
                for mut num in Arc::make_mut(&mut row) {
                    if num.value == i.value {
                        Arc::make_mut(&mut num).was_called();
                    }
                }
            }
            let val = check(board.clone());
            if val {
                // add index to pop list
                ilist.push(iboard);
            }
            iboard += 1;
        }
        if Arc::as_ref(&boards.clone()).len() == 1 {
            println!("boards {:?}", &boards.clone());
        }
        println!("ilist {:?} called {:?}", &ilist, &i);
        ilist.reverse();
        for j in ilist {
            if Arc::as_ref(&boards.clone()).len() == 1 {
                let boards_c = boards.clone();
                println!("boards_c {:?}", &boards_c);
                let boardi = boards_c[0].clone();
                return summation(boardi) * i.value;
            }
            Arc::make_mut(&mut boards).remove(j);
        }
        ilist = vec![];
    }
    return 0;
}

fn part1(input: &Vec<&str>) -> u32 {
    let mut input = input.to_vec();
    //println!("{:?}", input);
    let called: Vec<Arc<Number>> = input[0].split(',').collect::<Vec<&str>>().to_vec().into_iter().map(convert).collect();
    //println!("{:?}", guesses);
    input.remove(0); // remove guesses
    input.remove(0); // remove empty line
    let boards = parse_boards(input);
    //println!("\nboards: {:?}\n", boards);
    let _winning_board = bingo(called, boards);
    //println!("\nwinning board: {:?}\n", winning_board);
    return 0;
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut input = input.to_vec();
    //println!("{:?}", input);
    let called: Vec<Arc<Number>> = input[0].split(',').collect::<Vec<&str>>().to_vec().into_iter().map(convert).collect();
    //println!("{:?}", guesses);
    input.remove(0); // remove guesses
    input.remove(0); // remove empty line
    let boards = parse_boards(input);
    //println!("\nboards: {:?}\n", boards);
    let winning_board = bingo2(called, boards);
    println!("\nwinning board: {:?}\n", winning_board);
    return 0;
}
