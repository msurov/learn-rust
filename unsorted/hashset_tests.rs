use std::collections::HashSet;

#[allow(unused)]
fn employees_test() {
  let allowed_employees: HashSet<i32> = HashSet::from([123, 456, 789, 101, 202]);
  let mut employee_id = String::new();
  std::io::stdin().read_line(&mut employee_id).expect("Can't read imployee's ID");
  let employee_id : i32 = employee_id.trim().parse().expect("Can't parse imployee's ID");
  
  if let Some(_) = allowed_employees.get(&employee_id) {
    println!("Доступ разрешен");
  } else {
    println!("Доступ запрещен");
  }
}

fn main() {
  let mut team: HashSet<String> = HashSet::new();

  let mut num_inputs = String::new();
  std::io::stdin().read_line(&mut num_inputs).expect("Can't read number of nicknames");
  let num_inputs : u32 = num_inputs.trim().parse().expect("Can't parse number of nicknames");

  for _ in 0..num_inputs {
    let mut nickname = String::new();
    std::io::stdin().read_line(&mut nickname).expect("Can't read nickname");
    team.insert(
      nickname.trim().to_owned()
    );
  }

  println!("Количество: {:?}", team.len());
}
