use std::io;

fn main() {
    repl();
}

fn repl(){
    let mut last_guess = "hellz";
    let words = vec!["hello", "carts", "lisps", "phone"];
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let arr = buffer.replace("\n", "").chars().collect::<Vec<char>>();
    let candidates = words.iter().filter(|&&x| verify(x.to_string(),&arr, last_guess.to_string())).collect::<Vec<&&str>>();
    //println!("{:?}", candidates);
    last_guess = candidates[0];
    println!("{}", last_guess);
}

fn verify(target: String, hints: &Vec<char>, guess: String) -> bool {
    let mut res = true;
    for (i,c) in hints.iter().enumerate(){
        let cur = guess.chars().nth(i).unwrap();
        match c{
            '0' => {
                res &= notIn(&target,cur);//not in
            },
            '1' => {
                res &= inNotHere(&target,cur,i);//in not here
            },
            '2' => {
                res &= inHere(&target,cur,i);//in here
            }, 
            _ => {panic!("bad input");}
        }
    }
    res
}

#[allow(non_snake_case)]
fn notIn(target: &String, cur: char) -> bool{
    !target.contains(cur)
}

#[allow(non_snake_case)]
fn inNotHere(target: &String, cur: char, index: usize) -> bool{
    target.contains(cur) && (target.chars().nth(index).unwrap() != cur)
}

#[allow(non_snake_case)]
fn inHere(target: &String, cur: char, index: usize) -> bool{
    target.chars().nth(index).unwrap() == cur
}

/* p a s t e   // hasNotIn('p',0) && notHave('a') && hasIn('s',2) && hasNotIn('t',3) && notHave('e')
*  1 0 2 1 0
*
*
*/